#![allow(unused)]

use super::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub(super) enum WlSeatCapability {
    Pointer = 1,
    Keyboard = 2,
    Touch = 4,
}

#[repr(C)]
pub(super) struct WlInterface {
    /** Interface name */
    pub(super) name: *const c_void,
    /** Interface version */
    pub(super) version: i32,
    /** Number of methods (requests) */
    pub(super) method_count: i32,
    /** Method (request) signatures */
    pub(super) methods: *const c_void, // *wl_message
    /** Number of events */
    pub(super) event_count: i32,
    /** Event signatures */
    pub(super) events: *const c_void, // *wl_message
}

#[repr(C)]
pub(super) struct WlCursor {
    pub(super) image_count: u32,
    pub(super) images: *mut *mut WlCursorImage,
    pub(super) name: *mut c_void,
}

#[repr(C)]
pub(super) struct WlCursorImage {
    pub(super) width: u32,     /* actual width */
    pub(super) height: u32,    /* actual height */
    pub(super) hotspot_x: u32, /* hot spot x (must be inside image) */
    pub(super) hotspot_y: u32, /* hot spot y (must be inside image) */
    pub(super) delay: u32,     /* animation delay to next frame (ms) */
}

pub(super) unsafe fn wl_surface_destroy(surface: *mut c_void) {
    extern "C" {
        fn wl_proxy_marshal(p: *mut c_void, opcode: u32) -> ();
    }

	wl_proxy_marshal(surface, 0 /*WL_SURFACE_DESTROY*/);
	super::wl_proxy_destroy(surface);
}
