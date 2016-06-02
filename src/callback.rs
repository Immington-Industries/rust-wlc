//! Register wlc callbacks to events.
//!
//! See individual methods for callback details.
//!
//! # wlc Example
//! ```no_run
//! use rustwlc;
//! use rustwlc::callback;
//! use rustwlc::WlcView;
//!
//! // An example callback function
//! // See the various functions in this module for more information
//! extern "C" fn view_focus_callback(view: WlcView, focused: bool) {
//!     println!("A view came into focus!");
//! }
//!
//! // Set a default log callback
//! rustwlc::log_set_default_handler();
//!
//! // Register some callbacks
//! callback::view_focus(view_focus_callback);
//! // ... and additional callbacks
//!
//! // The only thing your code should do before init2 is register callbacks
//! // and log handlers.
//! let run_wlc = rustwlc::init2()
//!     .expect("Unable to initialize wlc!");
//!
//! run_wlc();
//! ```

use super::types::*;
use super::handle::{WlcOutput, WlcView};

#[link(name = "wlc")]
extern "C" {
    // Output was created. Return false if you want to destroy the output.
    // (e.g. failed to allocate data related to view)
    fn wlc_set_output_created_cb(cb: extern "C" fn(WlcOutput) -> bool);

    // Output was destroyed.
    fn wlc_set_output_destroyed_cb(cb: extern "C" fn(WlcOutput));

    // Output got or lost focus.
    fn wlc_set_output_focus_cb(cb: extern "C" fn(WlcOutput, bool));

    // Output resolution changed.
    fn wlc_set_output_resolution_cb(cb: extern "C" fn(WlcOutput, &Size, &Size));

    // Output context callbacks
    fn wlc_set_output_context_created_cb(cb: extern "C" fn(WlcOutput));

    fn wlc_set_output_context_destroyed_cb(cb: extern "C" fn(WlcOutput));

    // Output pre render hook.
    fn wlc_set_output_render_pre_cb(cb: extern "C" fn(WlcOutput));

    // Output post render hook.
    fn wlc_set_output_render_post_cb(cb: extern "C" fn(WlcOutput));

    // View was created. Return false if you want to destroy the view.
    // (e.g. failed to allocate data related to view)
    fn wlc_set_view_created_cb(cb: extern "C" fn(WlcView) -> bool);

    // View was destroyed.
    fn wlc_set_view_destroyed_cb(cb: extern "C" fn(handle: WlcView));

    // View got or lost focus.
    fn wlc_set_view_focus_cb(cb: extern "C" fn(WlcView, bool));

    // View was moved to output.
    fn wlc_set_view_move_to_output_cb(cb: extern "C" fn(WlcView, WlcOutput,
                                                    WlcOutput));

    // Request to set given geometry for view.
    // Apply using wlc_view_set_geometry to agree.
    fn wlc_set_view_request_geometry_cb(cb: extern "C" fn(WlcView,
                                                          &Geometry));

    // Request to disable or enable the given state for view.
    // Apply using wlc_view_set_state to agree.
    fn wlc_set_view_request_state_cb(cb: extern "C" fn(WlcView,
                                                       ViewState, bool));

    // Request to move itself. Start a interactive move to agree.
    fn wlc_set_view_request_move_cb(cb: extern "C" fn(WlcView, &Point));

    // Request to resize itself with the given edges.
    // Start a interactive resize to agree.
    fn wlc_set_view_request_resize_cb(cb: extern "C" fn(WlcView, ResizeEdge,
                                                    &Point));

    // View pre render hook.
    fn wlc_set_view_render_pre_cb(cb: extern "C" fn(WlcView));

    // View post render hook.
    fn wlc_set_view_render_post_cb(cb: extern "C" fn(WlcView));

    // Key event was triggered, view handle will be zero if there was no focus.
    // Return true to prevent sending the event to clients.
    fn wlc_set_keyboard_key_cb(cb: extern "C" fn(WlcView, u32,
                                                 &KeyboardModifiers,
                                                 u32, KeyState) -> bool);

    // Button event was triggered, view handle will be zero if there
    // was no focus. Return true to prevent sending the event to clients.
    fn wlc_set_pointer_button_cb(cb: extern "C" fn(WlcView, u32,
                                                   &KeyboardModifiers, u32,
                                                  ButtonState, &Point) -> bool);

    // Scroll event was triggered, view handle will be zero if there was no
    // focus. Return true to prevent sending the event to clients.
    fn wlc_set_pointer_scroll_cb(cb: extern "C" fn(WlcView, u32,
                                                   &KeyboardModifiers,
                                                 ScrollAxis, [f64; 2]) -> bool);

    // Motion event was triggered, view handle will be zero if there was no
    // focus. Apply with wlc_pointer_set_position to agree. Return true to
    // prevent sending the event to clients.
    fn wlc_set_pointer_motion_cb(cb: extern "C" fn(WlcView, u32,
                                                   &Point) -> bool);

    // Touch event was triggered, view handle will be zero if there was no
    // focus. Return true to prevent sending the event to clients.
    fn wlc_set_touch_cb(cb: extern "C" fn(WlcView, u32, &KeyboardModifiers,
                                    TouchType, i32, &Point) -> bool);

    // Compositor is ready to accept clients.
    fn wlc_set_compositor_ready_cb(cb: extern "C" fn());

    // Compositor is about to terminate.
    fn wlc_set_compositor_terminate_cb(cb: extern "C" fn());

    // We're not supporting libinput experimental callbacks

    // Input device was created. Return value does nothing. (Experimental)
    //fn wlc_set_input_created_cb(cb: extern "C" fn(&LibinputDevice) -> bool);

    // Input device was destroyed. (Experimental)
    //fn wlc_set_input_destroyed_cb(cb: extern "C" fn(&LibinputDevice));
}

/// Callback invoked when an output is created.
/// Return `true` to allow the output to exist.
///
/// # Example
/// ```rust
/// use rustwlc::{WlcOutput, Output};
///
/// extern fn on_output_created(output: WlcOutput) -> bool {
///     println!("Output {} ({:?}) was created", output.get_name(), output);
///     return true;
/// }
/// # fn main() { }
/// ```
pub fn output_created(callback: extern "C" fn(output: WlcOutput) -> bool) {
    unsafe {
        wlc_set_output_created_cb(callback);
    }
}

/// Callback invoked when an output is destroyed.
///
/// # Example
/// ```rust
/// use rustwlc::WlcOutput;
///
/// extern fn output_destroyed(output: WlcOutput) {
///     println!("Goodbye, {:?}", output);
/// }
/// # fn main() { }
/// ```
pub fn output_destroyed(callback: extern "C" fn(output: WlcOutput)) {
    unsafe {
        wlc_set_output_destroyed_cb(callback);
    }
}

/// Callback invoked when an output gains focus.
///
/// # Example
/// ```rust
/// use rustwlc::{WlcOutput, Output};
///
/// extern fn output_focus(output: WlcOutput, focused: bool) {
///     println!("Output {} {} focus", output.get_name(),
///              if focused { "gained" } else { "lost" });
/// }
/// # fn main() { }
/// ```
pub fn output_focus(callback: extern "C" fn(output: WlcOutput, focused: bool)) {
    unsafe {
        wlc_set_output_focus_cb(callback);
    }
}

/// Callback invoked when an output's resolution changes.
///
/// # Example
/// ```rust
/// use rustwlc::{WlcOutput, Output};
/// use rustwlc::Size;
///
/// extern fn output_resolution(output: WlcOutput,
///                             old_size: &Size, new_size: &Size) {
///     println!("Output {} went from {} to {}",
///              output.get_name(), old_size, new_size);
/// }
/// # fn main() { }
/// ```
pub fn output_resolution(callback: extern "C" fn(output: WlcOutput,
                                                 old_size: &Size,
                                                 new_size: &Size)) {
    unsafe {
        wlc_set_output_resolution_cb(callback);
    }
}

/// Output context created. This generally happens on a tty switch.
pub fn output_context_destroyed(cb: extern "C" fn(output: WlcOutput)) {
    unsafe {
        wlc_set_output_context_destroyed_cb(cb);
    }
}

/// Output context destroyed
pub fn output_context_created(cb: extern "C" fn(output: WlcOutput)) {
    unsafe {
        wlc_set_output_context_created_cb(cb);
    }
}

/// Callback invoked pre-render for an output.
pub fn output_render_pre(callback: extern "C" fn(output: WlcOutput)) {
    unsafe {
        wlc_set_output_render_pre_cb(callback);
    }
}

/// Callback invoked post-render for an output.
pub fn output_render_post(callback: extern "C" fn(output: WlcOutput)) {
    unsafe {
        wlc_set_output_render_post_cb(callback);
    }
}

/// Callback invoked when a view is created.
/// Return `true` to allow the view to be created.
///
/// When a new view is created, the following should probably be applied:
/// * Set the view's mask to the output's mask
/// * Focus the view
/// * Bring the view to the front
///
/// # Example
/// ```rust
/// use rustwlc::{WlcView, View, Output};
///
/// extern fn view_created(view: WlcView) -> bool {
///     println!("View \"{}\" was created ({:?})", view.get_class(), view);
///     view.set_mask(view.get_output().get_mask());
///     view.bring_to_front();
///     view.focus();
///     return true;
/// }
/// # fn main() { }
/// ```
pub fn view_created(callback: extern "C" fn(view: WlcView) -> bool) {
    unsafe {
        wlc_set_view_created_cb(callback);
    }
}

/// Callback invoked when a view is destroyed.
///
/// When a view is destroyed, it's a good idea to shift focus to
/// some other view, i.e. the last one used.
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
///
/// extern fn view_destroyed(view: WlcView) {
///     println!("Goodbye, {:?}", view);
/// }
/// # fn main() { }
/// ```
pub fn view_destroyed(callback: extern "C" fn(view: WlcView)) {
    unsafe {
        wlc_set_view_destroyed_cb(callback);
    }
}

/// Callback invoked when a view is focused.
///
/// The view's `ViewState::VIEW_ACTIVATED` bit should be set to true here.
///
/// # Example
/// ```rust
/// use rustwlc::{WlcView, View};
/// // The bitflags constants need to be imported manually.
/// use rustwlc::VIEW_ACTIVATED;
///
/// extern fn view_focus(view: WlcView, focused: bool) {
///     println!("View {:?} is {} focus, updating...",
///               view, if focused { "in" } else { "out of" });
///     view.set_state(VIEW_ACTIVATED, focused);
/// }
/// ```
pub fn view_focus(callback: extern "C" fn(handle: WlcView, focused: bool)) {
    unsafe {
        wlc_set_view_focus_cb(callback);
    }
}

/// Callback invoked when a view switches outputs.
///
/// Moving views between outputs is unsupported in wlc at the time of writing.
/// Wayland mandates each output have its own memory buffer so it may take wlc
/// some time before this is implemented.
pub fn view_move_to_output(callback: extern "C" fn(view: WlcView,
                                                   old_output: WlcOutput,
                                                   new_output: WlcOutput)) {
    unsafe {
        wlc_set_view_move_to_output_cb(callback);
    }
}

/// Callback invoked when a view requests geometry.
pub fn view_request_geometry(callback: extern "C" fn(handle: WlcView,
                                                     geometry: &Geometry)) {
    unsafe {
        wlc_set_view_request_geometry_cb(callback);
    }
}

/// Callback invoked when a view requests a `ViewState`.
pub fn view_request_state(callback: extern "C" fn(current: WlcView,
                                                  state: ViewState,
                                                  handled: bool)) {
    unsafe {
        wlc_set_view_request_state_cb(callback);
    }
}

/// Callback invoked when a view requests a move.
pub fn view_request_move(callback: extern "C" fn(handle: WlcView,
                                                 destination: &Point)) {
    unsafe {
        wlc_set_view_request_move_cb(callback);
    }
}

/// Callback invoked when a view requests a resize.
pub fn view_request_resize(callback: extern "C" fn(handle: WlcView,
                                                   edge: ResizeEdge,
                                                   location: &Point)) {
    unsafe {
        wlc_set_view_request_resize_cb(callback);
    }
}

/// Callback invoked pre-view-render.
pub fn view_render_pre(callback: extern "C" fn(view: WlcView)) {
    unsafe {
        wlc_set_view_render_pre_cb(callback);
    }
}

/// Callback invoked post-view-render.
pub fn view_render_post(callback: extern "C" fn(view: WlcView)) {
    unsafe {
        wlc_set_view_render_post_cb(callback);
    }
}

/// Callback invoked on keypresses.
/// Return `true` to block the press from the view.
///
/// # Arguments
/// The first `u32` is a timestamp, the second is the key code. The view may be
/// the root window.
///
/// Proper values for `key` can be found in `input.h` or a similar library/crate
/// - see wlc documentation on the subject, it may not support your keyboard
/// layout at the moment.
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
/// use rustwlc::{KeyboardModifiers, KeyState};
///
/// extern fn keyboard_key(view: WlcView, time: u32, mods: &KeyboardModifiers,
///                        key: u32, state: KeyState) -> bool {
///     println!("Key {} {:?} on {:?} at {} with modifiers {:?}",
///              key, view, state, time, mods);
///     return false;
/// }
/// # fn main() { }
/// ```
pub fn keyboard_key(callback: extern "C" fn(view: WlcView, time: u32,
                                            mods: &KeyboardModifiers, key: u32,
                                            state: KeyState) -> bool) {
    unsafe {
        wlc_set_keyboard_key_cb(callback);
    }
}

/// Callback invoked on mouse clicks.
/// Return `true` to block the click from the view.
///
/// # Arguments
/// The first u32 is a timestamp, the second is the button code.
/// The view may be the root window. Proper values for `button`
/// can be found in `input.h` or a similar library/crate.
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
/// use rustwlc::{KeyboardModifiers, ButtonState, Point};
///
/// extern fn pointer_button(view: WlcView, time: u32,
///                          mods: &KeyboardModifiers, button: u32,
///                          state: ButtonState, point: &Point) -> bool {
///     println!("Button {} {:?} at {} at {} in {:?}, keyboard mods: {:?}",
///              button, state, time, point, view, mods);
///     return false;
/// }
/// # fn main() { }
/// ```
pub fn pointer_button(callback: extern "C" fn(view: WlcView, time: u32,
                                              mods: &KeyboardModifiers,
                                              button: u32, state: ButtonState,
                                              point: &Point) -> bool) {
    unsafe {
        wlc_set_pointer_button_cb(callback);
    }
}

/// Callback invoked on mouse scroll.
/// Return `true` to block the scroll from the view.
///
/// # Arguments
/// * view: The WlcView (or output root) that was scrolled in
/// * time: Timestamp
/// * mods: Current pressed keyboard modifiers
/// * axis: Which direction the scroll was in
/// * amount: The first argument seems to be either 10 or -10 depending on
/// up/down (or right/left if `axis == ScrollAxis::Horizontal`).
/// The second one, when tested on a standard laptop trackpad, seems to be
/// a double slightly above zero.
pub fn pointer_scroll(callback: extern "C" fn(view: WlcView, time: u32,
                                              mods: &KeyboardModifiers,
                                              axis: ScrollAxis,
                                              amount: [f64; 2]) -> bool) {
    unsafe {
        wlc_set_pointer_scroll_cb(callback);
    }
}

/// Callback invoked on pointer motion.
/// Return `true` to block the motion from the view.
///
/// `rustwlc::input::pointer::set_position`
/// must be invoked to actually move the cursor!
///
/// # Example
/// ```rust
/// use rustwlc::WlcView;
/// use rustwlc::Point;
/// use rustwlc::input::pointer;
///
/// extern fn pointer_motion(view: WlcView, time: u32, point: &Point) -> bool {
///     println!("Pointer was moved to {} in {:?} at {}", point, view, time);
///     // This is very important.
///     pointer::set_position(point);
///     return false;
/// }
/// # fn main() { }
/// ```
pub fn pointer_motion(callback: extern "C" fn(view: WlcView, time: u32,
                                              point: &Point) -> bool) {
    unsafe {
        wlc_set_pointer_motion_cb(callback);
    }
}

/// Callback invoked on touchscreen touch.
/// Return `true` to block the touch from the view.
///
/// # Arguments
/// * `mods`: Which keyboard modifiers are being pressed during the event
/// * `touch`: What kind of event it is (a touch down, a frame being made,
/// a touch release). In the case of `TouchType::Frame`, `slot` and `point`
/// will both be zero.
/// * `slot`: Which finger - in cases of multiple touches down - is causing
/// the event
/// * `point`: Where the touch event happened
pub fn touch(callback: extern "C" fn(handle: WlcView, time: u32,
                                     mods: &KeyboardModifiers, touch: TouchType,
                                     slot: i32, point: &Point) -> bool) {
    unsafe {
        wlc_set_touch_cb(callback);
    }
}

/// Callback invoked by wlc after `rustwlc::init` is called.
pub fn compositor_ready(callback: extern "C" fn()) {
    unsafe {
        wlc_set_compositor_ready_cb(callback);
    }
}

/// Callback invoked by wlc when a compositor is terminating
pub fn compositor_terminate(callback: extern "C" fn()) {
    unsafe {
        wlc_set_compositor_terminate_cb(callback);
    }
}
