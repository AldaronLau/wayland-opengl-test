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
    /// Get the next frame.  Return false on quit.
    fn main_loop(&mut self) -> ();
}

trait Draw {
    /// Get a pointer that refers to this graphics context for interfacing.
    fn handle(&self) -> DrawHandle;
    /// Finish initializing graphics context.
    fn connect(&mut self, connection: *mut c_void);
    /// Redraw on the screen.
    fn redraw(&mut self);
    /// Test drawing.
    fn test(&mut self);
}

/// A window on the monitor.
struct Window {
    draw: Box<Draw>,
    nwin: Box<Nwin>,
    redraw: fn(nanos: u64) -> (),
}

static mut WINDOW: [u8; std::mem::size_of::<Box<Window>>()] =
    [0u8; std::mem::size_of::<Box<Window>>()];

pub fn test() {
    let window: &mut Box<Window> = unsafe { std::mem::transmute(&mut WINDOW) };

    window.draw.test();
}

/// Start the Wayland + OpenGL application.
pub fn start(run: fn(nanos: u64) -> ()) {
    let window: &mut Box<Window> = unsafe { std::mem::transmute(&mut WINDOW) };

    /*********************/
    /* Declare Variables */
    /*********************/
    unsafe {
        std::ptr::write(window, Box::new(std::mem::zeroed()));
    }

    /*********************/
    /* Create The Window */
    /*********************/

    let mut win = None;

    // Try to initialize Wayland first.
    #[cfg(unix)]
    {
        if win.is_none() {
            win = wayland::new(window);
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
        draw = draw.or_else(|| opengl::new(window));
    }

    // Hopefully we found one of the backends.
    unsafe {
        std::ptr::write(
            &mut window.draw,
            draw.or_else(|| {
                panic!("Couldn't find a graphics API.");
            })
            .unwrap(),
        );
    }

    /****************************/
    /* Connect Window & Drawing */
    /****************************/

    window.nwin.connect(&mut window.draw);

    /*******************/
    /* Set Redraw Loop */
    /*******************/

    unsafe {
        std::ptr::write(&mut window.redraw, run);
    }

    /*********************/
    /* Enter Render Loop */
    /*********************/

    window.nwin.main_loop();
}
