/// Start the Wayland application.
pub fn start() {
    #[link(name = "wayland-client")]
    #[link(name = "wayland-egl")]
    #[link(name = "wayland-cursor")]
    #[link(name = "EGL")]
    #[link(name = "GL")]
    #[link(name = "xkbcommon")]
    #[link(name = "m")]
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
