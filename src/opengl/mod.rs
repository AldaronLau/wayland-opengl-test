use std::ffi::c_void;

use super::Draw;
use super::Nwin;
use super::Window;

mod platform;

extern "C" {
     fn eglGetDisplay(native_display: self::platform::NativeDisplayType) -> *mut c_void;
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
    // Get EGL Display from Window.
    let display = unsafe {
        eglGetDisplay(window.nwin.handle())
    };
    debug_assert!(!display.is_null());

    // TODO: Instead of writing to Nwin, write to Draw.  Avoids unsafe.
    unsafe {
        (*(*std::mem::transmute::<&Box<_>, &[*mut crate::wayland::WaylandWindow; 2]>(&window.nwin))[0]).egl_dpy = display;
    }

    let draw = OpenGL {
//        display,
    };

    Some(Box::new(draw))
}
