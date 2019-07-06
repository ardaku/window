//! # Window
//! Minimal Rust code for creating a window, automatically choosing a backend window manager and graphics API.
//!
//! Other Rust window creation libraries require you to build for a specific backend, so I made this crate to fix the issue.  You can now make a program that runs Wayland on a machine that has Wayland installed, and will fall back to XCB if it's not installed.  And, will run OpenGLES (eventually try Vulkan first, too) if it's installed, and fall back to OpenGL if it's not installed.
//!
//! Since this crate is minimal, it doesn't even handle window decoration.  If you want window decoration and GUI widgets, check out [barg](https://crates.io/crates/barg) which depends on this crate.  And if you want more than just rendering, check out [cala](https://crates.io/crates/cala).  And, eventually, specifically for video games [plop](https://crates.io/crates/plop).

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://jeronlau.plopgrizzly.com/cala/icon.svg",
    html_favicon_url = "https://jeronlau.plopgrizzly.com/cala/icon.svg"
)]

use std::ffi::c_void;

/// A transformation matrix.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Matrix {
    mat: [[f32;4];4],
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new()
    }
}

impl Matrix {
    /// Create a new identity matrix.
    pub fn new() -> Matrix {
        Matrix {
            mat: [[1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]],
        }
    }

    /// Scale transformation (make biggger or smaller).
    pub fn scale(mut self, x: f32, y: f32, z: f32) -> Self {
        self.mat[0][0] *= x;
        self.mat[1][1] *= y;
        self.mat[2][2] *= z;
        self
    }

    /// Translate (move) transformation.
    pub fn translate(mut self, x: f32, y: f32, z: f32) -> Self {
        self.mat[3][0] += x;
        self.mat[3][1] += y;
        self.mat[3][2] += z;
        self
    }
}

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

#[allow(unused)]
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
    fn connect(&mut self, draw: &mut dyn Draw);
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
    /// Draw a shape.
    fn draw(&mut self, shader: &Nshader, vertlist: &Nvertices, shape: &Nshape);
    /// Set instances for a shape.
    fn instances(&mut self, shape: &mut Nshape, matrices: &[Matrix]);
    /// Transform 1 instance.
    fn transform(&mut self, shape: &mut Nshape, instance: u16, matrix: Matrix);
}

trait Nshader {
    fn depth(&self) -> bool;
    fn gradient(&self) -> bool;
    fn blending(&self) -> bool;
    fn bind(&self);
    fn transform(&self, index: usize) -> Option<&i32>;
    fn id(&self) -> i32;
    fn num_instances(&self) -> u16;
}

trait Nshape {
    fn len(&self) -> i32;
    fn ptr(&self) -> *const c_void;
    fn instances(&mut self, matrices: &[Matrix]);
    fn transform(&mut self, index: u16, matrix: Matrix);
    fn instances_ptr(&self) -> *const c_void;
    fn instances_num(&self) -> i32;
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

fn nearly_equal(a: f32, b: f32) -> bool {
	let abs_a = a.abs();
	let abs_b = b.abs();
	let diff = (a - b).abs();
    let both = abs_a + abs_b;

	if a.to_bits() == b.to_bits() { // shortcut, handles infinities
		true
    } else if a.to_bits() == 0 || b.to_bits() == 0 || (abs_a + abs_b < std::f32::MIN_POSITIVE) {
		// a or b is zero or both are extremely close to it
		// relative error is less meaningful here
		diff < (std::f32::EPSILON * std::f32::MIN_POSITIVE)
	} else if both < std::f32::MAX { // use relative error
		diff / both < std::f32::EPSILON
	} else {
        diff / std::f32::MAX < std::f32::EPSILON
    }
}

/// A shader.
pub struct Shader(Box<Nshader>, Either);

/// A shape builder.
pub struct ShapeBuilder<'a> {
    shader: &'a mut Shader,
    indices: Vec<u16>,
    vertices: Vec<f32>,
    num_instances: u16,
}

impl<'a> ShapeBuilder<'a> {
    /// Create a new `ShapeBuilder` for a specific `Shader`.
    pub fn new(shader: &'a mut Shader) -> ShapeBuilder<'a> {
        let num_instances = shader.0.num_instances();

        ShapeBuilder {
            shader,
            indices: Vec::new(),
            vertices: Vec::new(),
            num_instances,
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
        let shader1 = match self.shader.1 {
            Either::Builder(ref mut list) => list,
            Either::VertList(_) => panic!("Already built!"),
        };
        loop {
            if index == self.vertices.len() {
                break;
            }
            // Read vertex position.
            let vertex = if dimensions == 2 {
                [self.vertices[index], self.vertices[index + 1], 0.0]
            } else {
                [
                    self.vertices[index],
                    self.vertices[index + 1],
                    self.vertices[index + 2],
                ]
            };
            // Transform vertex position.
            let vertex = [
                matrix[0][0] * vertex[0]
                    + matrix[1][0] * vertex[1]
                    + matrix[2][0] * vertex[2]
                    + matrix[3][0],
                matrix[0][1] * vertex[0]
                    + matrix[1][1] * vertex[1]
                    + matrix[2][1] * vertex[2]
                    + matrix[3][1],
                matrix[0][2] * vertex[0]
                    + matrix[1][2] * vertex[1]
                    + matrix[2][2] * vertex[2]
                    + matrix[3][2],
            ];
            // Find index
            let mut jndex = 0;
            self.indices.push('l: loop {
                //
                if jndex == shader1.len() {
                    let rtn = jndex / stride;
                    for k in vertex.iter().take(dimensions) {
                        shader1.push(*k)
                    }
                    for k in dimensions..stride {
                        shader1.push(self.vertices[index + k]);
                    }
                    break 'l rtn as u16;
                }
                //
                let mut equal = true;
                'b: for k in 0..stride {
                    if !nearly_equal(self.vertices[index + k], shader1[jndex + k]) {
                        equal = false;
                        break 'b;
                    }
                }
                if equal {
                    'c: for k in 0..stride {
                        if !nearly_equal(self.vertices[index + k], shader1[jndex + k]) {
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
    /// Number of transform matrices for this shader.
    pub transform: u8,
    /// Number of group matrices for this shader.
    pub group: u8,
    /// Whether or not shapes for this shader have a tint
    pub tint: bool,
    /// Whether or not vertices have attached colors for this shader.
    pub gradient: bool,
    /// Whether or not depth test & perspective are enabled for this shader.
    pub depth: bool,
    /// Whether or not blending is enabled for this shader.
    pub blend: bool,
    /// OpenGL/OpenGLES GLSL Fragment Shader
    pub opengl_frag: &'static str,
    /// OpenGL/OpenGLES GLSL Vertex Shader
    pub opengl_vert: &'static str,
    /// Number of instances allowed in shader.
    pub instance_count: u16,
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
        /*********************/
        /* Declare Variables */
        /*********************/

        let mut window = Box::new(unsafe { std::mem::zeroed() });

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

        window.nwin.connect(&mut *window.draw);

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

    /// Set the instances for a shape.
    pub fn instances(&mut self, shape: &mut Shape, transforms: &[Matrix]) {
        self.draw.instances(&mut *shape.0, transforms);
    }

    /// Update transformation matrix for an instance of a shape.
    pub fn transform(&mut self, shape: &mut Shape, instance: u16, transform: Matrix) {
        self.draw.transform(&mut *shape.0, instance, transform);
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
