use std::ffi::c_void;

#[link(name = "wayland-client")]
#[link(name = "wayland-egl")]
#[link(name = "wayland-cursor")]
#[link(name = "EGL")]
#[link(name = "GL")]
extern "C" {
    static wl_registry_interface: WlInterface;

    fn wl_display_connect(name: *mut c_void) -> *mut c_void;
    fn wl_display_disconnect(name: *mut c_void) -> ();
    fn wl_display_flush(name: *mut c_void) -> i32;
    fn wl_proxy_marshal_constructor(name: *mut c_void, opcode: u32,
        interface: *const c_void, p: *mut c_void) -> *mut c_void;
    fn dive_wayland(c: *mut Context) -> ();
}

#[repr(C)]
struct WlInterface {
    /** Interface name */
    name: *const c_void,
    /** Interface version */
    version: i32,
    /** Number of methods (requests) */
    method_count: i32,
    /** Method (request) signatures */
    methods: *const c_void, // *wl_message
    /** Number of events */
    event_count: i32,
    /** Event signatures */
    events: *const c_void, // *wl_message
}

#[repr(C)]
pub struct Context {
    running: i32,

    window_width: i32,
    window_height: i32,

	gl_rotation_uniform: u32,
	gl_pos: u32,
	gl_col: u32,

	native: *mut c_void, // wl_egl_window*
	surface: *mut c_void, // wl_surface*
	shell_surface: *mut c_void, // wl_shell_surface*

	egl_surface: *mut c_void, // EGLSurface
	callback: *mut c_void, // wl_callback*
    configured: i32,
    fullscreen: bool,

	wldisplay: *mut c_void, // wl_display*
	registry: *mut c_void, // wl_registry*
	compositor: *mut c_void, // wl_compositor*
	shell: *mut c_void, // wl_shell*
	seat: *mut c_void, // wl_seat*
	pointer: *mut c_void, // wl_pointer*
	keyboard: *mut c_void, // wl_keyboard*
	shm: *mut c_void, // wl_shm*
	cursor_theme: *mut c_void, // wl_cursor_theme*
	default_cursor: *mut c_void, // wl_cursor*
	cursor_surface: *mut c_void, // wl_surface*

	egl_dpy: *mut c_void, // EGLDisplay
	egl_ctx: *mut c_void, // EGLContext
	egl_conf: *mut c_void, // EGLConfig
}

impl Drop for Context {
    fn drop(&mut self) {
        unsafe {
            wl_display_flush(self.wldisplay);
            wl_display_disconnect(self.wldisplay);
        }
    }
}

/// Start the Wayland + OpenGL application.
pub fn start() -> Option<Context> {
    let wldisplay = unsafe {
        wl_display_connect(std::ptr::null_mut())
    };
    if wldisplay.is_null() { return None }

    let registry = unsafe {
        wl_proxy_marshal_constructor(wldisplay, 1 /*WL_DISPLAY_GET_REGISTRY*/,
            &wl_registry_interface as *const _ as *const _, std::ptr::null_mut())
    };

    let mut context = Context {
        running: 1,

        window_width: 640,
        window_height: 360,

	    gl_rotation_uniform: 0,
	    gl_pos: 0,
	    gl_col: 0,

	    native: std::ptr::null_mut(), // wl_egl_window*
	    surface: std::ptr::null_mut(), // wl_surface*
	    shell_surface: std::ptr::null_mut(), // wl_shell_surface*

	    egl_surface: std::ptr::null_mut(), // EGLSurface
	    callback: std::ptr::null_mut(), // wl_callback*
        configured: 1,
        fullscreen: false,

	    wldisplay,
	    registry, // wl_registry*
	    compositor: std::ptr::null_mut(), // wl_compositor*
	    shell: std::ptr::null_mut(), // wl_shell*
	    seat: std::ptr::null_mut(), // wl_seat*
	    pointer: std::ptr::null_mut(), // wl_pointer*
	    keyboard: std::ptr::null_mut(), // wl_keyboard*
	    shm: std::ptr::null_mut(), // wl_shm*
	    cursor_theme: std::ptr::null_mut(), // wl_cursor_theme*
	    default_cursor: std::ptr::null_mut(), // wl_cursor*
	    cursor_surface: std::ptr::null_mut(), // wl_surface*

	    egl_dpy: std::ptr::null_mut(), // EGLDisplay
	    egl_ctx: std::ptr::null_mut(), // EGLContext
	    egl_conf: std::ptr::null_mut(), // EGLConfig
    };

    unsafe {
        dive_wayland(&mut context);
    }

    Some(context)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
