use std::ffi::c_void;

use super::Draw;
use super::DrawHandle;
use super::Window;
use crate::Ngraphic;
use crate::Ngroup;
use crate::Nshader;

mod platform;

// Position
const GL_ATTRIB_POS: u32 = 0;
// Texture Coordinates Begin (May have multiple)
const GL_ATTRIB_TEX: u32 = 1;
// Color
const GL_ATTRIB_COL: u32 = 2;

const GL_RGBA: u32 = 0x1908;
const GL_TEXTURE_2D: u32 = 0x0DE1;

const GL_ARRAY_BUFFER: u32 = 0x8892;
const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;

extern "C" {
    fn glGetError() -> u32;
}

#[allow(unused)]
fn _get_error(string: &str) {
    match unsafe { glGetError() } {
        0 => println!("GL {}", string),
        0x0500 => panic!("OpenGL '{}()': Invalid Enum", string),
        0x0501 => panic!("OpenGL '{}()': Invalid Value", string),
        0x0502 => panic!("OpenGL '{}()': Invalid Operation", string),
        0x0503 => panic!("OpenGL '{}()': Invalid Stack Overflow", string),
        0x0504 => panic!("OpenGL '{}()': Invalid Stack Underflow", string),
        0x0505 => panic!("OpenGL '{}()': Invalid Out of Memory", string),
        u => panic!("OpenGL '{}()': Unknown Error {}", string, u),
    }
}

#[cfg(debug_assertions)]
macro_rules! gl_assert {
    ($x:expr) => {
        _get_error($x);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! gl_assert {
    ($x:expr) => {
        $x
    };
}

#[link(name = "EGL")]
//#[link(name = "GL")]
#[link(name = "GLESv2")]
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
        num_config: *mut i32,
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
        value: *const c_void,
    ) -> ();
    fn glUniform4f(location: i32, v0: f32, v1: f32, v2: f32, v3: f32) -> ();
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
    fn glDisable(cap: u32) -> ();
    fn glEnable(cap: u32) -> ();
    fn glEnableVertexAttribArray(index: u32) -> ();
    fn glDisableVertexAttribArray(index: u32) -> ();
    fn glDrawElements(
        mode: u32,
        count: i32,
        draw_type: u32,
        indices: *const c_void,
    ) -> ();
    fn glGenBuffers(n: i32, buffers: *mut u32) -> ();
    fn glBindBuffer(target: u32, buffer: u32) -> ();
    fn glBufferData(
        target: u32,
        size: isize,
        data: *const c_void,
        usage: u32,
    ) -> ();
    fn glBufferSubData(
        target: u32,
        offs: isize,
        size: isize,
        data: *const c_void,
    ) -> ();
    fn glDeleteBuffers(n: i32, buffers: *const u32) -> ();
    // fn glGetString(name: u32) -> *const u8;
    fn glGenTextures(n: u32, textures: *mut u32) -> ();
    fn glBindTexture(target: u32, texture: u32) -> ();
    fn glTexParameteri(target: u32, pname: u32, param: i32) -> ();
    fn glTexImage2D(
        target: u32,
        level: i32,
        internalFormat: i32,
        width: i32,
        height: i32,
        border: i32,
        format: u32,
        stype: u32,
        pixels: *const u8,
    ) -> ();
    fn glGenerateMipmap(target: u32);
    fn glViewport(x: i32, y: i32, width: i32, height: i32) -> ();
    fn glBlendFuncSeparate(a: u32, b: u32, c: u32, d: u32) -> ();
}

/// A shader.  Shaders are a program that runs on the GPU to render a `Shape`.
pub struct Shader {
    // An OpenGL shader program ID.
    program: u32,
    // True if OpenGL color vertex attribute exists.
    gradient: bool,
    // Some if OpenGL texture uniform exists.
    graphic: bool,
    // Some if 3D.
    depth: Option<i32>,
    // Some if tint.
    tint: Option<i32>,
    // True if transparency is allowed.
    blending: bool,
}

///
pub struct Graphic {
    id: u32,
    pixels: Vec<u8>,
    width: i32,
}

impl Graphic {
    pub fn new(pixels: &[u8], width: usize, height: usize) -> Self {
        debug_assert!(pixels.len() >= width * height * 4);

        let mut width = width as i32;
        let mut height = height as i32;

        let new_texture = unsafe {
            let mut new_texture = std::mem::MaybeUninit::uninit();
            glGenTextures(1, new_texture.as_mut_ptr());
            gl_assert!("glGenTextures");
            new_texture.assume_init()
        };

        unsafe {
            const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
            const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
            const GL_NEAREST: i32 = 0x2600;
            const GL_NEAREST_MIPMAP_LINEAR: i32 = 0x2702;
            // const GL_NEAREST_MIPMAP_NEAREST: i32 = 0x2700;

            glBindTexture(GL_TEXTURE_2D, new_texture);
            gl_assert!("glBindTexture");

            // Rendered smaller than texture
            glTexParameteri(
                GL_TEXTURE_2D,
                GL_TEXTURE_MIN_FILTER,
                GL_NEAREST_MIPMAP_LINEAR,
            );
            gl_assert!("glTexParameteri#1");
            // Rendered bigger than texture.
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_NEAREST);
            gl_assert!("glTexParameteri#2");

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                width,
                height,
                0,
                GL_RGBA,
                0x1401, /*GL_UNSIGNED_BYTE*/
                pixels.as_ptr() as *const _,
            );
            gl_assert!("glTexImage2D");

            // Generate Mipmaps.
            let mut mipmap_level = 0;
            let mut offset = width as usize * height as usize * 4;
            //            let mut skip = 1;
            while width > 1 && height > 1 && offset != pixels.len() {
                // 2 ^ 5
                // Divide width & height.
                width >>= 1;
                height >>= 1;
                // Increase mipmap level.
                mipmap_level += 1;

                glTexImage2D(
                    GL_TEXTURE_2D,
                    mipmap_level,
                    GL_RGBA as i32,
                    width,
                    height,
                    0,
                    GL_RGBA,
                    0x1401, /*GL_UNSIGNED_BYTE*/
                    pixels[offset..].as_ptr() as *const _,
                );
                gl_assert!("glTexImage2D");

                offset += width as usize * height as usize * 4;
            }

            glTexParameteri(
                GL_TEXTURE_2D,
                0x813D, /*GL_TEXTURE_MAX_LEVEL*/
                mipmap_level,
            );
            gl_assert!("glTexParameteri#3");
        }

        Graphic {
            id: new_texture,
            pixels: pixels.to_vec(),
            width,
        }
    }
}

/// A shape.  Shapes are a list of indices into `Vertices`.
pub struct Group {
    index_buf: u32,
    indices: Vec<u32>,
    vertex_buf: u32,
    vertices: Vec<f32>,
    dirty_vertex_size: bool,
    dirty_index_size: bool,
    dirty_data: bool,
}

impl Group {
    /// Create a new group.
    pub fn new() -> Group {
        let (index_buf, indices) = vbo_new::<u32>(GL_ELEMENT_ARRAY_BUFFER);
        let (vertex_buf, vertices) = vbo_new::<f32>(GL_ARRAY_BUFFER);

        Group {
            index_buf,
            indices,
            vertex_buf,
            vertices,
            dirty_vertex_size: false,
            dirty_index_size: false,
            dirty_data: false,
        }
    }
}

impl Ngroup for Group {
    fn len(&self) -> i32 {
        self.indices.len() as i32
    }

    fn bind(&mut self) {
        if self.dirty_data {
            if self.dirty_vertex_size {
                vbo_resize::<f32>(
                    GL_ARRAY_BUFFER,
                    self.vertex_buf,
                    &self.vertices,
                );
                self.dirty_vertex_size = false;
            } else {
                vbo_set::<f32>(
                    GL_ARRAY_BUFFER,
                    self.vertex_buf,
                    0,
                    self.vertices.len(),
                    &self.vertices,
                );
            }
            if self.dirty_index_size {
                vbo_resize::<u32>(
                    GL_ELEMENT_ARRAY_BUFFER,
                    self.index_buf,
                    &self.indices,
                );
                self.dirty_index_size = false;
            } else {
                vbo_set::<u32>(
                    GL_ELEMENT_ARRAY_BUFFER,
                    self.index_buf,
                    0,
                    self.indices.len(),
                    &self.indices,
                );
            }
            self.dirty_data = false;
        }

        debug_assert_ne!(self.index_buf, 0);
        unsafe {
            glBindBuffer(GL_ELEMENT_ARRAY_BUFFER, self.index_buf);
            gl_assert!("glBindBuffer#Element");
        }
        debug_assert_ne!(self.vertex_buf, 0);
        unsafe {
            glBindBuffer(GL_ARRAY_BUFFER, self.vertex_buf);
            gl_assert!("glBindBuffer");
        }
    }

    fn id(&self) -> u32 {
        self.index_buf
    }

    fn push(&mut self, shape: &crate::Shape, transform: &crate::Transform) {
        self.push_tex(shape, transform, ([0.0, 0.0], [1.0, 1.0]))
    }

    fn push_tex(
        &mut self,
        shape: &crate::Shape,
        transform: &crate::Transform,
        tex_coords: ([f32; 2], [f32; 2]),
    ) {
        let vertex_offset = self.vertices.len() as u32 / shape.stride;
        let initial_vertex_cap = self.vertices.capacity();
        let initial_index_cap = self.indices.capacity();

        for index in shape.indices.iter() {
            self.indices.push(index + vertex_offset);
        }
        for i in 0..(shape.vertices.len() / shape.stride as usize) {
            let offset = i * shape.stride as usize;

            let vector = *transform
                * if shape.dimensions == 3 {
                    [
                        shape.vertices[offset],
                        shape.vertices[offset + 1],
                        shape.vertices[offset + 2],
                    ]
                } else {
                    [shape.vertices[offset], shape.vertices[offset + 1], 0.0]
                };

            self.vertices.push(vector[0]);
            self.vertices.push(vector[1]);
            if shape.dimensions == 3 {
                self.vertices.push(vector[2]);
            }

            // Check to see if there is extra texture coordinate data.
            if shape.dimensions + shape.components + 2 == shape.stride {
                self.vertices.push(
                    shape.vertices[offset + shape.dimensions as usize]
                        * tex_coords.1[0]
                        + tex_coords.0[0],
                );
                self.vertices.push(
                    shape.vertices[offset + shape.dimensions as usize + 1]
                        * tex_coords.1[1]
                        + tex_coords.0[1],
                );
            }

            for i in (shape.stride - shape.components)..shape.stride {
                self.vertices.push(shape.vertices[offset + i as usize]);
            }
        }

        if initial_vertex_cap != self.vertices.capacity() {
            self.dirty_vertex_size = true;
        }

        if initial_index_cap != self.indices.capacity() {
            self.dirty_index_size = true;
        }

        self.dirty_data = true;
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, &self.index_buf);
            glDeleteBuffers(1, &self.vertex_buf);
        }
    }
}

impl Shader {
    pub fn new(builder: crate::ShaderBuilder) -> Self {
        create_program(builder)
    }
}

impl Nshader for Shader {
    fn tint(&self) -> Option<i32> {
        self.tint
    }

    fn depth(&self) -> Option<i32> {
        self.depth
    }

    fn gradient(&self) -> bool {
        self.gradient
    }

    fn graphic(&self) -> bool {
        self.graphic
    }

    fn blending(&self) -> bool {
        self.blending
    }

    fn bind(&self) {
        unsafe {
            debug_assert_ne!(self.program, 0);
            glUseProgram(self.program);
            gl_assert!(&format!("glUseProgram {}", self.program));
        }
    }

    fn program(&self) -> u32 {
        self.program
    }
}

impl Ngraphic for Graphic {
    fn id(&self) -> u32 {
        self.id
    }

    fn width(&self) -> u16 {
        self.width as u16
    }

    fn height(&self) -> u16 {
        (((self.pixels.len() >> 2) as u32) / self.width as u32) as u16
    }

    fn resize(&mut self, pixels: &[u8], width: usize) {
        let width = width as i32;

        self.width = width;
        self.pixels = pixels.to_vec();

        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.id);
            gl_assert!("glBindTexture");

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                width,                                // w
                ((pixels.len() >> 2) as i32) / width, // h
                0,
                GL_RGBA,
                0x1401, /*GL_UNSIGNED_BYTE*/
                pixels.as_ptr() as *const _,
            );
            gl_assert!("glTexImage2D");

            glGenerateMipmap(GL_TEXTURE_2D);
            gl_assert!("glGenerateMipmap");
        }
    }

    fn update(&mut self, updater: &mut dyn FnMut(&mut [u8], u16)) {
        updater(self.pixels.as_mut_slice(), self.width as u16);

        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.id);
            gl_assert!("glBindTexture");

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                self.width,                                     // w
                ((self.pixels.len() >> 2) as i32) / self.width, // h
                0,
                GL_RGBA,
                0x1401, /*GL_UNSIGNED_BYTE*/
                self.pixels.as_ptr() as *const _,
            );
            gl_assert!("glTexImage2D");
        }
    }
}

pub struct OpenGL {
    surface: *mut c_void,
    display: *mut c_void,
    context: *mut c_void,
    config: *mut c_void,
    graphic: u32,
    depth: bool,
    blending: bool,
    shader: u32,
    shape_id: u32,
    vaa_col: bool,
    vaa_tex: bool,
}

impl OpenGL {
    #[cfg(unix)]
    pub(super) fn new(nwin: &mut dyn crate::Nwin) -> Option<Box<dyn Draw>> {
        let (display, config, context) = unsafe {
            // Get EGL Display from Window.
            let display = eglGetDisplay(match nwin.handle() {
                #[cfg(not(any(
                    target_os = "android",
                    target_os = "macos",
                    target_os = "ios"
                )))]
                crate::NwinHandle::Wayland(handle) => handle,
            });
            debug_assert!(!display.is_null());

            // Initialize EGL Display.
            let mut major = std::mem::MaybeUninit::uninit();
            let mut minor = std::mem::MaybeUninit::uninit();
            let ret =
                eglInitialize(display, major.as_mut_ptr(), minor.as_mut_ptr());
            debug_assert_eq!(ret, 1);

            // Connect EGL to either OpenGL or OpenGLES, whichever is available.
            // TODO: also support /*OPENGL:*/ 0x30A2
            let ret = eglBindAPI(/*OPENGL_ES:*/ 0x30A0);
            debug_assert_eq!(ret, 1);

            //
            let mut config = std::mem::MaybeUninit::<*mut c_void>::uninit();
            let mut n = std::mem::MaybeUninit::<i32>::uninit();
            let ret = eglChooseConfig(
                display,
                [
                    /*EGL_SURFACE_TYPE:*/ 0x3033,
                    /*EGL_WINDOW_BIT:*/ 0x04,
                    /*EGL_RED_SIZE:*/ 0x3024, 8,
                    /*EGL_GREEN_SIZE:*/ 0x3023, 8,
                    /*EGL_BLUE_SIZE:*/ 0x3022, 8,
                    //                /*EGL_ALPHA_SIZE:*/ 0x3021, 8,
                    /*EGL_DEPTH_SIZE*/
                    0x3025, 24, /*EGL_RENDERABLE_TYPE:*/ 0x3040,
                    /*EGL_OPENGL_ES2_BIT:*/ 0x0004,
                    /*EGL_NONE:*/ 0x3038,
                ]
                .as_ptr(),
                config.as_mut_ptr(),
                1,
                n.as_mut_ptr(),
            );
            debug_assert_eq!(ret, 1);

            let config = config.assume_init();

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
            graphic: 0,
            depth: false,
            blending: false,
            shader: 0,
            shape_id: std::u32::MAX,
            vaa_col: false,
            vaa_tex: false,
        };

        Some(Box::new(draw))
    }
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
        dbg!("Connecting 3…");

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

        unsafe {
            glEnable(0x0B44 /*GL_CULL_FACES*/);
            gl_assert!("glEnable#0");
            glDisable(0x0BD0 /*GL_DITHER*/);
            gl_assert!("glDisable#0");
        }

        unsafe {
            // Alpha Blending.
            glBlendFuncSeparate(
                /* GL_SRC_ALPHA */ 0x0302u32,
                /* GL_ONE_MINUS_SRC_ALPHA*/ 0x0303u32,
                /* GL_SRC_ALPHA */ 0x0302u32,
                /* GL_DST_ALPHA */ 0x0304u32,
            );
            gl_assert!("glBlendFuncSeparate");
        }

        /*        unsafe {
            let string = glGetString(0x1F03 /*gl extensions*/);
            let slice = std::ffi::CStr::from_ptr(string as *const _ as *const _);
            println!("ext: {}", slice.to_str().unwrap().contains("GL_EXT_base_instance"));
        }*/

        // Set default background for OpenGL.
        self.background(0.0, 0.0, 1.0);

        dbg!("End OpenGL initialization");
    }

    fn background(&mut self, r: f32, g: f32, b: f32) {
        unsafe {
            glClearColor(r, g, b, 1.0);
            gl_assert!("glClearColor");
        }
    }

    fn shader_new(
        &mut self,
        builder: crate::ShaderBuilder,
    ) -> Box<dyn Nshader> {
        Box::new(Shader::new(builder))
    }

    fn group_new(&mut self) -> Box<dyn Ngroup> {
        Box::new(Group::new())
    }

    fn toolbar(
        &mut self,
        w: u16,
        h: u16,
        toolbar_height: u16,
        shader: &dyn Nshader,
        shape: &mut dyn Ngroup,
    ) {
        /*let w = i32::from(w);
        let h = i32::from(h);
        let toolbar_height = i32::from(toolbar_height);
        unsafe {
            glViewport(0, h - toolbar_height, w, toolbar_height);
            self.draw(shader, shape);
            glViewport(0, 0, w, h - toolbar_height);
        }*/
    }

    fn begin_draw(&mut self) {
        self.shape_id = std::u32::MAX;
        unsafe {
            glClear(
                0x0000_4000 /*GL_COLOR_BUFFER_BIT*/ | 0x0000_0100, /*GL_DEPTH_BUFFER_BIT*/
            );
            gl_assert!("glClear");
        }
        unsafe { glEnableVertexAttribArray(GL_ATTRIB_POS) }
        gl_assert!("glEnableVertexAttribArray#4");
    }

    fn finish_draw(&mut self) {
        println!("FINISHING_DRAW");
        if self.vaa_col {
            unsafe { glDisableVertexAttribArray(GL_ATTRIB_COL) }
            gl_assert!("glDisableVertexAttribArray#0");
            self.vaa_col = false;
        }
        if self.vaa_tex {
            unsafe { glDisableVertexAttribArray(GL_ATTRIB_TEX) }
            gl_assert!("glDisableVertexAttribArray#1");
            self.vaa_tex = false;
        }
        unsafe { glDisableVertexAttribArray(GL_ATTRIB_POS) }
        gl_assert!("glDisableVertexAttribArray#4");
        unsafe {
            eglSwapBuffers(self.display, self.surface);
        }
        println!("FINISED_DRAW");
    }

    fn draw(&mut self, shader: &dyn Nshader, shape: &mut dyn Ngroup) {
        if self.bind_shader(shader) {
            if !self.vaa_col && shader.gradient() {
                unsafe { glEnableVertexAttribArray(GL_ATTRIB_COL) }
                gl_assert!("glEnableVertexAttribArray#2");
                self.vaa_col = true;
            }
            if !self.vaa_tex && shader.graphic() {
                unsafe { glEnableVertexAttribArray(GL_ATTRIB_TEX) }
                gl_assert!("glEnableVertexAttribArray#3");
                self.vaa_tex = true;
            }
            if self.vaa_col && !shader.gradient() {
                unsafe { glDisableVertexAttribArray(GL_ATTRIB_COL) }
                gl_assert!("glDisableVertexAttribArray#2");
                self.vaa_col = false;
            }
            if self.vaa_tex && !shader.graphic() {
                unsafe { glDisableVertexAttribArray(GL_ATTRIB_TEX) }
                gl_assert!("glDisableVertexAttribArray#3");
                self.vaa_tex = false;
            }
        }

        // IF SAME SHAPE
        let id = shape.id();
        if self.shape_id != id {
            self.shape_id = id;

            if shader.blending() && !self.blending {
                unsafe {
                    glEnable(0x0BE2 /*BLEND*/);
                    gl_assert!("glEnable#Blend");
                }
                self.blending = true;
            } else if !shader.blending() && self.blending {
                unsafe {
                    glDisable(0x0BE2 /*BLEND*/);
                    gl_assert!("glDisable#Blend");
                }
                self.blending = false;
            }

            if shader.depth().is_some() && !self.depth {
                unsafe {
                    glEnable(0x0B71 /*DEPTH_TEST*/);
                    gl_assert!("glEnable#DEPTH_TEST");
                }
                self.depth = true;
            } else if shader.depth().is_none() && self.depth {
                unsafe {
                    glDisable(0x0B71 /*DEPTH_TEST*/);
                    gl_assert!("glDisable#DEPTH_TEST");
                }
                self.depth = false;
            }

            unsafe {
                let stride = if shader.depth().is_some() { 3 } else { 2 }
                    + if shader.gradient() { 3 } else { 0 }
                    + if shader.graphic() { 2 } else { 0 };
                let stride = (stride * std::mem::size_of::<f32>()) as i32;

                shape.bind();

                // Always
                {
                    glVertexAttribPointer(
                        GL_ATTRIB_POS,
                        if shader.depth().is_some() { 3 } else { 2 },
                        0x1406, /*GL_FLOAT*/
                        0,      /*GL_FALSE*/
                        stride,
                        std::ptr::null(),
                    );
                    gl_assert!("glVertexAttribPointer#POS");
                }

                // Only if Gradient is enabled.
                if shader.gradient() {
                    let ptr: *const f32 = std::ptr::null();
                    glVertexAttribPointer(
                        GL_ATTRIB_COL,
                        3,
                        0x1406, /*GL_FLOAT*/
                        0,      /*GL_FALSE*/
                        stride,
                        ptr.offset(if shader.depth().is_some() {
                            3
                        } else {
                            2
                        }),
                    );
                    gl_assert!("glVertexAttribPointer#COL");
                }

                // Only if Texture is enabled.
                if shader.graphic() {
                    let ptr: *const f32 = std::ptr::null();
                    glVertexAttribPointer(
                        GL_ATTRIB_TEX,
                        2,
                        0x1406, /*GL_FLOAT*/
                        0,      /*GL_FALSE*/
                        stride,
                        ptr.offset(
                            if shader.depth().is_some() { 3 } else { 2 }
                                + if shader.gradient() { 3 } else { 0 },
                        ),
                    );
                    gl_assert!("glVertexAttribPointer#TEX");
                }
            }
        } // END IF

        unsafe {
            glDrawElements(
                0x0004, /*GL_TRIANGLES*/
                shape.len(),
                0x1405, /*GL_UNSIGNED_INT*/
                std::ptr::null(),
            );
        }
    }

    fn graphic(
        &mut self,
        pixels: &[u8],
        width: usize,
        height: usize,
    ) -> Box<dyn Ngraphic> {
        Box::new(Graphic::new(pixels, width, height))
    }

    fn bind_graphic(&mut self, graphic: &dyn Ngraphic) {
        // Only bind, if it's not already bound.
        if self.graphic != graphic.id() {
            unsafe {
                glBindTexture(GL_TEXTURE_2D, graphic.id());
            }
            gl_assert!("glBindTexture");
            // Update which graphic is bound.
            self.graphic = graphic.id();
        }
    }

    fn camera(&mut self, shader: &dyn Nshader, cam: crate::Transform) {
        if let Some(a) = shader.depth() {
            self.bind_shader(shader);
            unsafe {
                glUniformMatrix4fv(
                    a,
                    1,
                    0, /*GL_FALSE*/
                    cam.as_ptr() as *const c_void,
                );
            }
        }
    }

    fn tint(&mut self, shader: &dyn Nshader, tint: [f32; 4]) {
        if let Some(a) = shader.tint() {
            self.bind_shader(shader);
            unsafe {
                glUniform4f(a, tint[0], tint[1], tint[2], tint[3]);
            }
        }
    }
}

impl OpenGL {
    fn bind_shader(&mut self, shader: &dyn Nshader) -> bool {
        let shader_id = shader.program();
        if shader_id != self.shader {
            shader.bind();
            self.shader = shader_id;
            return true;
        }
        false
    }
}

fn vbo_set<T>(target: u32, vbo: u32, start: usize, size: usize, data: &[T]) {
    unsafe {
        glBindBuffer(target, vbo);
        gl_assert!(&format!("glBindBuffer#{:X}", target));
        glBufferSubData(
            target,
            start as isize,
            (size * std::mem::size_of::<T>()) as isize,
            data.as_ptr() as *const _,
        );
        gl_assert!("glBufferData");
    }
}

fn vbo_resize<T>(target: u32, vbo: u32, data: &Vec<T>) {
    unsafe {
        glBindBuffer(target, vbo);
        gl_assert!(&format!("glBindBuffer#{:X}", target));
        glBufferData(
            target,
            (data.capacity() * std::mem::size_of::<T>()) as isize,
            (*data).as_ptr() as *const _,
            0x88E8, /*GL_DYNAMIC_DRAW*/
        );
        gl_assert!("glBufferData");
    }
}

// Create an OpenGL vertex buffer object.
fn vbo_new<T>(target: u32) -> (u32, Vec<T>) {
    unsafe {
        let mut buffer = std::mem::MaybeUninit::<u32>::uninit();
        glGenBuffers(1 /*1 buffer*/, buffer.as_mut_ptr());
        gl_assert!("glGenBuffers");
        let buffer = buffer.assume_init();

        let vector = vec![];
        vbo_resize(target, buffer, &vector);
        (buffer, vector)
    }
}

/// Create a shader program.
fn create_program(builder: crate::ShaderBuilder) -> Shader {
    let frag = create_shader(
        builder.opengl_frag.as_ptr() as *const _ as *const _,
        0x8B30, /*GL_FRAGMENT_SHADER*/
    );
    let vert = create_shader(
        builder.opengl_vert.as_ptr() as *const _ as *const _,
        0x8B31, /*GL_VERTEX_SHADER*/
    );
    let program = unsafe { glCreateProgram() };
    gl_assert!("glCreateProgram");
    unsafe {
        glAttachShader(program, frag);
        gl_assert!("glAttachShader#1");
        glAttachShader(program, vert);
        gl_assert!("glAttachShader#2");
    }
    // Vertex attributes
    unsafe {
        // All shader programs have position.
        glBindAttribLocation(
            program,
            GL_ATTRIB_POS,
            b"pos\0".as_ptr() as *const _ as *const _,
        );
        gl_assert!("glBindAttribLocation#pos");
        //
        if builder.gradient {
            glBindAttribLocation(
                program,
                GL_ATTRIB_COL,
                b"col\0".as_ptr() as *const _ as *const _,
            );
            gl_assert!("glBindAttribLocation#col");
        }
        //
        if builder.graphic {
            glBindAttribLocation(
                program,
                GL_ATTRIB_TEX,
                b"texpos\0".as_ptr() as *const _ as *const _,
            );
            gl_assert!("glBindAttribLocation#texpos");
        }
        glLinkProgram(program);
        gl_assert!("glLinkProgram");
    }
    // Bind the shader program.
    unsafe {
        glUseProgram(program);
        gl_assert!(&format!("glUseProgram#0 {}", program));
    }
    // Link status
    let mut status = std::mem::MaybeUninit::<i32>::uninit();
    let status = unsafe {
        glGetProgramiv(
            program,
            0x8B82,
            /*GL_LINK_STATUS*/ status.as_mut_ptr(),
        );
        gl_assert!("glGetProgramiv");
        status.assume_init()
    };
    if status == 0 {
        let mut log = [0u8; 1000];
        let mut len = std::mem::MaybeUninit::<i32>::uninit();
        unsafe {
            glGetProgramInfoLog(
                program,
                1000,
                len.as_mut_ptr(),
                log.as_mut_ptr() as *mut _ as *mut _,
            );
            gl_assert!("glGetProgramInfoLog");
        }
        let log = String::from_utf8_lossy(&log);
        panic!("Error: linking:\n{}", log);
    }

    let depth = if builder.depth {
        let camera = unsafe {
            glGetUniformLocation(
                program,
                "cam\0".as_ptr() as *const _ as *const _,
            )
        };
        gl_assert!("glGetUniformLocation#cam");
        assert!(camera > -1);

        Some(camera)
    } else {
        None
    };

    let tint = if builder.tint {
        let tint = unsafe {
            glGetUniformLocation(
                program,
                "tint\0".as_ptr() as *const _ as *const _,
            )
        };
        gl_assert!("glGetUniformLocation#tint");
        assert!(tint > -1);

        Some(tint)
    } else {
        None
    };

    let graphic = builder.graphic;

    Shader {
        program,
        gradient: builder.gradient,
        graphic,
        depth,
        tint,
        blending: builder.blend,
    }
}

fn create_shader(source: *const i8, shader_type: u32) -> u32 {
    let shader = unsafe { glCreateShader(shader_type) };
    gl_assert!("glCreateShader");
    debug_assert!(shader != 0);

    unsafe {
        glShaderSource(shader, 1, [source].as_ptr(), std::ptr::null());
        gl_assert!("glShaderSource");
        glCompileShader(shader);
        gl_assert!("glCompileShader");
    }

    let mut status = std::mem::MaybeUninit::<i32>::uninit();
    let status = unsafe {
        glGetShaderiv(
            shader,
            0x8B81, /*GL_COMPILE_STATUS*/
            status.as_mut_ptr(),
        );
        gl_assert!("glGetShaderiv");
        status.assume_init()
    };
    if status == 0 {
        let mut log = [0u8; 1000];
        let mut len = std::mem::MaybeUninit::<i32>::uninit();
        unsafe {
            glGetShaderInfoLog(
                shader,
                1000,
                len.as_mut_ptr(),
                log.as_mut_ptr() as *mut _ as *mut _,
            );
            gl_assert!("glGetShaderInfoLog");
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

    shader
}
