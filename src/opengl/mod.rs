use std::ffi::c_void;

use super::Draw;
use super::Nwin;
use super::Window;

mod platform;

extern "C" {
    fn eglGetDisplay(native_display: self::platform::NativeDisplayType) -> *mut c_void;
    fn eglInitialize(dpy: *mut c_void, major: *mut i32, minor: *mut i32) -> u32;
}

#[repr(C)]
pub struct OpenGL {
//    display: *mut c_void,
}

impl Drop for OpenGL {
    fn drop(&mut self) {

    }
}

impl Draw for OpenGL {
}

pub(super) fn new(window: &mut Window) -> Option<Box<Draw>> {
    let display = unsafe {
        // Get EGL Display from Window.
        let display = eglGetDisplay(window.nwin.handle());
        debug_assert!(!display.is_null());

        // Initialize EGL Display.
        let mut major = std::mem::uninitialized();
        let mut minor = std::mem::uninitialized();
        let rtn = eglInitialize(display, &mut major, &mut minor);
        debug_assert_eq!(rtn, 1);

        //

        // TODO: Instead of writing to Nwin, write to Draw.  Avoids unsafe.
        (*(*std::mem::transmute::<&Box<_>, &[*mut crate::wayland::WaylandWindow; 2]>(&window.nwin))[0]).egl_dpy = display;

        display
    };

    let draw = OpenGL {
//        display,
    };

    Some(Box::new(draw))
}
