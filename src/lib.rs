use std::ffi::c_void;

#[cfg(unix)]
mod wayland;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
mod opengl;

/// Native Window Handle.
enum NwinHandle {
    /// Wayland window handle.
    #[cfg(all(
        unix,
        not(any(
            target_os = "android",
            target_os = "macos",
            target_os = "ios"
        ))
    ))]
    Wayland(*mut c_void),
}

/// Drawing Context Handle.
enum DrawHandle {
    /// EGL or WGL handle.
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    Gl(*mut c_void),
    /// Vulkan
    #[cfg(not(any(target_os = "macos", target_os = "ios")))]
    Vulkan(*mut c_void),
    /// Metal
    #[cfg(any(target_os = "macos", target_os = "ios"))]
    Metal(*mut c_void),
}

trait Nwin {
    /// Get a pointer that refers to this window for interfacing.
    fn handle(&self) -> NwinHandle;
    /// Connect window to the drawing context.
    fn connect(&mut self, draw: &mut Box<Draw>);
}

trait Draw {
    /// Get a pointer that refers to this graphics context for interfacing.
    fn handle(&self) -> DrawHandle;
    /// Finish initializing graphics context.
    fn connect(&mut self, connection: *mut c_void);
}

/// A window on the monitor.
#[repr(C)]
pub struct Window {
    nwin_c: *mut c_void,
    draw_c: *mut c_void,
    draw: Box<Draw>,
    nwin: Box<Nwin>,
}

/// Start the Wayland + OpenGL application.
pub fn start() -> Box<Window> {
    /*********************/
    /* Declare Variables */
    /*********************/
    let mut window: Box<Window> = Box::new(unsafe { std::mem::zeroed() });

    /*********************/
    /* Create The Window */
    /*********************/

    let mut win = None;

    // Try to initialize Wayland first.
    #[cfg(unix)]
    {
        if win.is_none() {
            win = wayland::new(&mut window);
        }
    }

    // Hopefully we found one of the backends.
    win.or_else(|| {
        panic!("Couldn't find a window manager.");
    });
/*    unsafe {
        std::ptr::write(
            &mut window.nwin,
            win.or_else(|| {
                panic!("Couldn't find a window manager.");
            }).unwrap(),
        );
    }*/

    /**********************/
    /* Initialize Drawing */
    /**********************/

    let mut draw = None;

    // Try to initialize OpenGL(ES).
    {
        draw = draw.or_else(|| opengl::new(&mut window));
    }

    // Hopefully we found one of the backends.
    unsafe {
        std::ptr::write(
            &mut window.draw,
            draw.or_else(|| {
                panic!("Couldn't find a graphics API.");
            }).unwrap(),
        );
    }

    /****************************/
    /* Connect Window & Drawing */
    /****************************/

    window.nwin.connect(&mut window.draw);

    /*********************/
    /* Enter Render Loop */
    /*********************/

    // Prepare for C.

    window.nwin_c = unsafe { (*std::mem::transmute::<
        &Box<_>, &[*mut c_void; 2]
    >(
        &window.nwin,
    ))[0] };

    window.draw_c = unsafe { (*std::mem::transmute::<
        &Box<_>, &[*mut c_void; 2]
    >(
        &window.draw,
    ))[0] };

    unsafe {
        wayland::dive_wayland(
            &mut *window,
        );
    }

    window
}
