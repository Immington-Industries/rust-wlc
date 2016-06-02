//! `rustwlc` is a wrapper for [wlc][], a library for writing a window
//! manager using the [wayland][] protocol. Compositors using rustwlc will
//! not need unsafe code for basic interaction with wayland.
//!
//! # wlc
//!
//! [wlc][] is a library written in C which acts as a wayland compositor. It
//! provides abstractions over wayland via structs such as `WlcView`,
//! `WlcOutput`, `Geometry`, or `KeyboardModifiers`. It provides callbacks
//! (found in the `callback` module) for events such as `view_created`,
//! `mouse_move`, or `view_focused`.
//!
//! # Example
//!
//! For more information on how to use rustwlc, see the `callbacks` module
//! and the `run_wlc()` method.
//!
//! For a more complete example, see [the example program][] on
//! [our GitHub page][].
//!
//! ```rust
//! extern crate rustwlc;
//! use rustwlc::callback;
//! // VIEW_ACTIVATED is a bitflags enum variant, and those must be imported
//! // manually, or using a wildcard.
//! use rustwlc::{WlcView, View, VIEW_ACTIVATED};
//!
//! // Callbacks must be labeled extern as they will be called from C
//! extern "C" fn view_created(view: WlcView) -> bool {
//!     view.bring_to_front();
//!     view.focus();
//!     return true;
//! }
//!
//! extern "C" fn view_focus(view: WlcView, focused: bool) {
//!     view.set_state(VIEW_ACTIVATED, focused);
//! }
//!
//! // Entry point for a compositor
//! fn compsoitor_main() {
//!     callback::view_created(view_created);
//!     callback::view_focus(view_focus);
//!
//!     // The default log handler will print wlc logs to stdout
//!     rustwlc::log_set_default_handler();
//!     let run_fn = rustwlc::init().expect("Unable to initialize!");
//!     // This will run wlc's event loop and launch wayland.
//!     run_fn();
//! }
//! # fn main() {}
//! ```
//! For a more full-featured compositor using rustwlc, see [way-cooler][].
//! [wlc]: https://github.com/Cloudef/wlc
//! [wayland]: https://wayland.freedesktop.org/
//! [the example program]: https://github.com/Immington-Industries/rust-wlc/blob/master/example/src/main.rs
//! [our GitHub page]: https://github.com/Immington-Industries/rustwlc
//! [way-cooler]: https://github.com/Immington-Industries/way-cooler

#![warn(missing_docs)]

extern crate libc;

#[macro_use]
extern crate bitflags;

use std::ffi;

pub mod handle;
pub mod callback;
pub mod types;
pub mod input;
pub mod wayland;
pub mod xkb;

pub use types::*;
pub use handle::{WlcOutput, Output, WlcView, View};

// Log Handler hack
static mut rust_logging_fn: fn(_type: LogType, string: &str) = default_log_callback;

// External WLC functions
#[link(name = "wlc")]
extern "C" {
    // init2 -> init :(
    fn wlc_init() -> bool;

    fn wlc_run();

    fn wlc_terminate();

    fn wlc_log_set_handler(callback: extern "C" fn(log_type: LogType, text: *const libc::c_char));

    fn wlc_get_backend_type() -> BackendType;
}

/// Query backend wlc is using.
///
/// # Results
/// * None: Unknown backend type
/// * DRM: "Direct Rendering Manager" - running on tty
/// * X11: Running inside an X server
pub fn get_backend_type() -> BackendType {
    unsafe { wlc_get_backend_type() }
}

/// Initialize wlc's callbacks and logger with a `WlcInterface`.
///
/// # Deprecated
/// wlc has deprecated this callback interface. They offer a new API with a
/// series of methods found in the `callback` module
///
/// To initialize wlc, register your callbacks with the functions described in
/// the `callbacks` module, and the logger using `log_set_handler` or
/// `log_set_default_handler`. Then call `init2()`.
///
/// # Permissions
/// If a compositor is initialized from the tty using suid or logind, it will
/// drop extra permissions after a call to `init()` or `init2()`. It is strongly
/// recommended to delay code which is not registering callbacks until after
/// this call.
///
/// # wlc Example
/// ```no_run
/// use rustwlc;
/// use rustwlc::callback;
/// use rustwlc::WlcView;
///
/// // An example callback function
/// // See the various functions in the callback module for more information
/// extern "C" fn view_focus_callback(view: WlcView, focused: bool) {
///     println!("A view came into focus!");
/// }
///
/// // Set a default log callback
/// rustwlc::log_set_default_handler();
///
/// // Register some callbacks
/// callback::view_focus(view_focus_callback);
/// // ... and additional callbacks
///
/// // The only thing your code should do before init2 is register callbacks
/// // and log handlers.
/// let run_wlc = rustwlc::init()
///     .expect("Unable to initialize wlc!");
///
/// run_wlc();
/// ```
pub fn init() -> Option<fn() -> ()> {
    if unsafe { wlc_init() } {
        Some(run_wlc)
    }
    else {
        None
    }
}

/// Deprecated alias to init().
///
/// When wlc went to 0.0.1, they added an argumentless init2
/// to replace the old init that took a WlcInterface. Now,
/// init2 has been renamed init and init is removed.
pub fn init2() -> Option<fn() -> ()> {
    init()
}

/// Runs wlc's event loop.
///
/// The initialize functions will return this function in an Option.
/// Only then can it be called to being wlc's main event loop.
fn run_wlc() {
    unsafe {
        wlc_run();
    }
}

/// Halts execution of wlc.
pub fn terminate() {
    unsafe {
        wlc_terminate();
    }
}

/// Registers a C callback for wlc logging.
///
/// Note that `rustwlc::log_set_default_handler()` will register a simple callback
/// that will print the type and text to the console.
///
/// # Parameters
/// The `handler` callback has two parameters:
/// * `type`: The `LogType` of the message being printed.
/// * `text`: The text to be logged, currently in C form. One may call `rustwlc::pointer_to_string`
/// to convert it to a Rust String.
///
/// # Safety
/// The callback function (like other callbacks in `rustwlc`) must be marked as extern as it is called
/// from C code.
///
/// In addition, `unsafe` will be required to convert the text into a Rust String.
pub fn log_set_handler(handler: extern "C" fn(type_: LogType, text: *const libc::c_char)) {
    unsafe {
        wlc_log_set_handler(handler);
    }
}

/// Registers a Rust callback for wlc logging.

/// This is a nice convenience function that should be used in place of
/// `log_set_handler`. That way you can just pass a safe Rust `&str`
/// and not depend on libc`.
pub fn log_set_rust_handler(handler: fn(type_: LogType, text: &str)) {
        // Set global handler function
        unsafe {
            rust_logging_fn = handler;
            extern "C" fn c_handler(type_: LogType, text: *const libc::c_char) {
                unsafe {
                    let string = ffi::CStr::from_ptr(text).to_string_lossy().into_owned();
                    rust_logging_fn(type_, &string);
                }
            }
            wlc_log_set_handler(c_handler);
        }
}

fn default_log_callback(log_type: LogType, text: &str) {
    println!("wlc [{:?}] {}", log_type, text);
}

/// Sets the wlc log callback to a simple function that prints to console.
///
/// Not calling any `log_set_handler` will have no logging, use this or
/// `log_set_handler` with a callback to use wlc logging.
///
/// # Example
/// ```no_run
/// use rustwlc;
///
/// // An example where only the default log handler is registered
/// rustwlc::log_set_default_handler();
///
/// if let Some(run_wlc) = rustwlc::init2() {
///      run_wlc();
/// }
/// else {
///     panic!("Unable to initialize wlc!");
/// }
/// ```
pub fn log_set_default_handler() {
    log_set_rust_handler(default_log_callback);
}

/// Unsafe strings conversion function.
///
/// Converts a `*const libc::c_char` to an owned `String`.
/// Useful for log callbacks.
///
/// # Example
/// Standard usage may be for the log callbacks.
/// ```rust
/// extern "C" fn default_log_callback(log_type: LogType, text: *const libc::c_char) {
///     let string = unsafe { pointer_to_string(text) };
///     println!("wlc [{:?}]: {}", log_type, string);
/// }
/// ```
pub unsafe fn pointer_to_string(pointer: *const libc::c_char) -> String {
    if pointer.is_null() {
        return "".to_string();
    }
    let slice = ffi::CStr::from_ptr(pointer);
    slice.to_string_lossy().into_owned()
}
