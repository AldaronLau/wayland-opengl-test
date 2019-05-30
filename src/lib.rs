/*use std::ffi::c_void;

struct Display {
	struct wl_registry *registry: *mut c_void,
	struct wl_compositor *compositor: *mut c_void,
	struct wl_shell *shell: *mut c_void,
	struct wl_seat *seat: *mut c_void,
	struct wl_pointer *pointer: *mut c_void,
	struct wl_keyboard *keyboard: *mut c_void,
	struct wl_shm *shm: *mut c_void,
	struct wl_cursor_theme *cursor_theme: *mut c_void,
	struct wl_cursor *default_cursor: *mut c_void,
	struct wl_surface *cursor_surface: *mut c_void,
	struct {
		EGLDisplay dpy;
		EGLContext ctx;
		EGLConfig conf;
	} egl;
	struct window *window: *mut c_void,
}*/

/// Start the Wayland + OpenGL application.
pub fn start() {
    #[link(name = "wayland-client")]
    #[link(name = "wayland-egl")]
    #[link(name = "wayland-cursor")]
    #[link(name = "EGL")]
    #[link(name = "GL")]
    extern "C" {
        fn dive_wayland() -> ();
    }

    unsafe {
        dive_wayland();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
