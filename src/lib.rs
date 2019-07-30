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

/// **video** Load a generated shader from `res`.
#[macro_export(self)] macro_rules! shader {
    ($shadername: literal) => {
        include!(concat!(env!("OUT_DIR"), "/res/", $shadername, ".rs"));
    }
}

/// A transformation matrix.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Transform {
    mat: [[f32;4];4],
}

impl Default for Transform {
    fn default() -> Self {
        Self::new()
    }
}

impl Transform {
    /// Create a new identity matrix (transform that does nothing).
    pub fn new() -> Self {
        Self {
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

    /// Rotate transformation.  Parameters are quaternion in axis-angle form.
    /// - `x`: axis-vector x.
    /// - `y`: axis-vector y.
    /// - `z`: axis-vector z.
    /// - `c`: angle in cycles.
    pub /*const*/ fn rotate(self, x: f32, y: f32, z: f32, c: f32) -> Self {
        // Step 1. Normalize xyz rotation vector.
        let length = ((x * x) + (y * y) + (z * z)).sqrt();
        let (x, y, z) = (x / length, y / length, z / length);

        // Step 2. Get quaternion vector.
        let angle = c * std::f32::consts::PI;
        let scalar = angle.sin();
        let (x, y, z) = (x * scalar, y * scalar, z * scalar);

        // Step 3. Get quaternion scalar.
        let scalar = angle.cos();

        // Step 4. Convert quaternion into matrix.
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let xx2 = x2 * x;
        let xy2 = x2 * y;
        let xz2 = x2 * z;

        let yy2 = y2 * y;
        let yz2 = y2 * z;
        let zz2 = z2 * z;

        let sy2 = y2 * scalar;
        let sz2 = z2 * scalar;
        let sx2 = x2 * scalar;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Self {
            mat: [
                [1.0 - yy2 - zz2, xy2 + sz2, xz2 - sy2, 0.0],
                [xy2 - sz2, 1.0 - xx2 - zz2, yz2 + sx2, 0.0],
                [xz2 + sy2, yz2 - sx2, 1.0 - xx2 - yy2, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ]
        }
    }

    /// Create a perspective matrix.
    /// - `fovy` - Y dimension field of view (in cycles), 0.25 is standard
    ///   domain: 0 < fovy < 0.5
    /// - `aspect` - `screen.width / screen.height`
    /// - `near` - Near clipping pane, domain: 0 < near
    /// - `far` - Far clipping pane, domain: near < far
    pub fn perspective(fovy: f32, aspect: f32, near: f32, far: f32) -> Self {
        let f = 1.0 / (fovy * std::f32::consts::PI).tan();
        let s = (f * aspect, f);

        let zcoord_domain = near - far;
        let zscale = (far + near) / zcoord_domain; // far / zcoord_domain;
        let zwithw = (2.0 * far * near) / zcoord_domain; //far * near / zcoord_domain;

        #[cfg_attr(rustfmt, rustfmt_skip)]
        Self {
            mat: [
                [s.0, 0.0, 0.0, 0.0],
                [0.0, s.1, 0.0, 0.0],
                [0.0, 0.0, zscale, -1.0],
                [0.0, 0.0, zwithw, 0.0],
            ]
        }
    }
}

impl std::ops::Mul<Transform> for Transform {
	type Output = Transform;

	fn mul(self, rhs: Transform) -> Self::Output {
        Transform {
            mat: [
			    [(self.mat[0][0] * rhs.mat[0][0])
                + (self.mat[0][1] * rhs.mat[1][0])
                + (self.mat[0][2] * rhs.mat[2][0])
                + (self.mat[0][3] * rhs.mat[3][0]),
			    (self.mat[0][0] * rhs.mat[0][1])
                + (self.mat[0][1] * rhs.mat[1][1])
                + (self.mat[0][2] * rhs.mat[2][1])
                + (self.mat[0][3] * rhs.mat[3][1]),
			    (self.mat[0][0] * rhs.mat[0][2])
                + (self.mat[0][1] * rhs.mat[1][2])
                + (self.mat[0][2] * rhs.mat[2][2])
                + (self.mat[0][3] * rhs.mat[3][2]),
			    (self.mat[0][0] * rhs.mat[0][3])
                + (self.mat[0][1] * rhs.mat[1][3])
                + (self.mat[0][2] * rhs.mat[2][3])
                + (self.mat[0][3] * rhs.mat[3][3])],

			    [(self.mat[1][0] * rhs.mat[0][0])
                + (self.mat[1][1] * rhs.mat[1][0])
                + (self.mat[1][2] * rhs.mat[2][0])
                + (self.mat[1][3] * rhs.mat[3][0]),
			    (self.mat[1][0] * rhs.mat[0][1])
                + (self.mat[1][1] * rhs.mat[1][1])
                + (self.mat[1][2] * rhs.mat[2][1])
                + (self.mat[1][3] * rhs.mat[3][1]),
			    (self.mat[1][0] * rhs.mat[0][2])
                + (self.mat[1][1] * rhs.mat[1][2])
                + (self.mat[1][2] * rhs.mat[2][2])
                + (self.mat[1][3] * rhs.mat[3][2]),
			    (self.mat[1][0] * rhs.mat[0][3])
                + (self.mat[1][1] * rhs.mat[1][3])
                + (self.mat[1][2] * rhs.mat[2][3])
                + (self.mat[1][3] * rhs.mat[3][3])],

			    [(self.mat[2][0] * rhs.mat[0][0])
                + (self.mat[2][1] * rhs.mat[1][0])
                + (self.mat[2][2] * rhs.mat[2][0])
                + (self.mat[2][3] * rhs.mat[3][0]),
			    (self.mat[2][0] * rhs.mat[0][1])
                + (self.mat[2][1] * rhs.mat[1][1])
                + (self.mat[2][2] * rhs.mat[2][1])
                + (self.mat[2][3] * rhs.mat[3][1]),
			    (self.mat[2][0] * rhs.mat[0][2])
                + (self.mat[2][1] * rhs.mat[1][2])
                + (self.mat[2][2] * rhs.mat[2][2])
                + (self.mat[2][3] * rhs.mat[3][2]),
			    (self.mat[2][0] * rhs.mat[0][3])
                + (self.mat[2][1] * rhs.mat[1][3])
                + (self.mat[2][2] * rhs.mat[2][3])
                + (self.mat[2][3] * rhs.mat[3][3])],

			    [(self.mat[3][0] * rhs.mat[0][0])
                + (self.mat[3][1] * rhs.mat[1][0])
                + (self.mat[3][2] * rhs.mat[2][0])
                + (self.mat[3][3] * rhs.mat[3][0]),
			    (self.mat[3][0] * rhs.mat[0][1])
                + (self.mat[3][1] * rhs.mat[1][1])
                + (self.mat[3][2] * rhs.mat[2][1])
                + (self.mat[3][3] * rhs.mat[3][1]),
			    (self.mat[3][0] * rhs.mat[0][2])
                + (self.mat[3][1] * rhs.mat[1][2])
                + (self.mat[3][2] * rhs.mat[2][2])
                + (self.mat[3][3] * rhs.mat[3][2]),
			    (self.mat[3][0] * rhs.mat[0][3])
                + (self.mat[3][1] * rhs.mat[1][3])
                + (self.mat[3][2] * rhs.mat[2][3])
                + (self.mat[3][3] * rhs.mat[3][3])],
            ],
        }
	}
}

mod keycodes;

#[cfg(unix)]
mod wayland;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
mod opengl;

pub use self::keycodes::*;

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
    /// Get the window width & height.
    fn dimensions(&mut self) -> (u16, u16);
    /// Get if a key is held down.
    fn key_held(&self, key: crate::Key) -> bool;
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
    fn instances(&mut self, shape: &mut Nshape, matrices: &[Transform]);
    /// Transform 1 instance.
    fn transform(&mut self, shape: &mut Nshape, instance: u16, transform: Transform);
    /// Upload graphic.
    fn graphic(&mut self, pixels: &[u8], width: usize) -> Box<Ngraphic>;
    /// Use a graphic.
    fn bind_graphic(&mut self, graphic: &Ngraphic);
    /// Render toolbar with width & height.
    fn toolbar(&mut self, w: u16, height: u16, toolbar_height: u16, shader: &Nshader, vertlist: &Nvertices, shape: &Nshape);
    /// Set texture coordinates
    fn texture_coords(&mut self, shader: &Nshader, coords: ([f32; 2], [f32; 2]));
    /// Set camera
    fn camera(&mut self, shader: &Nshader, cam: Transform);
}

trait Nshader {
    fn depth(&self) -> Option<i32>;
    fn gradient(&self) -> bool;
    fn graphic(&self) -> Option<(i32, i32)>;
    fn blending(&self) -> bool;
    fn bind(&self);
    fn transform(&self, index: usize) -> Option<&i32>;
    fn id(&self) -> i32;
    fn num_instances(&self) -> u16;
}

trait Nshape {
    fn len(&self) -> i32;
    fn ptr(&self) -> *const c_void;
    fn instances(&mut self, matrices: &[Transform]);
    fn transform(&mut self, index: u16, transform: Transform);
    fn instances_ptr(&self) -> *const c_void;
    fn instances_num(&self) -> i32;
}

trait Nvertices {
    fn bind(&self);
}

/// A shape.
pub struct Shape(Box<Nshape>);

trait Ngraphic {
    fn id(&self) -> u32;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn resize(&mut self, pixels: &[u8], width: usize);
    fn update(&mut self, updater: &mut FnMut(&mut [u8], u16));
}

/// A graphic on the GPU.
pub struct Graphic(Box<Ngraphic>);

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
    pub fn face(mut self, transform: Transform) -> Self {
        let dimensions = if self.shader.0.depth().is_some() { 3 } else { 2 };
        let components = if self.shader.0.blending() { 4 } else { 3 };
        let stride = dimensions
            + if self.shader.0.gradient() {
                components
            } else {
                0
            } + if self.shader.0.graphic().is_some() {
                2
            } else { 0 };
        assert!(self.vertices.len() % stride == 0);
        let mut index = 0;
        let shader1 = match self.shader.1 {
            Either::Builder(ref mut list) => list,
            Either::VertList(_) => panic!("Already built!"),
        };
        println!("BGMKN");
        // Loop through vertices.
        'v: loop {
            // Break out of loop.
            if index == self.vertices.len() {
                break 'v;
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
                transform.mat[0][0] * vertex[0]
                    + transform.mat[1][0] * vertex[1]
                    + transform.mat[2][0] * vertex[2]
                    + transform.mat[3][0],
                transform.mat[0][1] * vertex[0]
                    + transform.mat[1][1] * vertex[1]
                    + transform.mat[2][1] * vertex[2]
                    + transform.mat[3][1],
                transform.mat[0][2] * vertex[0]
                    + transform.mat[1][2] * vertex[1]
                    + transform.mat[2][2] * vertex[2]
                    + transform.mat[3][2],
            ];
            // Find index to push to index buffer.
            let mut jndex = 0;
            self.indices.push('l: loop {
                // Haven't found the vertex, add to shader's vertex list.
                if jndex == shader1.len() {
                    let rtn = jndex / stride;
                    // Push transformed coordinates
                    for k in vertex.iter().take(dimensions) {
                        shader1.push(*k)
                    }
                    // Don't transform the data.
                    for k in dimensions..stride {
                        shader1.push(self.vertices[index + k]);
                    }
                    break 'l rtn as u16;
                }

                // Test to see if vertex already exists.
                let mut equal = true;
                'b: for k in 0..dimensions {
                    if !nearly_equal(vertex[k], shader1[jndex + k]) {
                        equal = false;
                        break 'b;
                    }
                }
                'c: for k in dimensions..stride {
                    if !nearly_equal(self.vertices[index + k], shader1[jndex + k]) {
                        equal = false;
                        break 'c;
                    }
                }
/*                if equal {
                    'c: for k in 0..stride {
                        if !nearly_equal(self.vertices[index + k], shader1[jndex + k]) {
                            equal = false;
                            break 'c;
                        }
                    }
                }*/
                if equal {
                    break 'l (jndex / stride) as u16;
                }
                jndex += stride;
            });

            index += stride;
        }

        println!("{:?}", &self.indices);

        self
    }
}

/// A builder for portable shaders.
pub struct ShaderBuilder {
    /// Number of transform matrices for this shader.
    pub transform: u8,
    /// Whether or not shapes for this shader have a tint
    pub tint: bool,
    /// Whether or not vertices have attached colors for this shader.
    pub gradient: bool,
    /// Whether or not a graphic is attached to this shader.
    pub graphic: bool,
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
    toolbar_graphic: Graphic,
    toolbar_shader: Shader,
    toolbar_shape: Shape,
    toolbar_callback: fn(&mut [u8], u16),
    // Height of the toolbar.
    pub toolbar_height: u16,
    draw: Box<Draw>,
    nwin: Box<Nwin>,
    redraw: fn(nanos: u64) -> (),
}

impl Window {
    /// Start the Wayland + OpenGL application.
    pub fn new(name: &str, run: fn(nanos: u64) -> (), toolbar: fn(&mut Self) -> (Shader, Shape)) -> Box<Self> {
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

        /**********************/
        /* Initialize Toolbar */
        /**********************/

        window.toolbar_height = 48;

        let (toolbar_shader, toolbar_shape) = (toolbar)(&mut window);
        let width = window.nwin.dimensions().0;
        let pixels = vec![255; (width * window.toolbar_height) as usize * 4];
        let toolbar_graphic = window.graphic(pixels.as_slice(), width as usize);
        fn toolbar_callback(a: &mut [u8], b: u16) {}

        unsafe {
            std::ptr::write(&mut window.toolbar_shader, toolbar_shader);
            std::ptr::write(&mut window.toolbar_shape, toolbar_shape);
            std::ptr::write(&mut window.toolbar_graphic, toolbar_graphic);
            std::ptr::write(&mut window.toolbar_callback, toolbar_callback);
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
    pub fn instances(&mut self, shape: &mut Shape, transforms: &[Transform]) {
        self.draw.instances(&mut *shape.0, transforms);
    }

    /// Update transformation matrix for an instance of a shape.
    pub fn transform(&mut self, shape: &mut Shape, instance: u16, transform: Transform) {
        self.draw.transform(&mut *shape.0, instance, transform);
    }

    /// Load an RGBA graphic to the GPU.
    pub fn graphic(&mut self, pixels: &[u8], width: usize) -> Graphic {
        Graphic(self.draw.graphic(pixels, width))
    }

    /// Update RGBA graphic on the GPU.
    pub fn update_graphic(&mut self, graphic: &mut Graphic, closure: &mut FnMut(&mut [u8], u16)) {
        graphic.0.update(closure);
    }

    /// Set texture coordinates for a shader.
    pub fn camera(&mut self, shader: &Shader, cam: Transform) {
        self.draw.camera(&*shader.0, cam)
    }

    /// Set texture coordinates for a shader.
    pub fn texture_coords(&mut self, shader: &Shader, coords: ([f32; 2], [f32; 2])) {
        self.draw.texture_coords(&*shader.0, coords)
    }

    /// Use a graphic for drawing.
    pub fn draw_graphic(&mut self, shader: &Shader, shape: &Shape, graphic: &Graphic) {
        self.draw.bind_graphic(&*graphic.0);
        self.draw(shader, shape);
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

    /// Draw the toolbar.
    fn draw_toolbar(&mut self, shader: &Shader, shape: &Shape, graphic: &Graphic) {
        self.draw.bind_graphic(&*graphic.0);
        self.draw.texture_coords(&*shader.0, ([0f32, 0f32], [1f32, 1f32]));
        self.draw.toolbar(
            self.nwin.dimensions().0,
            self.nwin.dimensions().1,
            self.toolbar_height,
            &*shader.0,
            &**match shader.1 {
                Either::Builder(_) => panic!("Not built yet!"),
                Either::VertList(ref a) => a,
            },
            &*shape.0,
        );
    }

    /// Update toolbar graphic.
    pub fn toolbar(&mut self, callback: fn(&mut [u8], u16)) {
        self.toolbar_graphic.0.update(&mut |a, b| callback(a, b));
        self.toolbar_callback = callback;
    }

    /// Build a shader.
    pub fn build(&mut self, shader: &mut Shader) {
        if let Either::Builder(ref vertices) = shader.1 {
            shader.1 = Either::VertList(self.draw.vertices_new(vertices))
        } else {
            panic!("Already built!");
        }
    }

    /// If a key is being held down.
    pub fn key(&self, key: Key) -> bool {
        self.nwin.key_held(key)
    }
}
