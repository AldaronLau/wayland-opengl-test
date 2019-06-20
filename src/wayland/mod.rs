use std::ffi::c_void;

mod keycodes;
mod wl;

use self::keycodes::*;
pub(super) use self::wl::*;
use super::Nwin;

#[link(name = "wayland-client")]
#[link(name = "wayland-egl")]
#[link(name = "wayland-cursor")]
#[link(name = "EGL")]
//#[link(name = "GL")]
#[link(name = "GLESv2")]
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
    static wl_surface_interface: WlInterface;

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
        data: *mut crate::Window,
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

    pub fn dive_wayland(c: *mut crate::Window) -> ();

    // Wayland EGL:
    fn wl_egl_window_create(
        surface: *mut c_void,
        width: i32,
        height: i32,
    ) -> *mut c_void;
    fn wl_egl_window_resize(
        egl_window: *mut c_void,
        width: i32,
        height: i32,
        dx: i32,
        dy: i32,
    ) -> ();
    fn wl_egl_window_destroy(egl_window: *mut c_void) -> ();
    fn glViewport(x: i32, y: i32, width: i32, height: i32) -> ();
}

fn get(value: &mut Box<dyn Nwin>) -> *mut Box<WaylandWindow> {
    value as *mut _ as *mut _
}

extern "C" {
    fn redraw(c: *mut crate::Window, callback: *mut c_void, millis: u32) -> ();
}

static mut ZXDG_SURFACE_V6_INTERFACE: WlInterface = WlInterface {
    /** Interface name */
    name: b"zxdg_surface_v6\0".as_ptr() as *const _,
    /** Interface version */
    version: 1,
    /** Number of methods (requests) */
    method_count: 5,
    /** Method (request) signatures */
    methods: [
	    WlMessage {
            name: b"destroy\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"get_toplevel\0".as_ptr() as *const _,
            signature: b"n\0".as_ptr() as *const _,
            wl_interface: unsafe { &(&wl_surface_interface as *const _) },
        },
	    WlMessage {
            name: b"get_popup\0".as_ptr() as *const _,
            signature: b"noo\0".as_ptr() as *const _,
            wl_interface: unsafe { &(&ZXDG_TOPLEVEL_V6_INTERFACE as *const _) },
        },
	    WlMessage {
            name: b"set_window_geometry\0".as_ptr() as *const _,
            signature: b"iiii\0".as_ptr() as *const _,
            wl_interface: std::ptr::null(),
        },
	    WlMessage {
            name: b"ack_configure\0".as_ptr() as *const _,
            signature: b"u\0".as_ptr() as *const _,
            wl_interface: std::ptr::null(),
        },
    ].as_ptr(),
    /** Number of events */
    event_count: 1,
    /** Event signatures */
    events: [
        WlMessage {
            name: b"configure\0".as_ptr() as *const _,
            signature: b"u\0".as_ptr() as *const _,
            wl_interface: std::ptr::null(),
        },
    ].as_ptr(), // *wl_message
};

static mut ZXDG_TOPLEVEL_V6_INTERFACE: WlInterface = WlInterface {
    /** Interface name */
    name: b"zxdg_toplevel_v6\0".as_ptr() as *const _,
    /** Interface version */
    version: 1,
    /** Number of methods (requests) */
    method_count: 14,
    /** Method (request) signatures */
    methods: [
	    WlMessage {
            name: b"destroy\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_parent\0".as_ptr() as *const _,
            signature: b"?o\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_title\0".as_ptr() as *const _,
            signature: b"s\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_app_id\0".as_ptr() as *const _,
            signature: b"s\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"show_window_menu\0".as_ptr() as *const _,
            signature: b"ouii\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"move\0".as_ptr() as *const _,
            signature: b"ou\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"resize\0".as_ptr() as *const _,
            signature: b"ouu\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_max_size\0".as_ptr() as *const _,
            signature: b"ii\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_min_size\0".as_ptr() as *const _,
            signature: b"ii\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_maximized\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"unset_maximized\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_fullscreen\0".as_ptr() as *const _,
            signature: b"?o\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"unset_fullscreen\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"set_minimized\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
    ].as_ptr(),
    /** Number of events */
    event_count: 2,
    /** Event signatures */
    events: [
        WlMessage {
            name: b"configure\0".as_ptr() as *const _,
            signature: b"iia\0".as_ptr() as *const _,
            wl_interface: std::ptr::null(),
        },
        WlMessage {
            name: b"close\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null(),
        },
    ].as_ptr(), // *wl_message
};

static mut ZXDG_SHELL_V6_INTERFACE: WlInterface = WlInterface {
    /** Interface name */
    name: b"zxdg_shell_v6\0".as_ptr() as *const _,
    /** Interface version */
    version: 1,
    /** Number of methods (requests) */
    method_count: 4,
    /** Method (request) signatures */
    methods: [
	    WlMessage {
            name: b"destroy\0".as_ptr() as *const _,
            signature: b"\0".as_ptr() as *const _,
            wl_interface: std::ptr::null()
        },
	    WlMessage {
            name: b"create_positioner\0".as_ptr() as *const _,
            signature: b"n\0".as_ptr() as *const _,
            wl_interface: unsafe { &(&wl_surface_interface as *const _) },
        },
	    WlMessage {
            name: b"get_xdg_surface\0".as_ptr() as *const _,
            signature: b"no\0".as_ptr() as *const _,
            wl_interface: unsafe { &(&ZXDG_TOPLEVEL_V6_INTERFACE as *const _) },
        },
	    WlMessage {
            name: b"pong\0".as_ptr() as *const _,
            signature: b"u\0".as_ptr() as *const _,
            wl_interface: std::ptr::null(),
        },
    ].as_ptr(),
    /** Number of events */
    event_count: 1,
    /** Event signatures */
    events: [
        WlMessage {
            name: b"ping\0".as_ptr() as *const _,
            signature: b"u\0".as_ptr() as *const _,
            wl_interface: std::ptr::null(),
        },
    ].as_ptr(), // *wl_message
};

unsafe extern "C" fn pointer_handle_enter(
    window: *mut crate::Window,
    pointer: *mut c_void,
    serial: u32,
    surface: *mut c_void,
    sx: i32,
    sy: i32,
) {
    let c = get(&mut (*window).nwin);

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
    window: *mut crate::Window,
    pointer: *mut c_void,
    serial: u32,
    surface: *mut c_void,
) {
}

unsafe extern "C" fn pointer_handle_motion(
    window: *mut crate::Window,
    pointer: *mut c_void,
    time: u32,
    sx: i32,
    sy: i32,
) {
}

unsafe extern "C" fn pointer_handle_button(
    window: *mut crate::Window,
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

    match button {
        BTN_LEFT => {
            // pressed.
            if state != 0 {
                println!("Press");
/*                wl_proxy_marshal(
                    (*c).toplevel,
                    5, /*ZXDG_TOPLEVEL_V6_MOVE*/
                    (*c).seat,
                    serial,
                );*/
            } else {
                println!("Release");
            }
        }
        BTN_RIGHT => {}
        BTN_MIDDLE => {}
        BTN_SIDE => {}
        _ => { eprintln!("Unknown") },
    }
}

unsafe extern "C" fn pointer_handle_axis(
    window: *mut crate::Window,
    pointer: *mut c_void,
    time: u32,
    axis: u32,
    value: i32,
) {
}

unsafe extern "C" fn configure_callback(
    c: *mut crate::Window,
    callback: *mut c_void,
    time: u32,
) {
    wl_proxy_destroy(callback);

    glViewport(
        0,
        0,
        (*((*c).nwin_c as *mut WaylandWindow)).window_width,
        (*((*c).nwin_c as *mut WaylandWindow)).window_height
    );

    if (*((*c).nwin_c as *mut WaylandWindow)).callback.is_null() {
        redraw(c, std::ptr::null_mut(), time);
    }
}

unsafe extern "C" fn handle_xdg_shell_ping(
    data: *mut c_void,
    shell: *mut c_void,
    serial: u32,
) {
    extern "C" {
        fn wl_proxy_marshal(
            p: *mut c_void,
            opcode: u32,
            b: u32,
        ) -> ();
    }

	wl_proxy_marshal(shell, 3 /*ZXDG_SHELL_V6_PONG*/, serial);
}

static mut XDG_SHELL_LISTENER: [*mut c_void; 1] = [
    handle_xdg_shell_ping as *mut _,
];

static mut CONFIGURE_CALLBACK_LISTENER: [*mut c_void; 1] = [
    configure_callback as *mut _,
];

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
    window: *mut crate::Window,
    keyboard: *mut c_void,
    format: u32,
    fd: i32,
    size: u32,
) {
}

unsafe extern "C" fn keyboard_handle_enter(
    window: *mut crate::Window,
    keyboard: *mut c_void,
    serial: u32,
    surface: *mut c_void,
    keys: *mut c_void,
) {
}

unsafe extern "C" fn keyboard_handle_leave(
    window: *mut crate::Window,
    keyboard: *mut c_void,
    serial: u32,
    surface: *mut c_void,
) {
}

unsafe extern "C" fn keyboard_handle_key(
    window: *mut crate::Window,
    keyboard: *mut c_void,
    serial: u32,
    time: u32,
    key: u32,
    state: u32,
) {
    let c = (*window).nwin_c as *mut WaylandWindow;

    if key == KEY_ESC && state != 0 {
        (*c).running = 0;
    } else if key == KEY_F11 && state != 0 {
        (*c).configured = 1;

        if (*c).fullscreen {
            /*if (*c).is_restored != 0 {
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
            }*/

            // UnFullscreen
            extern "C" {
                fn wl_proxy_marshal(
                    p: *mut c_void,
                    opcode: u32,
                    //                    a: *mut c_void,
                ) -> ();
            }

            wl_proxy_marshal(
                (*c).toplevel,
                12, /*ZXDG_TOPLEVEL_V6_UNSET_FULLSCREEN*/
            );

            (*c).fullscreen = false;
        } else {
            /*extern "C" {
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
            );*/

            // Fullscreen
            extern "C" {
                fn wl_proxy_marshal(
                    p: *mut c_void,
                    opcode: u32,
                    a: *mut c_void,
                ) -> ();
            }

            wl_proxy_marshal(
                (*c).toplevel,
                11, /*ZXDG_TOPLEVEL_V6_SET_FULLSCREEN*/
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

        wl_proxy_add_listener(
            callback,
            CONFIGURE_CALLBACK_LISTENER.as_ptr(),
            window,
        );
    }
}

unsafe extern "C" fn keyboard_handle_modifiers(
    window: *mut crate::Window,
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
    window: *mut crate::Window,
    seat: *mut c_void,
    caps: WlSeatCapability,
) {
    let c = get(&mut (*window).nwin);

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
        wl_proxy_add_listener((*c).pointer, POINTER_LISTENER.as_ptr(), window);
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
        wl_proxy_add_listener((*c).keyboard, KEYBOARD_LISTENER.as_ptr(), window);
    } else if !has_keyboard && !(*c).keyboard.is_null() {
        wl_proxy_destroy((*c).keyboard);
        (*c).keyboard = std::ptr::null_mut();
    }

    println!("SEAT!");

    // Allow Touch Events
    // TODO
    /*
        let has_touch = (caps as u32 & WlSeatCapability::Touch as u32) != 0;
        if has_touch && (*c).touch.is_null() {
            (*c).touch = wl_proxy_marshal_constructor(seat, 2,
                &wl_touch_interface, std::ptr::null_mut());
            wl_proxy_add_listener((*c).touch, touch_listener.as_ptr(), c);
        } else if !has_touch && !(*c).touch.is_null() {
            wl_proxy_destroy((*c).touch);
            (*c).touch = std::ptr::null_mut();
        }
    */
}

unsafe extern "C" fn registry_handle_global(
    window: *mut crate::Window,
    registry: *mut c_void,
    name: u32,
    interface: *const c_void, // text
    version: u32,
) {
//    let c = (*window).nwin_c as *mut WaylandWindow;
    let c = get(&mut (*window).nwin);

    if strcmp(interface, b"wl_compositor\0" as *const _ as *const _) == 0 {
        let compositor = wl_proxy_marshal_constructor_versioned(
            registry,
            0, /*WL_REGISTRY_BIND*/
            &wl_compositor_interface,
            1,
            name,
            wl_compositor_interface.name,
            1,
            std::ptr::null_mut(),
        );
        (*c).compositor = compositor;
    } else if strcmp(interface, b"zxdg_shell_v6\0" as *const _ as *const _) == 0
    {
        (*c).shell = wl_proxy_marshal_constructor_versioned(
            registry,
            0, /*WL_REGISTRY_BIND*/
            &ZXDG_SHELL_V6_INTERFACE,
            1,
            name,
            ZXDG_SHELL_V6_INTERFACE.name,
            1,
            std::ptr::null_mut(),
        );

        wl_proxy_add_listener((*c).shell, XDG_SHELL_LISTENER.as_ptr(), window);
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

        wl_proxy_add_listener((*c).seat, SEAT_LISTENER.as_ptr(), window);
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
        (*c).cursor_theme = wl_cursor_theme_load(std::ptr::null_mut(), 16, (*c).shm);
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

unsafe extern "C" fn surface_configure(
    data: *mut c_void,
    zxdg_surface_v6: *mut c_void,
    serial: u32,
) {
    extern "C" {
        fn wl_proxy_marshal(p: *mut c_void, opcode: u32, serial: u32) -> ();
    }

    // ZXDG_SURFACE_V6_ACK_CONFIGURE
    wl_proxy_marshal(zxdg_surface_v6, 4, serial);
}

unsafe extern "C" fn toplevel_configure(
    window: *mut crate::Window,
    zxdg_toplevel_v6: *mut c_void,
    width: i32,
    height: i32,
    _states: *mut c_void,
) {
    let c = get(&mut (*window).nwin);

    if !(*c).egl_window.is_null() && (*c).configured != 0 {
        wl_egl_window_resize((*c).egl_window, width, height, 0, 0);
        (*c).configured = 0;
        (*c).window_width = width;
        (*c).window_height = height;
    } else {
        if (*c).fullscreen {
        } else if width != 0 && height != 0 {
            if (*c).is_restored != 0 {
                (*c).restore_width = (*c).window_width;
                (*c).restore_height = (*c).window_height;
            }
            (*c).is_restored = 0;
            if !(*c).egl_window.is_null() {
                wl_egl_window_resize((*c).egl_window, width, height, 0, 0);
            }
            (*c).window_width = width;
            (*c).window_height = height;
        } else {
            (*c).window_width = (*c).restore_width;
            (*c).window_height = (*c).restore_height;
            (*c).is_restored = 1;
            if !(*c).egl_window.is_null() {
                wl_egl_window_resize(
                    (*c).egl_window,
                    (*c).restore_width,
                    (*c).restore_height,
                    0,
                    0,
                );
            }
        }
        glViewport(0, 0, (*c).window_width, (*c).window_height);
    }
}

unsafe extern "C" fn toplevel_close(
    window: *mut crate::Window,
    zxdg_toplevel_v6: *mut c_void,
) {
    let c = get(&mut (*window).nwin);

    (*c).running = 0;
}

pub(super) static mut SEAT_LISTENER: [*mut c_void; 2] =
    [seat_handle_capabilities as *mut _, std::ptr::null_mut()];

pub(super) static mut REGISTRY_LISTENER: [*mut c_void; 2] = [
    registry_handle_global as *mut _,
    registry_handle_global_remove as *mut _,
];

static mut SURFACE_LISTENER: [*mut c_void; 1] = [surface_configure as *mut _];

static mut TOPLEVEL_LISTENER: [*mut c_void; 2] =
    [toplevel_configure as *mut _, toplevel_close as *mut _];

#[repr(C)]
pub struct WaylandWindow {
    // Is program still running?
    pub(super) running: i32,
    // Is program restored (not fullscreen)?
    pub(super) is_restored: i32,

    // Current window width.
    pub(super) window_width: i32,
    // Current window height.
    pub(super) window_height: i32,

    // Window width to restore (exit fullscreen) to.
    pub(super) restore_width: i32,
    // Window height to restore (exit fullscreen) to.
    pub(super) restore_height: i32,

    // Millisecond counter on last frame.
    pub(super) last_millis: u32,

    pub(super) gl_rotation_uniform: u32,
    pub(super) gl_pos: u32,
    pub(super) gl_col: u32,

    // NULL if not using EGL (NULL when using Vulkan + Wayland).
    pub(super) egl_window: *mut c_void, // wl_egl_window*
    pub(super) surface: *mut c_void,    // wl_surface*
    pub(super) shell_surface: *mut c_void, // wl_shell_surface*

//    pub(super) egl_surface: *mut c_void, // EGLSurface
    pub(super) callback: *mut c_void,    // wl_callback*
    pub(super) configured: i32,
    pub(super) fullscreen: bool,

    pub(super) wldisplay: *mut c_void, // wl_display*
    pub(super) registry: *mut c_void,  // wl_registry*
    pub(super) compositor: *mut c_void, // wl_compositor*
    pub(super) shell: *mut c_void,     // wl_shell*
    pub(super) seat: *mut c_void,      // wl_seat*
    pub(super) pointer: *mut c_void,   // wl_pointer*
    pub(super) keyboard: *mut c_void,  // wl_keyboard*
    pub(super) shm: *mut c_void,       // wl_shm*
    pub(super) cursor_theme: *mut c_void, // wl_cursor_theme*
    pub(super) default_cursor: *mut WlCursor, // wl_cursor*
    pub(super) cursor_surface: *mut c_void, // wl_surface*
    pub(super) toplevel: *mut c_void,  // void*

//    pub(super) egl_dpy: *mut c_void, // EGLDisplay
//    pub(super) egl_ctx: *mut c_void, // EGLContext
//    pub(super) egl_conf: *mut c_void, // EGLConfig
}

impl Drop for WaylandWindow {
    fn drop(&mut self) {
        extern "C" {
            fn wl_proxy_marshal(
                p: *mut c_void,
                opcode: u32,
            ) -> ();
        }

        unsafe {
            // 
	        wl_surface_destroy(self.surface);
	        wl_egl_window_destroy(self.egl_window);

            // Free
	        wl_proxy_marshal(self.shell_surface, 0);
            wl_proxy_destroy(self.shell_surface);

	        if !self.callback.is_null() {
		        wl_proxy_destroy(self.callback);
            }

            // ---
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
        println!("Drop Widnow");
    }
}

impl Nwin for WaylandWindow {
    fn handle(&self) -> crate::NwinHandle {
        crate::NwinHandle::Wayland(self.wldisplay)
    }

    fn connect(&mut self, draw: &mut Box<crate::Draw>) {
        match draw.handle() {
            crate::DrawHandle::Gl(c) => {
                self.egl_window = unsafe {
                    wl_egl_window_create(
                        self.surface,
                        self.window_width,
                        self.window_height,
                    )
                };
            }
            crate::DrawHandle::Vulkan(_c) => unimplemented!(),
        }
        draw.connect(self.egl_window);
    }
}

pub(super) fn new(window: &mut crate::Window) -> Option<()> {
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

    unsafe {
        std::ptr::write(
            &mut window.nwin,
            Box::new(WaylandWindow {
                running: 1,
                is_restored: 0,

                window_width: 640,
                window_height: 360,

                restore_width: 640,
                restore_height: 360,

                last_millis: 0,

                gl_rotation_uniform: 0,
                gl_pos: 0,
                gl_col: 0,

                egl_window: std::ptr::null_mut(), // wl_egl_window*
                surface: std::ptr::null_mut(),    // wl_surface*
                shell_surface: std::ptr::null_mut(), // wl_shell_surface*

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
            })
        )
    };

    let nwin = get(&mut window.nwin);

    unsafe {
        wl_proxy_add_listener(
            (*nwin).registry,
            REGISTRY_LISTENER.as_ptr(),
            window,
        );

        wl_display_dispatch((*nwin).wldisplay);
    }

    unsafe {
        (*nwin).surface = wl_proxy_marshal_constructor(
            (*nwin).compositor,
            0,
            &wl_surface_interface,
            std::ptr::null_mut(),
        );
        (*nwin).cursor_surface = wl_proxy_marshal_constructor(
            (*nwin).compositor,
            0,
            &wl_surface_interface,
            std::ptr::null_mut(),
        );
    }

    // Create shell_surface
    unsafe {
        extern "C" {
            pub(super) fn wl_proxy_marshal_constructor(
                name: *mut c_void,
                opcode: u32,
                interface: &WlInterface,
                p: *mut c_void,
                s: *mut c_void,
            ) -> *mut c_void;
        }

        (*nwin).shell_surface = wl_proxy_marshal_constructor(
            (*nwin).shell,
            2,
            &ZXDG_SURFACE_V6_INTERFACE,
            std::ptr::null_mut(),
            (*nwin).surface,
        );

        wl_proxy_add_listener(
            (*nwin).shell_surface,
            SURFACE_LISTENER.as_ptr(),
            window
        );
    }

    // Create toplevel
    unsafe {
        (*nwin).toplevel = wl_proxy_marshal_constructor(
            (*nwin).shell_surface,
            1,
            &ZXDG_TOPLEVEL_V6_INTERFACE,
            std::ptr::null_mut(),
        );

        wl_proxy_add_listener(
            (*nwin).toplevel,
            TOPLEVEL_LISTENER.as_ptr(),
            window
        );
    }

    let window_title = "Cala Window 🙂️\0";

    // Set window title.
    unsafe {
        extern "C" {
            fn wl_proxy_marshal(
                p: *mut c_void,
                opcode: u32,
                a: *const c_void,
            ) -> ();
        }

        // Set Window Title.
        wl_proxy_marshal((*nwin).toplevel, 2, window_title.as_ptr() as *const _);
        // Set App Title.
        wl_proxy_marshal((*nwin).toplevel, 3, window_title.as_ptr() as *const _);
    }

    // Maximize window.
    unsafe {
        extern "C" {
            fn wl_proxy_marshal(
                p: *mut c_void,
                opcode: u32,
            ) -> ();
        }

        // Set Maximized.
        wl_proxy_marshal((*nwin).toplevel, 9);
    }

    // Show window.
    unsafe {
        let callback = wl_proxy_marshal_constructor(
            (*nwin).wldisplay,
            0, /*WL_DISPLAY_SYNC*/
            &wl_callback_interface,
            std::ptr::null_mut(),
        );

        wl_proxy_add_listener(
            callback,
            CONFIGURE_CALLBACK_LISTENER.as_ptr(),
            window,
        );
    }

    Some(())
}
