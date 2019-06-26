//! # Window
//! Minimal Rust code for creating a window, automatically choosing a backend window manager and graphics API.
//!
//! Other Rust window creation libraries require you to build for a specific backend, so I made this crate to fix the issue.  You can now make a program that runs Wayland on a machine that has Wayland installed, and will fall back to XCB if it's not installed.  And, will run OpenGLES (eventually try Vulkan first, too) if it's installed, and fall back to OpenGL if it's not installed.
//!
//! Since this crate is minimal, it doesn't even handle window decoration.  If you want window decoration and GUI widgets, check out [barg](https://crates.io/crates/barg) which depends on this crate.

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://jeronlau.plopgrizzly.com/cala/icon.svg",
    html_favicon_url = "https://jeronlau.plopgrizzly.com/cala/icon.svg"
)]

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
    fn run(&mut self) -> bool;
}

trait Draw {
    /// Get a pointer that refers to this graphics context for interfacing.
    fn handle(&self) -> DrawHandle;
    /// Finish initializing graphics context.
    fn connect(&mut self, connection: *mut c_void);
    /// Begin draw (clear screen).
    fn begin_draw(&mut self);
    /// Redraw on the screen.
    fn finish_draw(&mut self);
    /// Change the background color.
    fn background(&mut self, r: f32, g: f32, b: f32);
    /// Test drawing.
    fn test(&mut self);
}

/// A window on the monitor.
pub struct Window {
    draw: Box<Draw>,
    nwin: Box<Nwin>,
    redraw: fn(nanos: u64) -> (),
}

impl Window {
    /// Start the Wayland + OpenGL application.
    pub fn new(name: &str, run: fn(nanos: u64) -> ()) -> Box<Self> {
        let mut window: Box<Window> = unsafe { std::mem::uninitialized() };

        /*********************/
        /* Declare Variables */
        /*********************/
        unsafe {
            std::ptr::write(&mut window, Box::new(std::mem::zeroed()));
        }

        /*********************/
        /* Create The Window */
        /*********************/

        let mut win = None;

        // Try to initialize Wayland first.
        #[cfg(unix)]
        {
            if win.is_none() {
                win = wayland::new(name, &mut window);
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

        window
    }

    /// Run the next frame in the window.
    pub fn run(&mut self) -> bool {
        self.nwin.run()
    }

    /// Change the background color.
    pub fn background(&mut self, r: f32, g: f32, b: f32) {
        self.draw.background(r, g, b)
    }

    pub fn test(&mut self) {
        self.draw.test();
    }
}
