#![allow(unused)]

use super::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub(crate) enum WlSeatCapability {
    Pointer = 1,
    Keyboard = 2,
    Touch = 4,
}

#[repr(C)]
pub(crate) struct WlMessage {
    /** Message name */
    pub(crate) name: *const c_void,
    /** Message signature */
    pub(crate) signature: *const c_void,
    /** Object argument interfaces */
    pub(crate) wl_interface: *const *const WlInterface,
}

#[repr(C)]
pub(crate) struct WlInterface {
    /** Interface name */
    pub(crate) name: *const c_void,
    /** Interface version */
    pub(crate) version: i32,
    /** Number of methods (requests) */
    pub(crate) method_count: i32,
    /** Method (request) signatures */
    pub(crate) methods: *const WlMessage, // *wl_message
    /** Number of events */
    pub(crate) event_count: i32,
    /** Event signatures */
    pub(crate) events: *const WlMessage, // *wl_message
}

#[repr(C)]
pub(crate) struct WlCursor {
    pub(crate) image_count: u32,
    pub(crate) images: *mut *mut WlCursorImage,
    pub(crate) name: *mut c_void,
}

#[repr(C)]
pub(crate) struct WlCursorImage {
    pub(crate) width: u32,     /* actual width */
    pub(crate) height: u32,    /* actual height */
    pub(crate) hotspot_x: u32, /* hot spot x (must be inside image) */
    pub(crate) hotspot_y: u32, /* hot spot y (must be inside image) */
    pub(crate) delay: u32,     /* animation delay to next frame (ms) */
}

pub(crate) unsafe fn wl_surface_destroy(surface: *mut c_void) {
    extern "C" {
        fn wl_proxy_marshal(p: *mut c_void, opcode: u32) -> ();
    }

    wl_proxy_marshal(surface, 0 /*WL_SURFACE_DESTROY*/);
    super::wl_proxy_destroy(surface);
}
