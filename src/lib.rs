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
    /// Create a shader.
    fn shader_new(&mut self, builder: ShaderBuilder) -> Box<Nshader>;
    /// Create a collection of vertices.
    fn vertices_new(&mut self, vertices: &[f32]) -> Box<Nvertices>;
    /// Create a shape.
    fn shape_new(&mut self, builder: ShapeBuilder) -> Box<Nshape>;
    // Draw a shape.
    fn draw(&mut self, shader: &Nshader, vertlist: &Nvertices, shape: &Nshape);
}

trait Nshader {
    fn depth(&self) -> bool;
    fn gradient(&self) -> bool;
    fn blending(&self) -> bool;
    fn bind(&self);
    fn transform(&self, index: usize) -> Option<&i32>;
}

trait Nshape {
    fn len(&self) -> i32;
    fn ptr(&self) -> *const c_void;
}

trait Nvertices {
    fn bind(&self);
}

/// A shape.
pub struct Shape(Box<Nshape>);

enum Either {
    Builder(Vec<f32>),
    VertList(Box<Nvertices>),
}

/// A shader.
pub struct Shader(Box<Nshader>, Either);

/// A shape builder.
pub struct ShapeBuilder<'a> {
    shader: &'a mut Shader,
    indices: Vec<u16>,
    vertices: Vec<f32>,
}

impl<'a> ShapeBuilder<'a> {
    /// Create a new `ShapeBuilder` for a specific `Shader`.
    pub fn new(shader: &'a mut Shader) -> ShapeBuilder<'a> {
        ShapeBuilder {
            shader,
            indices: Vec::new(),
            vertices: Vec::new(),
        }
    }

    /// Set vertices for shape.
    pub fn vert(mut self, vertices: &[f32]) -> Self {
        self.vertices = vertices.to_vec();
        self
    }

    /// Add a face to the shape.
    pub fn face(mut self, matrix: [[f32; 4]; 4]) -> Self {
        let dimensions = if self.shader.0.depth() { 3 } else { 2 };
        let components = if self.shader.0.blending() { 4 } else { 3 };
        let stride = dimensions
            + if self.shader.0.gradient() {
                components
            } else {
                0
            };
        assert!(self.vertices.len() % stride == 0);
        let mut index = 0;
        let mut shader1 = match self.shader.1 {
            Either::Builder(ref mut list) => list,
            Either::VertList(_) => panic!("Already built!"),
        };
        loop {
            if index == self.vertices.len() {
                break;
            }
            // Read vertex position.
            let vertex = if dimensions == 2 {
                [self.vertices[index + 0], self.vertices[index + 1], 0.0]
            } else {
                [
                    self.vertices[index + 0],
                    self.vertices[index + 1],
                    self.vertices[index + 2],
                ]
            };
            // Transform vertex position.
            let vertex = [
                matrix[0][0] * vertex[0]
                    + matrix[0][1] * vertex[1]
                    + matrix[0][2] * vertex[2]
                    + matrix[0][3],
                matrix[1][0] * vertex[0]
                    + matrix[1][1] * vertex[1]
                    + matrix[1][2] * vertex[2]
                    + matrix[1][3],
                matrix[2][0] * vertex[0]
                    + matrix[2][1] * vertex[1]
                    + matrix[2][2] * vertex[2]
                    + matrix[2][3],
            ];
            // Find index
            let mut jndex = 0;
            self.indices.push('l: loop {
                //
                if jndex == shader1.len() {
                    let rtn = jndex / stride;
                    for k in 0..dimensions {
                        shader1.push(vertex[k])
                    }
                    for k in dimensions..stride {
                        shader1.push(self.vertices[index + k]);
                    }
                    break 'l rtn as u16;
                }
                //
                let mut equal = true;
                'b: for k in 0..stride {
                    if self.vertices[index + k] != shader1[jndex + k] {
                        equal = false;
                        break 'b;
                    }
                }
                if equal {
                    'c: for k in 0..stride {
                        if self.vertices[index + k] != shader1[jndex + k] {
                            equal = false;
                            break 'c;
                        }
                    }
                }
                if equal {
                    break 'l (jndex / stride) as u16;
                }
                jndex += stride;
            });

            index += stride;
        }
        self
    }
}

/// A builder for portable shaders.
pub struct ShaderBuilder {
    pub transform: u8,
    pub group: u8,
    pub tint: bool,
    pub gradient: bool,
    pub depth: bool,
    pub blend: bool,
    pub opengl_frag: &'static str,
    pub opengl_vert: &'static str,
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
            win = win.or_else(|| wayland::new(name, &mut window));
        }

        // Hopefully we found one of the backends.
        win.or_else(|| {
            panic!("Couldn't find a window manager.");
        });

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

    /// Build a shader program.
    pub fn shader_new(&mut self, builder: ShaderBuilder) -> Shader {
        Shader(self.draw.shader_new(builder), Either::Builder(vec![]))
    }

    /// Create a new shape.
    pub fn shape_new(&mut self, builder: ShapeBuilder) -> Shape {
        Shape(self.draw.shape_new(builder))
    }

    /// Draw a shape.
    pub fn draw(&mut self, shader: &Shader, shape: &Shape) {
        self.draw.draw(
            &*shader.0,
            &**match shader.1 {
                Either::Builder(_) => panic!("Not built yet!"),
                Either::VertList(ref a) => a,
            },
            &*shape.0,
        );
    }

    /// Build a shader.
    pub fn build(&mut self, shader: &mut Shader) {
        if let Either::Builder(ref vertices) = shader.1 {
            shader.1 = Either::VertList(self.draw.vertices_new(vertices))
        } else {
            panic!("Already built!");
        }
    }
}
