pub struct Context {
}

fn main() {
    pub fn run(nanos: u64) {
    }

    println!("{:?}", std::thread::current().name());
    let mut window = window::Window::new("My Window", run);
    while window.run() {}
}
