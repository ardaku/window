//! # Window
//! Minimal Rust code for creating a window, automatically choosing a backend window manager and graphics API.
//!
//! Other Rust window creation libraries require you to build for a specific backend, so I made this crate to fix the issue.  You can now make a program that runs Wayland on a machine that has Wayland installed, and will fall back to XCB if it's not installed.  And, will run OpenGLES (eventually try Vulkan first, too) if it's installed, and fall back to OpenGL if it's not installed.
//!
//! Since this crate is minimal, it doesn't even handle window decoration.  If you want window decoration and GUI widgets, check out [barg](https://crates.io/crates/barg) which depends on this crate.  And if you want more than just rendering, check out [cala](https://crates.io/crates/cala).  And, eventually, specifically for video games [plop](https://crates.io/crates/plop).

#![warn(missing_docs)]
#![doc(
    html_logo_url = "https://libcala.github.io/logo.svg",
    html_favicon_url = "https://libcala.github.io/icon.svg"
)]

use std::ffi::c_void;

/// **video** Load a generated shader from `res`.
#[macro_export(self)]
macro_rules! shader {
    ($shadername: literal) => {
        include!(concat!(env!("OUT_DIR"), "/res/", $shadername, ".rs"));
    };
}

mod keycodes;
mod mat4;
mod shape;

#[cfg(unix)]
mod wayland;

#[cfg(not(any(target_os = "macos", target_os = "ios")))]
mod opengl;

pub use self::keycodes::*;
pub use self::mat4::*;
pub use self::shape::*;

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
    fn connect(&mut self, draw: &mut Box<dyn Draw>);
    /// Get the next frame.  Return false on quit.
    fn run(&mut self) -> bool;
    /// Get the window width & height.
    fn dimensions(&self) -> (u16, u16);
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
    fn shader_new(&mut self, builder: ShaderBuilder) -> Box<dyn Nshader>;
    /// Create a shape.
    fn group_new(&mut self) -> Box<dyn Ngroup>;
    /// Draw a shape.
    fn draw(&mut self, shader: &dyn Nshader, shape: &mut dyn Ngroup);
    /// Upload graphic.
    fn graphic(
        &mut self,
        pixels: &[u8],
        width: usize,
        height: usize,
    ) -> Box<dyn Ngraphic>;
    /// Use a graphic.
    fn bind_graphic(&mut self, graphic: &dyn Ngraphic);
    /// Render toolbar with width & height.
    fn toolbar(
        &mut self,
        w: u16,
        height: u16,
        toolbar_height: u16,
        shader: &dyn Nshader,
        shape: &mut dyn Ngroup,
    );
    /// Set camera
    fn camera(&mut self, shader: &dyn Nshader, cam: Transform);
    /// Set tint
    fn tint(&mut self, shader: &dyn Nshader, tint: [f32; 4]);
}

trait Nshader {
    fn depth(&self) -> Option<i32>;
    fn tint(&self) -> Option<i32>;
    fn gradient(&self) -> bool;
    fn graphic(&self) -> bool;
    fn blending(&self) -> bool;
    fn bind(&self);
    fn program(&self) -> u32;
}

trait Ngroup {
    fn len(&self) -> i32;
    fn bind(&mut self);
    fn id(&self) -> u32;
    fn push(&mut self, shape: &crate::Shape, transform: &crate::Transform);
    fn push_tex(
        &mut self,
        shape: &crate::Shape,
        transform: &crate::Transform,
        tex_coords: ([f32; 2], [f32; 2]),
    );
}

/// A group.  Groups
pub struct Group(Box<dyn Ngroup>);

impl Group {
    /// Push a shape into the group.
    pub fn push(&mut self, shape: &crate::Shape, transform: &crate::Transform) {
        self.0.push(shape, transform);
    }

    /// Push a shape into the group.
    pub fn push_tex(
        &mut self,
        shape: &crate::Shape,
        transform: &crate::Transform,
        tex_coords: ([f32; 2], [f32; 2]),
    ) {
        self.0.push_tex(shape, transform, tex_coords);
    }
}

trait Ngraphic {
    fn id(&self) -> u32;
    fn width(&self) -> u16;
    fn height(&self) -> u16;
    fn resize(&mut self, pixels: &[u8], width: usize);
    fn update(&mut self, updater: &mut dyn FnMut(&mut [u8], u16));
}

/// A graphic on the GPU.
pub struct Graphic(Box<dyn Ngraphic>);

fn nearly_equal(a: f32, b: f32) -> bool {
    let abs_a = a.abs();
    let abs_b = b.abs();
    let diff = (a - b).abs();
    let both = abs_a + abs_b;

    if a.to_bits() == b.to_bits() {
        // shortcut, handles infinities
        true
    } else if a.to_bits() == 0
        || b.to_bits() == 0
        || (abs_a + abs_b < std::f32::MIN_POSITIVE)
    {
        // a or b is zero or both are extremely close to it
        // relative error is less meaningful here
        diff < (std::f32::EPSILON * std::f32::MIN_POSITIVE)
    } else if both < std::f32::MAX {
        // use relative error
        diff / both < std::f32::EPSILON
    } else {
        diff / std::f32::MAX < std::f32::EPSILON
    }
}

/// A shader.
pub struct Shader(Box<dyn Nshader>);

/// A builder for portable shaders.
pub struct ShaderBuilder {
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
}

/// A window on the monitor.
pub struct Window {
    // toolbar_graphic: Graphic,
    // toolbar_shader: Shader,
    // toolbar_shape: Group,
    // toolbar_callback: fn(&mut [u8], u16),
    // /// Height of the toolbar.
    // pub toolbar_height: u16,
    draw: Box<dyn Draw>,
    nwin: Box<dyn Nwin>,
}

impl Window {
    /// Start the Wayland + OpenGL application.
    pub fn new(
        name: &str,
        run: fn(nanos: u64) -> (),
    ) -> Self {
        /*********************/
        /* Create The Window */
        /*********************/

        // Hopefully find a backend.
        let mut nwin = Err("No backends built!".to_string())
            .or_else(|_| wayland::Wayland::new(name, run))
            .or_else(|e| Err(format!("Couldn't find a window manager: {}", e)))
            .unwrap();

        /**********************/
        /* Initialize Drawing */
        /**********************/

        // Try to initialize OpenGL(ES).
        let mut draw = None
            .or_else(|| opengl::OpenGL::new(&mut *nwin))
            .expect("Couldn't find a GPU library.");

        /****************************/
        /* Connect Window & Drawing */
        /****************************/

        nwin.connect(&mut draw);

        /**********************/
        /* Initialize Toolbar */
        /**********************/

        /* unsafe { (*window.as_mut_ptr()).toolbar_height = 48; }

        let (toolbar_shader, toolbar_shape) = unsafe { (toolbar)(&mut *window.as_mut_ptr()) };
        let width = unsafe { (*window.as_mut_ptr()).nwin.dimensions().0 };
        let height = unsafe { (*window.as_mut_ptr()).toolbar_height };
        let pixels = unsafe { vec![255; (width * (*window.as_mut_ptr()).toolbar_height) as usize * 4] };
        let toolbar_graphic = unsafe {
            (*window.as_mut_ptr()).graphic(pixels.as_slice(), width as usize, height as usize) };
        fn toolbar_callback(_: &mut [u8], _: u16) {}

        unsafe {
            std::ptr::write(&mut (*window.as_mut_ptr()).toolbar_shader, toolbar_shader);
            std::ptr::write(&mut (*window.as_mut_ptr()).toolbar_shape, toolbar_shape);
            std::ptr::write(&mut (*window.as_mut_ptr()).toolbar_graphic, toolbar_graphic);
            std::ptr::write(&mut (*window.as_mut_ptr()).toolbar_callback, toolbar_callback);
        }

        unsafe { std::mem::transmute(window) }*/

        Window { nwin, draw }
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
        Shader(self.draw.shader_new(builder))
    }

    /// Create a new shape.
    pub fn group_new(&mut self) -> Group {
        Group(self.draw.group_new())
    }

    /// Load an RGBA graphic to the GPU.
    pub fn graphic(
        &mut self,
        pixels: &[u8],
        width: usize,
        height: usize,
    ) -> Graphic {
        Graphic(self.draw.graphic(pixels, width, height))
    }

    /// Update RGBA graphic on the GPU.
    pub fn update_graphic(
        &mut self,
        graphic: &mut Graphic,
        closure: &mut dyn FnMut(&mut [u8], u16),
    ) {
        graphic.0.update(closure);
    }

    /// Set camera coordinates for a shader.
    pub fn camera(&mut self, shader: &Shader, cam: Transform) {
        self.draw.camera(&*shader.0, cam)
    }

    /// Set RGBA tint for a shader.
    pub fn tint(&mut self, shader: &Shader, color: [f32; 4]) {
        self.draw.tint(&*shader.0, color)
    }

    /// Use a graphic for drawing.
    pub fn draw_graphic(
        &mut self,
        shader: &Shader,
        shape: &mut Group,
        graphic: &Graphic,
    ) {
        self.draw.bind_graphic(&*graphic.0);
        self.draw(shader, shape);
    }

    /// Draw a group.
    pub fn draw(&mut self, shader: &Shader, group: &mut Group) {
        self.draw.draw(&*shader.0, &mut *group.0);
    }

    /// Draw the toolbar.
    fn draw_toolbar(
        &mut self,
        shader: &Shader,
        shape: &mut Group,
        graphic: &Graphic,
    ) {
        self.draw.bind_graphic(&*graphic.0);
        self.draw.toolbar(
            self.nwin.dimensions().0,
            self.nwin.dimensions().1,
            0, // self.toolbar_height,
            &*shader.0,
            &mut *shape.0,
        );
    }

    /// Update toolbar graphic.
    pub fn toolbar(&mut self, callback: fn(&mut [u8], u16)) {
        // self.toolbar_graphic.0.update(&mut |a, b| callback(a, b));
        // self.toolbar_callback = callback;
    }

    /// If a key is being held down.
    pub fn key(&self, key: Key) -> bool {
        self.nwin.key_held(key)
    }

    /// Get the aspect ratio: `window_height / window_width`.
    pub fn aspect(&self) -> f32 {
        let (w, h) = self.nwin.dimensions();
        let (w, h) = (f32::from(w), f32::from(h));

        h / w
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        std::process::exit(0);
    }
}
