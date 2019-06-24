use dive_wayland;

pub fn run(nanos: u64) {
    dive_wayland::test();
}

fn main() {
    dive_wayland::start(run);
}