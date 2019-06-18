use std::ffi::c_void;

#[cfg(unix)]
mod wayland;

mod opengl;

use self::opengl::*;
#[cfg(unix)]
use self::wayland::*;

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
    fn connect(&self, draw: DrawHandle);
}

trait Draw {
    // Get a pointer that refers to this graphics context for interfacing.
    fn handle(&self) -> DrawHandle;
}

/// A window on the monitor.
#[repr(C)]
pub struct Window {
    nwin: Box<Nwin>,
    draw: Box<Draw>,
}

/// Start the Wayland + OpenGL application.
pub fn start() -> Option<Window> {
    /*********************/
    /* Declare Variables */
    /*********************/
    let mut window: Window = unsafe { std::mem::zeroed() };

    /*********************/
    /* Create The Window */
    /*********************/

    let mut win = None;

    // Try to initialize Wayland first.
    #[cfg(unix)]
    {
        win = win.or_else(wayland::new);
    }

    // Hopefully we found one of the backends.
    unsafe {
        std::ptr::write(
            &mut window.nwin,
            win.or_else(|| {
                eprintln!("Couldn't find a window manager.");
                return None;
            })?,
        );
    }

    /*********************/
    /* Connect Rendering */
    /*********************/

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
                eprintln!("Couldn't find a graphics API.");
                return None;
            })?,
        );
    }

    /*********************/
    /* Enter Render Loop */
    /*********************/

    unsafe {
        wayland::dive_wayland(
            (*std::mem::transmute::<&Box<_>, &[*mut wayland::WaylandWindow; 2]>(
                &window.nwin,
            ))[0],
        );
    }

    Some(window)
}
