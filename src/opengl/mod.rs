use std::ffi::c_void;

use super::Draw;
use super::Nwin;
use super::Window;
use super::DrawHandle;

mod platform;

extern "C" {
    fn eglGetDisplay(
        native_display: self::platform::NativeDisplayType,
    ) -> *mut c_void;
    fn eglInitialize(dpy: *mut c_void, major: *mut i32, minor: *mut i32)
        -> u32;
    fn eglBindAPI(api: u32) -> u32;
    fn eglChooseConfig(
        dpy: *mut c_void,
        attrib_list: *const i32,
        configs: *mut *mut c_void,
        config_size: i32,
        num_config: &mut i32,
    ) -> u32;
    fn eglCreateContext(
        dpy: *mut c_void,
        config: *mut c_void,
        share_context: *mut c_void,
        attrib_list: *const i32,
    ) -> *mut c_void;
    fn eglCreateWindowSurface(
        dpy: *mut c_void,
        config: *mut c_void,
        win: usize, // EGLNativeWindowType
        attrib_list: *const i32,
    ) -> *mut c_void;
    fn eglMakeCurrent(
        dpy: *mut c_void,
        draw: *mut c_void,
        read: *mut c_void,
        ctx: *mut c_void,
    ) -> u32;
}

#[repr(C)]
pub struct OpenGL {
    surface: *mut c_void,
    display: *mut c_void,
    context: *mut c_void,
    config: *mut c_void,
}

impl Drop for OpenGL {
    fn drop(&mut self) {}
}

impl Draw for OpenGL {
    fn handle(&self) -> DrawHandle {
        // TODO
        DrawHandle::Gl(std::ptr::null_mut())
    }

    fn connect(&mut self, connection: *mut c_void) {
    	self.surface = unsafe { eglCreateWindowSurface(
            self.display,
            self.config,
            std::mem::transmute(connection),
            std::ptr::null(),
        ) };
	    let ret = unsafe { eglMakeCurrent(
            self.display,
            self.surface,
            self.surface,
            self.context,
        ) };
	    debug_assert_ne!(ret, 0);
        println!("OOF");
    }
}

#[cfg(unix)]
pub(super) fn new(window: &mut Window) -> Option<Box<Draw>> {
    let (display, config, context) = unsafe {
        // Get EGL Display from Window.
        let display = eglGetDisplay(match window.nwin.handle() {
            #[cfg(not(any(
                target_os = "android",
                target_os = "macos",
                target_os = "ios"
            )))]
            crate::NwinHandle::Wayland(handle) => handle,
        });
        debug_assert!(!display.is_null());

        // Initialize EGL Display.
        let mut major = std::mem::uninitialized();
        let mut minor = std::mem::uninitialized();
        let rtn = eglInitialize(display, &mut major, &mut minor);
        debug_assert_eq!(rtn, 1);

        // Connect EGL to either OpenGL or OpenGLES, whichever is available.
        // TODO: also support /*OPENGL:*/ 0x30A2
        let ret = eglBindAPI(/*OPENGL_ES:*/ 0x30A0);
        debug_assert_eq!(rtn, 1);

        //
        let mut config: *mut c_void = std::mem::uninitialized();
        let mut n: i32 = std::mem::uninitialized();
        let ret = eglChooseConfig(
            display,
            [
                /*EGL_SURFACE_TYPE:*/ 0x3033,
                /*EGL_WINDOW_BIT:*/ 0x04,
                /*EGL_RED_SIZE:*/ 0x3024,
                8,
                /*EGL_GREEN_SIZE:*/ 0x3023,
                8,
                /*EGL_BLUE_SIZE:*/ 0x3022,
                8,
                /*EGL_RENDERABLE_TYPE:*/ 0x3040,
                /*EGL_OPENGL_ES2_BIT:*/ 0x0004,
                /*EGL_NONE:*/ 0x3038,
            ].as_ptr(),
            &mut config,
            1,
            &mut n,
        );
        debug_assert_eq!(rtn, 1);

        //
        let context = eglCreateContext(
            display,
            config,
            std::ptr::null_mut(),
            [
                /*EGL_CONTEXT_CLIENT_VERSION:*/ 0x3098, 2,
                /*EGL_NONE:*/ 0x3038,
            ].as_ptr(),
        );
        debug_assert!(!context.is_null());

        (display, config, context)
    };

    let draw: OpenGL = OpenGL {
        display,
        config,
        context,
        surface: std::ptr::null_mut(),
    };

    Some(Box::new(draw))
}
