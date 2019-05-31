extern crate cc;

fn main() {
    cc::Build::new().file("wayland-egl.c").compile("capi");
}
