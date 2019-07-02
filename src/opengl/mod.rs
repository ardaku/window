use std::ffi::c_void;

use super::Draw;
use super::DrawHandle;
use super::Window;
use crate::Nshader;
use crate::Nshape;
use crate::Nvertices;

mod platform;

// Position
const GL_ATTRIB_POS: u32 = 0;
// Color
const GL_ATTRIB_COL: u32 = 1;
// Texture Coordinates Begin (May have multiple)
const GL_ATTRIB_TEX: u32 = 2;

extern "C" {
    fn eglGetDisplay(
        native_display: self::platform::NativeDisplayType,
    ) -> *mut c_void;
    fn eglInitialize(dpy: *mut c_void, major: *mut i32, minor: *mut i32)
        -> u32;
    fn eglBindAPI(api: u32) -> u32;
    fn eglChooseConfig(
        dpy: *mut c_void,
        attrib_list: *const i32,
        configs: *mut *mut c_void,
        config_size: i32,
        num_config: &mut i32,
    ) -> u32;
    fn eglCreateContext(
        dpy: *mut c_void,
        config: *mut c_void,
        share_context: *mut c_void,
        attrib_list: *const i32,
    ) -> *mut c_void;
    fn eglCreateWindowSurface(
        dpy: *mut c_void,
        config: *mut c_void,
        win: usize, // EGLNativeWindowType
        attrib_list: *const i32,
    ) -> *mut c_void;
    fn eglMakeCurrent(
        dpy: *mut c_void,
        draw: *mut c_void,
        read: *mut c_void,
        ctx: *mut c_void,
    ) -> u32;
    fn eglTerminate(dpy: *mut c_void) -> u32;
    fn eglReleaseThread() -> u32;
    fn eglSwapBuffers(dpy: *mut c_void, surface: *mut c_void) -> u32;

    // OpenGL
    fn glCreateProgram() -> u32;
    fn glAttachShader(program: u32, shader: u32) -> ();
    fn glLinkProgram(program: u32) -> ();
    fn glGetProgramiv(program: u32, pname: u32, params: *mut i32) -> ();
    fn glGetProgramInfoLog(
        program: u32,
        max_len: i32,
        length: *mut i32,
        info_log: *mut i8,
    ) -> ();
    fn glUseProgram(program: u32) -> ();
    fn glBindAttribLocation(program: u32, index: u32, name: *const i8) -> ();
    fn glGetUniformLocation(program: u32, name: *const i8) -> i32;
    fn glCreateShader(shader_type: u32) -> u32;
    fn glShaderSource(
        shader: u32,
        count: i32,
        string: *const *const i8,
        length: *const i32,
    ) -> ();
    fn glCompileShader(shader: u32) -> ();
    fn glGetShaderiv(shader: u32, pname: u32, params: *mut i32) -> ();
    fn glGetShaderInfoLog(
        shader: u32,
        max_length: i32,
        length: *mut i32,
        infoLog: *mut i8,
    ) -> ();
    //
    fn glUniformMatrix4fv(
        location: i32,
        count: i32,
        transpose: u8,
        value: *const f32,
    ) -> ();
    fn glClearColor(red: f32, green: f32, blue: f32, alpha: f32) -> ();
    fn glClear(mask: u32) -> ();
    fn glVertexAttribPointer(
        indx: u32,
        size: i32,
        stype: u32,
        normalized: u32,
        stride: i32,
        ptr: *const f32,
    ) -> ();
    fn glEnableVertexAttribArray(index: u32) -> ();
    fn glDrawArrays(mode: u32, first: i32, count: i32);
    fn glDisableVertexAttribArray(index: u32) -> ();
    fn glGenBuffers(n: i32, buffers: *mut u32) -> ();
    fn glBindBuffer(target: u32, buffer: u32) -> ();
    fn glBufferData(target: u32, size: usize, data: *const c_void, usage: u32)
        -> ();
}

/// A shader.  Shaders are a program that runs on the GPU to render a `Shape`.
pub struct Shader {
    // An OpenGL shader program ID.
    program: u32,
    // True if OpenGL color vertex attribute exists.
    gradient: bool,
    // TODO
    groups: Vec<u32>,
    // TODO
    transforms: Vec<i32>,
    // True if 3D.
    depth: bool,
}

/// A list of vertices.
pub struct Vertices {
    vbo: u32,
    dim: u8,
    gradient: u8,
    graphic_coords: u8,
}

impl Vertices {
    /// Create a new `VertexList`.  `dim`: 0 or 2~4 dimensions.  `gradient` is 3(RGB) or 4(RGBA).  `graphic_coords` is how many graphics need coordinates.
    pub fn new(vertices: &[f32], dim: u8, gradient: u8, graphic_coords: u8) -> Vertices
    {
        Vertices {
            vbo: create_vbo(vertices),
            dim, gradient, graphic_coords,
        }
    }
}

/// A shape.  Shapes are a list of indices into `Vertices`.
pub struct Shape {
    indices: Vec<u16>,
}

impl Shape {
    pub fn new(indices: &[u16]) -> Shape {
        Shape {
            indices: indices.to_vec() // TODO: use vec??
        }
    }
}

impl Shader {
    pub fn new(builder: crate::ShaderBuilder) -> Self {
        create_program(builder)
    }
}

impl Nshader for Shader {
    fn draw(&mut self) {
    }
}

impl Nshape for Shape {}
impl Nvertices for Vertices {}

pub struct OpenGL {
    surface: *mut c_void,
    display: *mut c_void,
    context: *mut c_void,
    config: *mut c_void,
}

impl Drop for OpenGL {
    fn drop(&mut self) {
        unsafe {
            eglMakeCurrent(
                self.display,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
            );
            eglTerminate(self.display);
            eglReleaseThread();
        }
    }
}

impl Draw for OpenGL {
    fn handle(&self) -> DrawHandle {
        // TODO
        DrawHandle::Gl(std::ptr::null_mut())
    }

    fn connect(&mut self, connection: *mut c_void) {
        // Finish connecting EGL.
        self.surface = unsafe {
            eglCreateWindowSurface(
                self.display,
                self.config,
                std::mem::transmute(connection),
                std::ptr::null(),
            )
        };
        let ret = unsafe {
            eglMakeCurrent(
                self.display,
                self.surface,
                self.surface,
                self.context,
            )
        };
        debug_assert_ne!(ret, 0);

        // Configuration (TODO)
        // glEnable(GL_CULL_FACES);

        // Set default background for OpenGL.
        self.background(0.0, 0.0, 1.0);
    }

    fn background(&mut self, r: f32, g: f32, b: f32) {
        unsafe {
            glClearColor(r, g, b, 0.5); // TODO ?
        } 
    }

    fn shader_new(&mut self, builder: crate::ShaderBuilder) -> Box<Nshader> {
        Box::new(Shader::new(builder))
    }

    fn vertices_new(&mut self, vertices: &[f32], dim: u8, gradient: u8, graphic_coords: u8) -> Box<Nvertices> {
        Box::new(Vertices::new(vertices, dim, gradient, graphic_coords))
    }

    fn shape_new(&mut self, indices: &[u16]) -> Box<Nshape> {
        Box::new(Shape::new(indices))
    }

    fn begin_draw(&mut self) {
        unsafe {
            glClear(0x00004000 /*GL_COLOR_BUFFER_BIT*/);
        }
    }

    fn finish_draw(&mut self) {
        unsafe {
            eglSwapBuffers(self.display, self.surface);
        }
    }

    fn test(&mut self) {
        // let timer = 0; // TODO with nanos
        // let angle = (timer % 360) as f32 * std::f32::consts::PI / 180.0;

        unsafe {
            /*#[rustfmt::skip]
            let verts = [
                -0.5, -0.5,
                0.5,  -0.5,
                0.0,   0.5,
            ];
            #[rustfmt::skip]
            let colors = [
                1.0, 0.0, 0.0,
                0.0, 1.0, 0.0,
                0.0, 0.0, 0.0,
            ];
            #[rustfmt::skip]
            let rotation = [
                angle.cos(), 0.0, angle.sin(), 0.0,
                0.0, 1.0, 0.0, 0.0,
                -angle.sin(), 0.0, angle.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0,
            ];

            glUniformMatrix4fv(
                self.gl_rotation_uniform,
                1,
                0, /*GL_FALSE*/
                rotation.as_ptr(),
            );

            glVertexAttribPointer(
                GL_ATTRIB_POS,
                2,
                0x1406, /*GL_FLOAT*/
                0x1406, /*GL_FLOAT*/
                0,
                verts.as_ptr(),
            );
            glVertexAttribPointer(
                GL_ATTRIB_COL,
                3,
                0x1406, /*GL_FLOAT*/
                0,      /*GL_FALSE*/
                0,
                colors.as_ptr(),
            );
            glEnableVertexAttribArray(GL_ATTRIB_POS);
            glEnableVertexAttribArray(GL_ATTRIB_COL);

            glDrawArrays(0x0004 /*GL_TRIANGLES*/, 0, 3);

            glDisableVertexAttribArray(GL_ATTRIB_POS);
            glDisableVertexAttribArray(GL_ATTRIB_COL);*/
        }
    }
}

// Create an OpenGL vertex buffer object.
fn create_vbo(vertices: &[f32]) -> u32 {
    unsafe {
        let mut buffers = [std::mem::uninitialized()];
        glGenBuffers(1 /*1 buffer*/, buffers.as_mut_ptr());
        glBindBuffer(0x8892 /*GL_ARRAY_BUFFER*/, buffers[0]);
        // TODO: maybe use glMapBuffer & glUnmapBuffer instead?
        glBufferData(
            0x8892 /*GL_ARRAY_BUFFER*/,
            vertices.len() * std::mem::size_of::<f32>(),
            vertices.as_ptr() as *const _,
            0x88E4 /*GL_STATIC_DRAW - never changes */,
        );
//        glVertexAttribPointer(position as u32, 2, 0x1406 /*GL_FLOAT*/, 0, 0, std::ptr::null());
        buffers[0]
    }
}

/// Create a shader program.
fn create_program(builder: crate::ShaderBuilder) -> Shader {
    // Convert a number to text.
    fn num_to_text(l: u8) -> [u8; 2] {
        if l >= 128 {
            panic!("Number too high");
        }

        let a = (l >> 4) + b'a';
        let b = (l << 4) + b'a';

        [a, b]
    }

    let frag = create_shader(
        builder.opengl_frag.as_ptr() as *const _ as *const _,
        0x8B30, /*GL_FRAGMENT_SHADER*/
    );
    let vert = create_shader(
        builder.opengl_vert.as_ptr() as *const _ as *const _,
        0x8B31, /*GL_VERTEX_SHADER*/
    );
    let program = unsafe { glCreateProgram() };
    unsafe {
        glAttachShader(program, frag);
        glAttachShader(program, vert);
        glLinkProgram(program);
    }
    let mut status = unsafe { std::mem::uninitialized() };
    unsafe {
        glGetProgramiv(program, 0x8B82, /*GL_LINK_STATUS*/ &mut status);
    }
    if status == 0 {
        let mut log = [0u8; 1000];
        let mut len = unsafe { std::mem::uninitialized() };
        unsafe {
            glGetProgramInfoLog(
                program,
                1000,
                &mut len,
                log.as_mut_ptr() as *mut _ as *mut _,
            );
        }
        let log = String::from_utf8_lossy(&log);
        panic!("Error: linking:\n{}", log);
    }
    // Bind the shader program.
    unsafe {
        glUseProgram(program);
    }
    // Vertex attributes
    unsafe {
        // All shader programs have position.
        glBindAttribLocation(program, GL_ATTRIB_POS, b"pos\0".as_ptr() as *const _ as *const _);
        // 
        if builder.gradient {
            glBindAttribLocation(
                program,
                GL_ATTRIB_COL,
                b"col\0".as_ptr() as *const _ as *const _,
            );
        }
        glLinkProgram(program);
    }
    // Uniforms
    let mut groups = Vec::with_capacity(builder.group as usize);
    let mut transforms = Vec::with_capacity(builder.transform as usize);
    /*// 
    for group in builder.groups.iter() {
        
    }*/

    for transform in 0..builder.transform {
        let ntt = num_to_text(transform);
        let ntt = [ntt[0] as char, ntt[1] as char];
        let id = format!("transform_{}{}\0", ntt[0], ntt[1]);
        let handle = unsafe {
            glGetUniformLocation(program, id.as_ptr() as *const _ as *const _)
        };
        assert!(handle > -1);
        transforms.push(handle);
    }

/*    // Get Vertex Attributes
    let position = unsafe { (opengl.vdata)(program, b"position\0".as_ptr() as *const _) };
    let texpos = unsafe { (opengl.vdata)(program, b"texpos\0".as_ptr() as *const _) };
    // Enable Vertex Attributes
    unsafe {
        (opengl.enable_vdata)(position as u32);
        (opengl.enable_vdata)(texpos as u32);
    }*/

    Shader {
        program, gradient: builder.gradient, groups, transforms, depth: builder.depth
    }
}

fn create_shader(source: *const i8, shader_type: u32) -> u32 {
    let shader = unsafe { glCreateShader(shader_type) };
    debug_assert!(shader != 0);

    unsafe {
        glShaderSource(shader, 1, [source].as_ptr(), std::ptr::null());
        glCompileShader(shader);
    }

    let mut status = unsafe { std::mem::uninitialized() };
    unsafe {
        glGetShaderiv(shader, 0x8B81 /*GL_COMPILE_STATUS*/, &mut status);
    }
    if status == 0 {
        let mut log = [0u8; 1000];
        let mut len = unsafe { std::mem::uninitialized() };
        unsafe {
            glGetShaderInfoLog(
                shader,
                1000,
                &mut len,
                log.as_mut_ptr() as *mut _ as *mut _,
            );
        }
        let log = String::from_utf8_lossy(&log);
        panic!(
            "Error: compiling {}: {}\n",
            if shader_type == 0x8B31
            /*GL_VERTEX_SHADER*/
            {
                "vertex"
            } else {
                "fragment"
            },
            log
        );
    }

    return shader;
}

#[cfg(unix)]
pub(super) fn new(window: &mut Window) -> Option<Box<Draw>> {
    let (display, config, context) = unsafe {
        // Get EGL Display from Window.
        let display = eglGetDisplay(match window.nwin.handle() {
            #[cfg(not(any(
                target_os = "android",
                target_os = "macos",
                target_os = "ios"
            )))]
            crate::NwinHandle::Wayland(handle) => handle,
        });
        debug_assert!(!display.is_null());

        // Initialize EGL Display.
        let mut major = std::mem::uninitialized();
        let mut minor = std::mem::uninitialized();
        let ret = eglInitialize(display, &mut major, &mut minor);
        debug_assert_eq!(ret, 1);

        // Connect EGL to either OpenGL or OpenGLES, whichever is available.
        // TODO: also support /*OPENGL:*/ 0x30A2
        let ret = eglBindAPI(/*OPENGL_ES:*/ 0x30A0);
        debug_assert_eq!(ret, 1);

        //
        let mut config: *mut c_void = std::mem::uninitialized();
        let mut n: i32 = std::mem::uninitialized();
        let ret = eglChooseConfig(
            display,
            [
                /*EGL_SURFACE_TYPE:*/ 0x3033,
                /*EGL_WINDOW_BIT:*/ 0x04, /*EGL_RED_SIZE:*/ 0x3024,
                8, /*EGL_GREEN_SIZE:*/ 0x3023, 8,
                /*EGL_BLUE_SIZE:*/ 0x3022, 8,
                /*EGL_RENDERABLE_TYPE:*/ 0x3040,
                /*EGL_OPENGL_ES2_BIT:*/ 0x0004, /*EGL_NONE:*/ 0x3038,
            ]
            .as_ptr(),
            &mut config,
            1,
            &mut n,
        );
        debug_assert_eq!(ret, 1);

        //
        let context = eglCreateContext(
            display,
            config,
            std::ptr::null_mut(),
            [
                /*EGL_CONTEXT_CLIENT_VERSION:*/ 0x3098, 2,
                /*EGL_NONE:*/ 0x3038,
            ]
            .as_ptr(),
        );
        debug_assert!(!context.is_null());

        (display, config, context)
    };

    let draw: OpenGL = OpenGL {
        display,
        config,
        context,
        surface: std::ptr::null_mut(),
    };

    Some(Box::new(draw))
}
