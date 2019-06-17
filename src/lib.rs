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

trait Nwin {
    // Get a pointer that refers to this window for interfacing.
    fn handle(&self) -> NwinHandle;
}

trait Draw {
    //
    //    fn
}

/// A window on the monitor.
#[repr(C)]
pub struct Window {
    nwin: Box<Nwin>,
    draw: Box<Draw>,
}

/// Start the Wayland + OpenGL application.
pub fn start() -> Option<Window> {
    let mut window: Window = unsafe { std::mem::zeroed() };

    // // // // // //
    // // // // // //

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

    // // // // // //
    // // // // // //

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

    // // // // // //
    // // // // // //

    unsafe {
        wayland::dive_wayland(
            (*std::mem::transmute::<&Box<_>, &[*mut wayland::WaylandWindow; 2]>(
                &window.nwin,
            ))[0],
        );
    }

    Some(window)
}
