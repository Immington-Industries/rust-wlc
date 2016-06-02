//! Contains definitions for wlc handle types.
//!
//! # Implementations
//! - **Debug**: pointer-prints the underlying `uintptr_t` handle
//! - **Eq, Ord**: compare the underlying `uintptr_t` handle
//! - **Clone**: View handles can safely be cloned.

extern crate libc;
use libc::{uintptr_t, c_char, c_void};

use super::pointer_to_string;
use super::types::{Geometry, ResizeEdge, Point, Size, ViewType, ViewState};

use std::hash::Hash;
use std::fmt::Debug;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a handle to a wlc view.
///
pub struct WlcView(libc::uintptr_t);

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
/// Represents a handle to a wlc output.
pub struct WlcOutput(libc::uintptr_t);

#[link(name = "wlc")]
extern "C" {
    fn wlc_get_outputs(memb: *mut libc::size_t) -> *const libc::uintptr_t;

    fn wlc_get_focused_output() -> uintptr_t;

    fn wlc_output_get_name(output: uintptr_t) -> *const c_char;

    fn wlc_handle_get_user_data(handle: uintptr_t) -> *mut c_void;

    // Defined in wlc-render.h
    fn wlc_output_schedule_render(output: uintptr_t);

    fn wlc_handle_set_user_data(handle: uintptr_t, userdata: *const c_void);

    fn wlc_output_get_sleep(output: uintptr_t) -> bool;

    fn wlc_output_set_sleep(output: uintptr_t, sleep: bool);

    fn wlc_output_get_resolution(output: uintptr_t) -> *const Size;

    fn wlc_output_set_resolution(output: uintptr_t, resolution: *const Size);

    fn wlc_output_get_mask(output: uintptr_t) -> u32;

    fn wlc_output_set_mask(output: uintptr_t, mask: u32);

    // TODO tricky definition here
    //fn wlc_output_get_pixels(output: WlcHandle) -> ();

    fn wlc_output_get_views(output: uintptr_t,
                            out_memb: *mut libc::size_t) -> *const uintptr_t;

    fn wlc_output_set_views(output: uintptr_t, views: *const uintptr_t, memb: libc::size_t) -> bool;

    fn wlc_output_focus(output: uintptr_t);

    // View API

    fn wlc_view_focus(view: uintptr_t);

    fn wlc_view_close(view: uintptr_t);

    // View -> Output
    fn wlc_view_get_output(view: uintptr_t) -> uintptr_t;

    // "set output. Alternatively you can use wlc_output_set_views"
    fn wlc_view_set_output(view: uintptr_t, output: uintptr_t);

    fn wlc_view_send_to_back(view: uintptr_t);

    fn wlc_view_send_below(view: uintptr_t, other: uintptr_t);

    fn wlc_view_bring_above(view: uintptr_t, other: uintptr_t);

    fn wlc_view_bring_to_front(view: uintptr_t);

    fn wlc_view_get_mask(view: uintptr_t) -> u32;

    fn wlc_view_set_mask(view: uintptr_t, mask: u32);

    fn wlc_view_get_geometry(view: uintptr_t) -> *const Geometry;

    fn wlc_view_get_visible_geometry(view: uintptr_t, geo: *mut Geometry);

    fn wlc_view_set_geometry(view: uintptr_t, edges: u32, geo: *const Geometry);

    fn wlc_view_get_type(view: uintptr_t) -> ViewType;

    fn wlc_view_set_type(view: uintptr_t, view_type: ViewType, toggle: bool);

    fn wlc_view_get_state(view: uintptr_t) -> ViewState;

    fn wlc_view_set_state(view: uintptr_t, state: ViewState, toggle: bool);

    // Parent is Option<View>
    fn wlc_view_get_parent(view: uintptr_t) -> uintptr_t;

    // Parent is Option<View>
    fn wlc_view_set_parent(view: uintptr_t, parent: uintptr_t);

    fn wlc_view_get_title(view: uintptr_t) -> *const c_char;

    fn wlc_view_get_class(view: uintptr_t) -> *const c_char;

    fn wlc_view_get_app_id(view: uintptr_t) -> *const c_char;
}

impl From<WlcView> for WlcOutput {
    fn from(view: WlcView) -> Self {
        WlcOutput(view.0)
    }
}

impl From<WlcOutput> for WlcView {
    fn from(output: WlcOutput) -> Self {
        WlcView(output.0)
    }
}

/// A trait defining the methods on a wlc output handle.
/// This trait is exposed to aid testing when wlc isn't running
pub trait Output: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash {

    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    fn as_view(self) -> WlcView;

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn get_user_data<T>(&self) -> &mut T;

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn set_user_data<T>(&self, data: &T);

    /// Schedules output for rendering next frame.
    ///
    /// If the output was already scheduled, this is
    /// a no-op; if output is currently rendering,
    /// it will render immediately after.
    fn schedule_render(&self);

    /// Gets the name of the WlcOutput.
    ///
    /// Names are usually assigned in the format WLC-n,
    /// where the first output is WLC-1.
    fn get_name(&self) -> String;

    /// Gets the sleep status of the output.
    ///
    /// Returns `true` if the monitor is sleeping,
    /// such as having been set with `set_sleep`.
    fn get_sleep(&self) -> bool;

    /// Sets the sleep status of the output.
    fn set_sleep(&self, sleep: bool);

    /// Gets the output resolution in pixels.
    fn get_resolution(&self) -> &Size;

    /// Sets the resolution of the output.
    ///
    /// # Safety
    /// This method will crash the program if use when wlc is not running.
    fn set_resolution(&self, size: Size);

    /// Get views in stack order.
    ///
    /// This is mainly useful for wm's who need another view stack for inplace sorting.
    /// For example tiling wms, may want to use this to keep their tiling order separated
    /// from floating order.
    /// This handles `wlc_output_get_views` and `wlc_output_get_mutable_views`.
    fn get_views(&self) -> Vec<WlcView>;

    /// Gets the mask of this output
    fn get_mask(&self) -> u32;

    /// Sets the mask for this output
    fn set_mask(&self, mask: u32);

    /// # Deprecated
    /// This function is equivalent to simply calling get_views
    fn get_mutable_views(&self) -> Vec<WlcView>;

    /// Attempts to set the views of a given output.
    ///
    /// Returns success if operation succeeded. An error will be returned
    /// if something went wrong or if wlc isn't running.
    fn set_views(&self, views: &mut Vec<&WlcView>) -> Result<(), &'static str>;
}

/// A trait defining the methods on a wlc view handle.
/// This trait is exposed to aid testing when wlc isn't running
pub trait View: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash  {

    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    fn as_output(self) -> WlcOutput;

    /// Whether this view is the root window (desktop background).
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::{WlcView, View};
    /// # // This example can be run because WlcView::root() does not interact with wlc
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    #[inline]
    fn is_root(&self) -> bool;

    /// Whether this view is not the root window (desktop background).
    ///
    /// # Usage
    /// A convenience method, the opposite of `view.is_root()`.
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::{WlcView, View};
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// assert!(!view.is_window());
    /// ```
    #[inline]
    fn is_window(&self) -> bool;

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn get_user_data<T>(&self) -> &mut T;

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn set_user_data<T>(&self, data: &T);

    /// Closes this view.
    ///
    /// For the main windows of most programs, this should close the program where applicable.
    ///
    /// # Behavior
    /// This function will not do anything if `view.is_root()`.
    fn close(&self);

    /// Gets the WlcOutput this view is currently part of.
    fn get_output(&self) -> WlcOutput;

    /// Sets the output that the view renders on.
    ///
    /// This may not be supported by wlc at this time.
    fn set_output(&self, output: &WlcOutput);

    /// Brings this view to focus.
    ///
    /// Can be called on `WlcView::root()` to lose all focus.
    fn focus(&self);

    /// Sends the view to the back of the compositor
    fn send_to_back(&self);

    /// Sends this view underneath another.
    fn send_below(&self, other: &WlcView);

    /// Brings this view above another.
    fn bring_above(&self, other: &WlcView);

    /// Brings this view to the front of the stack
    /// within its WlcOutput.
    fn bring_to_front(&self);

    // TODO Get masks enum working properly
    /// Gets the current visibilty bitmask for the view.
    fn get_mask(&self) -> u32;

    // TODO Get masks enum working properly
    /// Sets the visibilty bitmask for the view.
    fn set_mask(&self, mask: u32);

    /// Gets the geometry of the view.
    fn get_geometry(&self) -> Option<&Geometry>;

    /// Gets the geometry of the view (that wlc displays).
    fn get_visible_geometry(&self) -> Geometry;

    /// Sets the geometry of the view.
    ///
    /// Set edges if geometry is caused by interactive resize.
    fn set_geometry(&self, edges: ResizeEdge, geometry: &Geometry);

    /// Gets the type bitfield of the curent view
    fn get_type(&self) -> ViewType;

    /// Set flag in the type field. Toggle indicates whether it is set.
    fn set_type(&self, view_type: ViewType, toggle: bool);

    // TODO get bitflags enums
    /// Get the current ViewState bitfield.
    fn get_state(&self) -> ViewState;

    /// Set ViewState bit. Toggle indicates whether it is set or not.
    fn set_state(&self, state: ViewState, toggle: bool);

    /// Gets parent view, returns `WlcView::root()` if this view has no parent.
    fn get_parent(&self) -> WlcView;

    /// Set the parent of this view.
    ///
    /// Call with `WlcView::root()` to make its parent the root window.
    fn set_parent(&self, parent: &WlcView);

    /// Get the title of the view
    fn get_title(&self) -> String;

    /// Get class (shell surface only).
    fn get_class(&self) -> String;

    /// Get app id (xdg-surface only).
    fn get_app_id(&self) -> String;
}

impl WlcOutput {

    /// Create a dummy WlcOutput for testing purposes.
    ///
    /// # Unsafety
    /// The following operations on a dummy WlcOutput will cause crashes:
    ///
    /// - `WlcOutput::focused` when wlc is not running
    /// - `WlcOutput::list` when wlc is not running
    /// - `WlcOutput::set_resolution` on a dummy output
    ///
    /// In addition, `WlcOutput::set_views` will return an error.
    ///
    /// All other methods can be used on dummy outputs.
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcOutput;
    /// let output = WlcOutput::dummy(0u32);
    /// let output2 = WlcOutput::dummy(1u32);
    /// assert!(output < output2);
    /// assert!(output != output2);
    /// ```
    pub fn dummy(code: u32) -> WlcOutput {
        WlcOutput(code as libc::uintptr_t)
    }

    /// Gets a list of the current outputs.
    ///
    /// # Safety
    /// This function will crash the program if run when wlc is not running.
    pub fn list() -> Vec<WlcOutput> {
        unsafe {
            let mut out_memb: libc::size_t = 0;
            let outputs = wlc_get_outputs(&mut out_memb);
            if outputs.is_null() {
                return Vec::new();
            }
            let mut result = Vec::with_capacity(out_memb);
            for index in (0 as isize) .. (out_memb as isize) {
                result.push(WlcOutput(*(outputs.offset(index))));
            }
            result
        }
    }

    /// Gets the currently focused output.
    ///
    /// # Safety
    /// This function will crash the program if run when wlc is not running.
    pub fn focused() -> WlcOutput {
        unsafe { WlcOutput(wlc_get_focused_output()) }
    }

    /// Focuses compositor on a specific output.
    ///
    /// Pass in Option::None for no focus.
    pub fn focus(output: Option<&WlcOutput>) {
        unsafe {
            match output {
                Some(output) => wlc_output_focus(output.0),
                None => wlc_output_focus(0)
            }
        }
    }

}

impl Output for WlcOutput {

    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    fn as_view(self) -> WlcView {
        return WlcView::from(self)
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn get_user_data<T>(&self) -> &mut T {
        let raw_data = wlc_handle_get_user_data(self.0);
        return &mut *(raw_data as *mut T);
    }

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn set_user_data<T>(&self, data: &T) {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        wlc_handle_set_user_data(self.0, data_ptr);
    }

    /// Schedules output for rendering next frame.
    ///
    /// If the output was already scheduled, this is
    /// a no-op; if output is currently rendering,
    /// it will render immediately after.
    fn schedule_render(&self) {
        unsafe { wlc_output_schedule_render(self.0) };
    }

    /// Gets the name of the WlcOutput.
    ///
    /// Names are usually assigned in the format WLC-n,
    /// where the first output is WLC-1.
    fn get_name(&self) -> String {
        let name: *const i8;
        unsafe {
            name = wlc_output_get_name(self.0);
            pointer_to_string(name)
        }
    }

    /// Gets the sleep status of the output.
    ///
    /// Returns `true` if the monitor is sleeping,
    /// such as having been set with `set_sleep`.
    fn get_sleep(&self) -> bool {
        unsafe { wlc_output_get_sleep(self.0) }
    }

    /// Sets the sleep status of the output.
    fn set_sleep(&self, sleep: bool) {
        unsafe { wlc_output_set_sleep(self.0, sleep); }
    }

    /// Gets the output resolution in pixels.
    fn get_resolution(&self) -> &Size {
        unsafe { &*wlc_output_get_resolution(self.0) }
    }

    /// Sets the resolution of the output.
    ///
    /// # Safety
    /// This method will crash the program if use when wlc is not running.
    fn set_resolution(&self, size: Size) {
        unsafe { wlc_output_set_resolution(self.0, &size); }
    }

    /// Get views in stack order.
    ///
    /// This is mainly useful for wm's who need another view stack for inplace sorting.
    /// For example tiling wms, may want to use this to keep their tiling order separated
    /// from floating order.
    /// This handles `wlc_output_get_views` and `wlc_output_get_mutable_views`.
    fn get_views(&self) -> Vec<WlcView> {
        unsafe {
            let mut out_memb: libc::size_t = 0;
            let views = wlc_output_get_views(self.0, &mut out_memb);
            if views.is_null() {
                return Vec::new();
            }
            let mut result = Vec::with_capacity(out_memb);

            for index in (0 as isize) .. (out_memb as isize) {
                  result.push(WlcView(*(views.offset(index))));
            }
            return result;
        }
    }

    /// Gets the mask of this output
    fn get_mask(&self) -> u32 {
        unsafe { wlc_output_get_mask(self.0) }
    }

    /// Sets the mask for this output
    fn set_mask(&self, mask: u32) {
        unsafe { wlc_output_set_mask(self.0, mask) }
    }

    /// # Deprecated
    /// This function is equivalent to simply calling get_views
    fn get_mutable_views(&self) -> Vec<WlcView> {
        self.get_views()
    }

    /// Attempts to set the views of a given output.
    ///
    /// Returns success if operation succeeded. An error will be returned
    /// if something went wrong or if wlc isn't running.
    fn set_views(&self, views: &mut Vec<&WlcView>) -> Result<(), &'static str> {
            let view_len = views.len() as libc::size_t;
            let view_vals: Vec<uintptr_t> = views.into_iter().map(|v| v.0).collect();
            let const_views = view_vals.as_ptr();
        unsafe {
            match wlc_output_set_views(self.0, const_views, view_len) {
                true => Ok(()),
                false => Err("Could not set views on output"),
            }
        }
    }
}

impl WlcView {
    /// Create a dummy WlcView for testing purposes.
    ///
    /// # Unsafety
    /// The following methods on views may crash the program:
    ///
    /// - `WlcView::focus` if wlc is not running
    /// - `WlcView::send_to_back` if wlc is not running
    /// - `WlcView::send_below` if wlc is not running
    /// - `WlcView::bring_above` if wlc is not running
    /// - `WlcView::bring_to_font` if wlc is not running
    ///
    /// All other methods can be used on dummy views.
    ///
    /// # Note
    /// `WlcView::root()` is equivalent to `WlcView::dummy(0)`.
    ///
    /// ```rust
    /// # use rustwlc::WlcView;
    /// assert!(WlcView::root() == WlcView::dummy(0))
    /// ```
    /// # Example
    /// ```rust
    /// # use rustwlc::WlcView;
    /// let view = WlcView::dummy(0u32);
    /// let view2 = WlcView::dummy(1u32);
    /// assert!(view < view2);
    /// assert!(view != view2);
    /// ```

    pub fn dummy(code: u32) -> WlcView {
        WlcView(code as uintptr_t)
    }

    /// Returns a reference to the root window (desktop background).
    ///
    /// # Example
    /// ```
    /// # use rustwlc::{WlcView, View};
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    pub fn root() -> WlcView {
        WlcView(0)
    }

}

impl View for WlcView {

    /// Compatability/debugging function.
    ///
    /// wlc internally stores views and outputs under the same type.
    /// If for some reason a conversion between the two was required,
    /// this function could be called. If this is the case please submit
    /// a bug report.
    fn as_output(self) -> WlcOutput {
        WlcOutput::from(self)
    }

    /// Whether this view is the root window (desktop background).
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::{WlcView, View};
    /// # // This example can be run because WlcView::root() does not interact with wlc
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// ```
    #[inline]
    fn is_root(&self) -> bool {
        self.0 == 0
    }

    /// Whether this view is not the root window (desktop background).
    ///
    /// # Usage
    /// A convenience method, the opposite of `view.is_root()`.
    ///
    /// # Example
    /// ```rust
    /// # use rustwlc::{WlcView, View};
    /// let view = WlcView::root();
    /// assert!(view.is_root());
    /// assert!(!view.is_window());
    /// ```
    #[inline]
    fn is_window(&self) -> bool {
        self.0 != 0
    }

    /// Gets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn get_user_data<T>(&self) -> &mut T {
        let raw_data = wlc_handle_get_user_data(self.0);
        return &mut *(raw_data as *mut T);
    }

    /// Sets user-specified data.
    ///
    /// # Unsafety
    /// The wlc implementation of this method uses `void*` pointers
    /// for raw C data. This function will internaly do a conversion
    /// between the input `T` and a `libc::c_void`.
    ///
    /// This is a highly unsafe conversion with no guarantees. As
    /// such, usage of these functions requires an understanding of
    /// what data they will have. Please review wlc's usage of these
    /// functions before attempting to use them yourself.
    unsafe fn set_user_data<T>(&self, data: &T) {
        let data_ptr: *const c_void = data as *const _ as *const c_void;
        wlc_handle_set_user_data(self.0, data_ptr);
    }

    /// Closes this view.
    ///
    /// For the main windows of most programs, this should close the program where applicable.
    ///
    /// # Behavior
    /// This function will not do anything if `view.is_root()`.
    fn close(&self) {
        if self.is_root() { return };
        unsafe { wlc_view_close(self.0); }
    }

    /// Gets the WlcOutput this view is currently part of.
    fn get_output(&self) -> WlcOutput {
        unsafe { WlcOutput(wlc_view_get_output(self.0)) }
    }

    /// Sets the output that the view renders on.
    ///
    /// This may not be supported by wlc at this time.
    fn set_output(&self, output: &WlcOutput) {
        unsafe { wlc_view_set_output(self.0, output.0) }
    }

    /// Brings this view to focus.
    ///
    /// Can be called on `WlcView::root()` to lose all focus.
    fn focus(&self) {
        unsafe { wlc_view_focus(self.0); }
    }

    /// Sends the view to the back of the compositor
    fn send_to_back(&self) {
        unsafe { wlc_view_send_to_back(self.0); }
    }

    /// Sends this view underneath another.
    fn send_below(&self, other: &WlcView) {
        unsafe { wlc_view_send_below(self.0, other.0); }
    }

    /// Brings this view above another.
    fn bring_above(&self, other: &WlcView) {
        unsafe { wlc_view_bring_above(self.0, other.0); }
    }

    /// Brings this view to the front of the stack
    /// within its WlcOutput.
    fn bring_to_front(&self) {
        unsafe { wlc_view_bring_to_front(self.0); }
    }

    // TODO Get masks enum working properly
    /// Gets the current visibilty bitmask for the view.
    fn get_mask(&self) -> u32 {
        unsafe { wlc_view_get_mask(self.0) }
    }

    // TODO Get masks enum working properly
    /// Sets the visibilty bitmask for the view.
    fn set_mask(&self, mask: u32) {
        unsafe { wlc_view_set_mask(self.0, mask); }
    }

    /// Gets the geometry of the view.
    fn get_geometry(&self) -> Option<&Geometry> {
        unsafe {
            let geometry = wlc_view_get_geometry(self.0);
            if geometry.is_null() {
                None
            } else {
                Some(&*geometry)
            }
        }
    }

    /// Gets the geometry of the view (that wlc displays).
    fn get_visible_geometry(&self) -> Geometry {
        let mut geo = Geometry { origin: Point { x: 0, y: 0}, size: Size { w: 0, h: 0 }};
        unsafe {
            wlc_view_get_visible_geometry(self.0, &mut geo);
        }
        return geo;
    }

    /// Sets the geometry of the view.
    ///
    /// Set edges if geometry is caused by interactive resize.
    fn set_geometry(&self, edges: ResizeEdge, geometry: &Geometry) {
        unsafe { wlc_view_set_geometry(self.0, edges.bits(), geometry as *const Geometry); }
    }

    /// Gets the type bitfield of the curent view
    fn get_type(&self) -> ViewType {
        unsafe { wlc_view_get_type(self.0) }
    }

    /// Set flag in the type field. Toggle indicates whether it is set.
    fn set_type(&self, view_type: ViewType, toggle: bool) {
        unsafe { wlc_view_set_type(self.0, view_type, toggle); }
    }

    // TODO get bitflags enums
    /// Get the current ViewState bitfield.
    fn get_state(&self) -> ViewState {
        unsafe { wlc_view_get_state(self.0) }
    }

    /// Set ViewState bit. Toggle indicates whether it is set or not.
    fn set_state(&self, state: ViewState, toggle: bool) {
        unsafe { wlc_view_set_state(self.0, state, toggle); }
    }

    /// Gets parent view, returns `WlcView::root()` if this view has no parent.
    fn get_parent(&self) -> WlcView {
        unsafe { WlcView(wlc_view_get_parent(self.0)) }
    }

    /// Set the parent of this view.
    ///
    /// Call with `WlcView::root()` to make its parent the root window.
    fn set_parent(&self, parent: &WlcView) {
        unsafe { wlc_view_set_parent(self.0, parent.0); }
    }

    /// Get the title of the view
    fn get_title(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_title(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                    pointer_to_string(chars)
            }
        }
    }

    /// Get class (shell surface only).
    fn get_class(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_class(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                pointer_to_string(chars)
            }
        }
    }

    /// Get app id (xdg-surface only).
    fn get_app_id(&self) -> String {
        let chars: *const i8;
        unsafe {
            chars = wlc_view_get_app_id(self.0);
            if chars == 0 as *const i8 {
                String::new()
            } else {
                pointer_to_string(chars)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn dummy_views() {
        let dummy = WlcView::dummy(1);
        assert!(!dummy.is_root(), "Dummy(1) is root");
        assert!(dummy.is_window(), "Dummy(1) is root");
        let _title = dummy.get_title();
        let _class = dummy.get_class();
        let _app_id = dummy.get_app_id();
        // Let's do some stuff with views
        dummy.close(); // works
        let output = dummy.get_output();
        assert!(output == WlcOutput::dummy(0));
        dummy.set_output(&output);
        // dummy.focus(); // SEGFAULTS
        // dummy.send_to_back();
        // dummy.send_below(&dummy);
        // dummy.bring_above(&dummy);
        // dummy.bring_to_front();
        let mask = dummy.get_mask();
        dummy.set_mask(mask);
        let geometry = dummy.get_geometry();
        assert!(geometry.is_none(), "Got geometry from dummy");
        dummy.set_geometry(EDGE_NONE, &Geometry {
            origin: Point { x: 0, y: 0 },
            size: Size { w: 0, h: 0 }
        });
        let view_type = dummy.get_type();
        assert!(view_type.is_empty(), "Dummy had a view type");
        dummy.set_type(ViewType::empty(), true);
        let view_state = dummy.get_state();
        assert!(view_state.is_empty(), "Dummu had a view state");
        dummy.set_state(view_state, true);
        let parent = dummy.get_parent();
        assert!(parent.is_root(), "Dummy had real parent");
        dummy.set_parent(&parent);
    }

    #[test]
    fn dummy_outputs() {
        let dummy = WlcOutput::dummy(1);
        //let _current = WlcOutput::focused();
        //let _outputs = WlcOutput::list();
        //dummy.set_resolution(resolution.clone());
        dummy.schedule_render();
        let _name = dummy.get_name();
        let sleep = dummy.get_sleep();
        dummy.set_sleep(sleep);
        let _resolution = dummy.get_resolution();
        let views = dummy.get_views();
        dummy.set_views(&mut views.iter().collect()).unwrap_err();
        let mask = dummy.get_mask();
        dummy.set_mask(mask);
        WlcOutput::focus(Some(&dummy));
    }
}
