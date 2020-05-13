#[derive(Copy, Clone)]
pub struct TimedLoop {
    // Incremented every `add()`.
    counter: u64,
    // `counter` wraps around
    maximum: u64,
}

impl TimedLoop {
    /// Seconds and nanoseconds (1 / 1_000_000_000 of a second).
    pub const fn new(secs: u32, nanos: u32) -> TimedLoop {
        let whol = secs as u64 * 1_000_000_000u64;
        let frac = nanos as u64;
        let value = whol + frac;

        TimedLoop {
            counter: 0,
            maximum: value,
        }
    }

    /// Add time to the `TimedLoop`.
    pub fn add(&mut self, mut nanos: u64) {
        let left = self.maximum - self.counter;
        if nanos > left {
            nanos -= left + 1;
            self.counter = nanos;
        } else {
            self.counter += nanos;
        }
    }
}

impl Into<f32> for TimedLoop {
    fn into(self) -> f32 {
        (self.counter as f64 / self.maximum as f64) as f32
    }
}

pub struct Context {
    timed: TimedLoop,
}

fn main() {
    thread_local!(static __CALA_CONTEXT: std::cell::RefCell<Context> = std::cell::RefCell::new(
        Context {
            timed: TimedLoop::new(3, 0),
        }
    ));

    pub fn run(nanos: u64) {
        let _context = __CALA_CONTEXT.with(|c| {
            c.borrow_mut().timed.add(nanos);

            let float: f32 = c.borrow_mut().timed.into();

            println!("{}", float);
        });
    }

    println!("{:?}", std::thread::current().name());
    let mut window = window::Window::new("My Window", run);
    while window.run() {}
}
