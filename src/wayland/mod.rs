use std::ffi::c_void;

mod keycodes;
mod wl;

use super::Nwin;
use self::keycodes::*;
pub(super) use self::wl::*;

#[link(name = "wayland-client")]
#[link(name = "wayland-egl")]
#[link(name = "wayland-cursor")]
#[link(name = "EGL")]
#[link(name = "GL")]
extern "C" {
    fn strcmp(s1: *const c_void, s2: *const c_void) -> i32;

    pub(super) static wl_registry_interface: WlInterface;
    static wl_compositor_interface: WlInterface;
    static wl_shell_interface: WlInterface;
    static wl_seat_interface: WlInterface;
    static wl_shm_interface: WlInterface;
    static wl_pointer_interface: WlInterface;
    static wl_keyboard_interface: WlInterface;
    static wl_touch_interface: WlInterface;
    static wl_callback_interface: WlInterface;
    static zxdg_shell_v6_interface: WlInterface;

    pub(super) fn wl_display_connect(name: *mut c_void) -> *mut c_void;
    fn wl_display_disconnect(display: *mut c_void) -> ();
    fn wl_display_flush(display: *mut c_void) -> i32;
    pub(super) fn wl_display_dispatch(display: *mut c_void) -> i32;
    pub(super) fn wl_proxy_marshal_constructor(
        name: *mut c_void,
        opcode: u32,
        interface: &WlInterface,
        p: *mut c_void,
    ) -> *mut c_void;
    pub(super) fn wl_proxy_add_listener(
        proxy: *mut c_void,
        implementation: *const *mut c_void,
        data: *mut WaylandWindow,
    ) -> i32;
    fn wl_proxy_marshal_constructor_versioned(
        proxy: *mut c_void,
        opcode: u32,
        interface: *const WlInterface,
        version: u32,
        name: u32,
        name2: *const c_void,
        version2: u32,
        pointer: *mut c_void,
    ) -> *mut c_void;
    fn wl_cursor_theme_load(
        name: *const c_void,
        size: i32,
        shm: *mut c_void,
    ) -> *mut c_void;
    fn wl_cursor_theme_get_cursor(
        theme: *mut c_void,
        name: *const c_void,
    ) -> *mut WlCursor;
    fn wl_cursor_theme_destroy(theme: *mut c_void) -> ();

    fn wl_proxy_destroy(proxy: *mut c_void) -> ();
    fn wl_cursor_image_get_buffer(image: *mut WlCursorImage) -> *mut c_void;

    pub fn dive_wayland(c: *mut WaylandWindow) -> ();
}

/*unsafe extern "C" fn configure_callback(c: *mut WaylandWindow,
    callback: *mut c_void, time: u32)
{
    wl_callback_destroy(callback);

    printf("GL2 %d %d\n", (*c).window_width, (*c).window_height);
    glViewport(0, 0, (*c).window_width, (*c).window_height);

    if ((*c).callback == NULL)
        redraw(c, std::ptr::null_mut(), time);
}

static CONFIGURE_CALLBACK_LISTENER: [*mut c_void; 1] = [
    configure_callback,
];*/

extern "C" {
    static CONFIGURE_CALLBACK_LISTENER: [*mut c_void; 1];
    static XDG_SHELL_LISTENER: [*mut c_void; 1];
}

unsafe extern "C" fn pointer_handle_enter(
    c: *mut WaylandWindow,
    pointer: *mut c_void,
    serial: u32,
    surface: *mut c_void,
    sx: i32,
    sy: i32,
) {
    let cursor = (*c).default_cursor;
    let image = *(*(*c).default_cursor).images;
    let buffer = wl_cursor_image_get_buffer(image);
    if buffer.is_null() {
        return;
    }

    {
        extern "C" {
            fn wl_proxy_marshal(
                p: *mut c_void,
                opcode: u32,
                a: u32,
                b: *mut c_void,
                c: u32,
                d: u32,
            ) -> ();
        }

        wl_proxy_marshal(
            pointer,
            0, /*WL_POINTER_SET_CURSOR*/
            serial,
            (*c).cursor_surface,
            (*image).hotspot_x,
            (*image).hotspot_y,
        );
    }
    {
        extern "C" {
            fn wl_proxy_marshal(
                p: *mut c_void,
                opcode: u32,
                a: *mut c_void,
                b: i32,
                c: i32,
            ) -> ();
        }

        wl_proxy_marshal(
            (*c).cursor_surface,
            1, /*WL_SURFACE_ATTACH*/
            buffer,
            0,
            0,
        );
    }
    {
        extern "C" {
            fn wl_proxy_marshal(
                p: *mut c_void,
                opcode: u32,
                a: u32,
                b: u32,
                c: u32,
                d: u32,
            ) -> ();
        }

        wl_proxy_marshal(
            (*c).cursor_surface,
            2, /*WL_SURFACE_DAMAGE*/
            0,
            0,
            (*image).width,
            (*image).height,
        );
    }
    {
        extern "C" {
            fn wl_proxy_marshal(p: *mut c_void, opcode: u32) -> ();
        }

        wl_proxy_marshal((*c).cursor_surface, 6 /*WL_SURFACE_COMMIT*/);
    }

    // Hide cursor
    //	wl_pointer_set_cursor(pointer, serial, (*c).cursor_surface, 0, 0);
}

unsafe extern "C" fn pointer_handle_leave(
    c: *mut WaylandWindow,
    pointer: *mut c_void,
    serial: u32,
    surface: *mut c_void,
) {
}

unsafe extern "C" fn pointer_handle_motion(
    c: *mut WaylandWindow,
    pointer: *mut c_void,
    time: u32,
    sx: i32,
    sy: i32,
) {
}

unsafe extern "C" fn pointer_handle_button(
    c: *mut WaylandWindow,
    pointer: *mut c_void,
    serial: u32,
    time: u32,
    button: u32,
    state: u32,
) {
    const BTN_LEFT: u32 = 0x110;
    const BTN_RIGHT: u32 = 0x111;
    const BTN_MIDDLE: u32 = 0x112;
    const BTN_SIDE: u32 = 0x113;

    extern "C" {
        fn wl_proxy_marshal(
            p: *mut c_void,
            opcode: u32,
            a: *mut c_void,
            b: u32,
        ) -> ();
    }

    if button == BTN_LEFT {
        if state == 1
        /*pressed*/
        {
            wl_proxy_marshal((*c).shell_surface, 1, (*c).seat, serial);
        }
    }
}

unsafe extern "C" fn pointer_handle_axis(
    c: *mut WaylandWindow,
    pointer: *mut c_void,
    time: u32,
    axis: u32,
    value: i32,
) {
}

static mut POINTER_LISTENER: [*mut c_void; 9] = [
    pointer_handle_enter as *mut _,
    pointer_handle_leave as *mut _,
    pointer_handle_motion as *mut _,
    pointer_handle_button as *mut _,
    pointer_handle_axis as *mut _,
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
    std::ptr::null_mut(),
];

unsafe extern "C" fn keyboard_handle_keymap(
    c: *mut WaylandWindow,
    keyboard: *mut c_void,
    format: u32,
    fd: i32,
    size: u32,
) {
}

unsafe extern "C" fn keyboard_handle_enter(
    c: *mut WaylandWindow,
    keyboard: *mut c_void,
    serial: u32,
    surface: *mut c_void,
    keys: *mut c_void,
) {
}

unsafe extern "C" fn keyboard_handle_leave(
    c: *mut WaylandWindow,
    keyboard: *mut c_void,
    serial: u32,
    surface: *mut c_void,
) {
}

unsafe extern "C" fn keyboard_handle_key(
    c: *mut WaylandWindow,
    keyboard: *mut c_void,
    serial: u32,
    time: u32,
    key: u32,
    state: u32,
) {
    if key == KEY_ESC && state != 0 {
        (*c).running = 0;
    } else if key == KEY_F11 && state != 0 {
        (*c).configured = 1;

        if (*c).fullscreen {
            if (*c).is_restored != 0 {
                // Restore
                extern "C" {
                    fn wl_proxy_marshal(p: *mut c_void, opcode: u32) -> ();
                }

                wl_proxy_marshal((*c).shell_surface, 3 /*toplevel*/);
            } else {
                // Maximize
                extern "C" {
                    fn wl_proxy_marshal(
                        p: *mut c_void,
                        opcode: u32,
                        a: *mut c_void,
                    ) -> ();
                }

                wl_proxy_marshal(
                    (*c).shell_surface,
                    7, /*maximized*/
                    std::ptr::null_mut(),
                );
            }

            (*c).fullscreen = false;
        } else {
            extern "C" {
                fn wl_proxy_marshal(
                    p: *mut c_void,
                    opcode: u32,
                    a: u32,
                    b: u32,
                    c: *mut c_void,
                ) -> ();
            }

            wl_proxy_marshal(
                (*c).shell_surface,
                5,    /*fullscreen*/
                0u32, /* WL_SHELL_SURFACE_FULLSCREEN_METHOD_DEFAULT */
                0u32,
                std::ptr::null_mut(),
            );

            (*c).fullscreen = true;
        }

        let callback = wl_proxy_marshal_constructor(
            (*c).wldisplay,
            0, /*WL_DISPLAY_SYNC*/
            &wl_callback_interface,
            std::ptr::null_mut(),
        );

        println!("CONFIGYO");
        wl_proxy_add_listener(
            callback,
            CONFIGURE_CALLBACK_LISTENER.as_ptr(),
            c,
        );
        println!("CONFIGYA");
    }
}

unsafe extern "C" fn keyboard_handle_modifiers(
    c: *mut WaylandWindow,
    keyboard: *mut c_void,
    serial: u32,
    mods_depressed: u32,
    mods_latched: u32,
    mods_locked: u32,
    group: u32,
) {
}

static mut KEYBOARD_LISTENER: [*mut c_void; 6] = [
    keyboard_handle_keymap as *mut _,
    keyboard_handle_enter as *mut _,
    keyboard_handle_leave as *mut _,
    keyboard_handle_key as *mut _,
    keyboard_handle_modifiers as *mut _,
    std::ptr::null_mut(),
];

unsafe extern "C" fn seat_handle_capabilities(
    c: *mut WaylandWindow,
    seat: *mut c_void,
    caps: WlSeatCapability,
) {
    println!("SEAAT");

    // Allow Pointer Events
    let has_pointer = (caps as u32 & WlSeatCapability::Pointer as u32) != 0;
    if has_pointer && (*c).pointer.is_null() {
        (*c).pointer = wl_proxy_marshal_constructor(
            seat,
            0,
            &wl_pointer_interface,
            std::ptr::null_mut(),
        );
        wl_proxy_add_listener((*c).pointer, POINTER_LISTENER.as_ptr(), c);
    } else if !has_pointer && !(*c).pointer.is_null() {
        wl_proxy_destroy((*c).pointer);
        (*c).pointer = std::ptr::null_mut();
    }

    // Allow Keyboard Events
    let has_keyboard = (caps as u32 & WlSeatCapability::Keyboard as u32) != 0;
    if has_keyboard && (*c).keyboard.is_null() {
        (*c).keyboard = wl_proxy_marshal_constructor(
            seat,
            1,
            &wl_keyboard_interface,
            std::ptr::null_mut(),
        );
        wl_proxy_add_listener((*c).keyboard, KEYBOARD_LISTENER.as_ptr(), c);
    } else if !has_keyboard && !(*c).keyboard.is_null() {
        wl_proxy_destroy((*c).keyboard);
        (*c).keyboard = std::ptr::null_mut();
    }

    println!("SEAT!");

    // Allow Touch Events
    // TODO
    /*
        let has_keyboard = (caps as u32 & WlSeatCapability::Keyboard as u32) != 0;
        if has_keyboard && (*c).keyboard.is_null() {
            (*c).keyboard = wl_proxy_marshal_constructor(seat, 2,
                &wl_touch_interface, std::ptr::null_mut());
            wl_proxy_add_listener((*c).keyboard, keyboard_listener.as_ptr(), c);
        } else if !has_keyboard && !(*c).keyboard.is_null() {
            wl_proxy_destroy((*c).keyboard);
            (*c).keyboard = std::ptr::null_mut();
        }
    */
}

unsafe extern "C" fn registry_handle_global(
    c: *mut WaylandWindow,
    registry: *mut c_void,
    name: u32,
    interface: *const c_void, // text
    version: u32,
) {
    if strcmp(interface, b"wl_compositor\0" as *const _ as *const _) == 0 {
        (*c).compositor = wl_proxy_marshal_constructor_versioned(
            registry,
            0, /*WL_REGISTRY_BIND*/
            &wl_compositor_interface,
            1,
            name,
            wl_compositor_interface.name,
            1,
            std::ptr::null_mut(),
        );
    } else if (strcmp(interface, b"zxdg_shell_v6\0" as *const _ as *const _) == 0) {
        println!("Initializing XDG-SHELL");

        (*c).shell = wl_proxy_marshal_constructor_versioned(
            registry,
            0, /*WL_REGISTRY_BIND*/
            &zxdg_shell_v6_interface,
            1,
            name,
            zxdg_shell_v6_interface.name,
            1,
            std::ptr::null_mut(),
        );

        wl_proxy_add_listener((*c).shell, XDG_SHELL_LISTENER.as_ptr(), c);

        println!("Initialized XDG-SHELL");
    } else if strcmp(interface, b"wl_shell\0" as *const _ as *const _) == 0 {
/*        (*c).shell = wl_proxy_marshal_constructor_versioned(
            registry,
            0, /*WL_REGISTRY_BIND*/
            &wl_shell_interface,
            1,
            name,
            wl_shell_interface.name,
            1,
            std::ptr::null_mut(),
        );*/
    } else if strcmp(interface, b"wl_seat\0" as *const _ as *const _) == 0 {
        (*c).seat = wl_proxy_marshal_constructor_versioned(
            registry,
            0, /*WL_REGISTRY_BIND*/
            &wl_seat_interface,
            1,
            name,
            wl_seat_interface.name,
            1,
            std::ptr::null_mut(),
        );

        wl_proxy_add_listener((*c).seat, SEAT_LISTENER.as_ptr(), c);
    } else if strcmp(interface, b"wl_shm\0" as *const _ as *const _) == 0 {
        (*c).shm = wl_proxy_marshal_constructor_versioned(
            registry,
            0, /*WL_REGISTRY_BIND*/
            &wl_shm_interface,
            1,
            name,
            wl_shm_interface.name,
            1,
            std::ptr::null_mut(),
        );

        (*c).cursor_theme =
            wl_cursor_theme_load(std::ptr::null_mut(), 16, (*c).shm);
        if (*c).cursor_theme.is_null() {
            eprintln!("unable to load default theme");
        }
        (*c).default_cursor = wl_cursor_theme_get_cursor(
            (*c).cursor_theme,
            b"left_ptr\0" as *const _ as *const _,
        );
        if (*c).default_cursor.is_null() {
            panic!("unable to load default left pointer");
        }
    }
}

unsafe extern "C" fn registry_handle_global_remove(
    data: *mut c_void,
    registry: *mut c_void,
    name: u32,
) {
}

pub(super) static mut SEAT_LISTENER: [*mut c_void; 2] =
    [seat_handle_capabilities as *mut _, std::ptr::null_mut()];

pub(super) static mut REGISTRY_LISTENER: [*mut c_void; 2] = [
    registry_handle_global as *mut _,
    registry_handle_global_remove as *mut _,
];

#[repr(C)]
pub struct WaylandWindow {
    pub(super) running: i32,
    pub(super) is_restored: i32,

    pub(super) window_width: i32,
    pub(super) window_height: i32,

    pub(super) restore_width: i32,
    pub(super) restore_height: i32,

    pub(super) gl_rotation_uniform: u32,
    pub(super) gl_pos: u32,
    pub(super) gl_col: u32,

    pub(super) native: *mut c_void,        // wl_egl_window*
    pub(super) surface: *mut c_void,       // wl_surface*
    pub(super) shell_surface: *mut c_void, // wl_shell_surface*

    pub(super) egl_surface: *mut c_void, // EGLSurface
    pub(super) callback: *mut c_void,    // wl_callback*
    pub(super) configured: i32,
    pub(super) fullscreen: bool,

    pub(super) wldisplay: *mut c_void,        // wl_display*
    pub(super) registry: *mut c_void,         // wl_registry*
    pub(super) compositor: *mut c_void,       // wl_compositor*
    pub(super) shell: *mut c_void,            // wl_shell*
    pub(super) seat: *mut c_void,             // wl_seat*
    pub(super) pointer: *mut c_void,          // wl_pointer*
    pub(super) keyboard: *mut c_void,         // wl_keyboard*
    pub(super) shm: *mut c_void,              // wl_shm*
    pub(super) cursor_theme: *mut c_void,     // wl_cursor_theme*
    pub(super) default_cursor: *mut WlCursor, // wl_cursor*
    pub(super) cursor_surface: *mut c_void,   // wl_surface*
    pub(super) toplevel: *mut c_void,         // void*

    pub(super) egl_dpy: *mut c_void,  // EGLDisplay
    pub(super) egl_ctx: *mut c_void,  // EGLContext
    pub(super) egl_conf: *mut c_void, // EGLConfig
}

impl Drop for WaylandWindow {
    fn drop(&mut self) {
        unsafe {
            wl_surface_destroy(self.cursor_surface);
            if !self.cursor_theme.is_null() {
                wl_cursor_theme_destroy(self.cursor_theme);
            }
            if !self.shell.is_null() {
                wl_proxy_destroy(self.shell);
            }
            if !self.compositor.is_null() {
                wl_proxy_destroy(self.compositor);
            }
            wl_proxy_destroy(self.registry);
            wl_display_flush(self.wldisplay);
            wl_display_disconnect(self.wldisplay);
        }
    }
}

impl Nwin for WaylandWindow {

}

pub(super) fn new() -> Option<Box<Nwin>> {
    let wldisplay = unsafe { wl_display_connect(std::ptr::null_mut()) };
    if wldisplay.is_null() {
        return None;
    }

    let registry = unsafe {
        wl_proxy_marshal_constructor(
            wldisplay,
            1, /*WL_DISPLAY_GET_REGISTRY*/
            &wl_registry_interface,
            std::ptr::null_mut(),
        )
    };

    let mut nwin = Box::new(WaylandWindow {
        running: 1,
        is_restored: 0,

        window_width: 640,
        window_height: 360,

        restore_width: 640,
        restore_height: 360,

        gl_rotation_uniform: 0,
        gl_pos: 0,
        gl_col: 0,

        native: std::ptr::null_mut(), // wl_egl_window*
        surface: std::ptr::null_mut(), // wl_surface*
        shell_surface: std::ptr::null_mut(), // wl_shell_surface*

        egl_surface: std::ptr::null_mut(), // EGLSurface
        callback: std::ptr::null_mut(),    // wl_callback*
        configured: 1,
        fullscreen: false,

        wldisplay,
        registry,                             // wl_registry*
        compositor: std::ptr::null_mut(),     // wl_compositor*
        shell: std::ptr::null_mut(),          // wl_shell*
        seat: std::ptr::null_mut(),           // wl_seat*
        pointer: std::ptr::null_mut(),        // wl_pointer*
        keyboard: std::ptr::null_mut(),       // wl_keyboard*
        shm: std::ptr::null_mut(),            // wl_shm*
        cursor_theme: std::ptr::null_mut(),   // wl_cursor_theme*
        default_cursor: std::ptr::null_mut(), // wl_cursor*
        cursor_surface: std::ptr::null_mut(), // wl_surface*
        toplevel: std::ptr::null_mut(),       // void*

        egl_dpy: std::ptr::null_mut(), // EGLDisplay
        egl_ctx: std::ptr::null_mut(), // EGLContext
        egl_conf: std::ptr::null_mut(), // EGLConfig
    });

    println!("REG!!");
    unsafe {
        wl_proxy_add_listener(
            nwin.registry,
            REGISTRY_LISTENER.as_ptr(),
            (*std::mem::transmute::<&Box<_>, &[*mut WaylandWindow; 2]>(&nwin))[0],
        );

        wl_display_dispatch(nwin.wldisplay);
    }
    println!("REGGG!!");

    Some(nwin)
}
