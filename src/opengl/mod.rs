use std::ffi::c_void;

use super::Draw;
use super::DrawHandle;
use super::Window;
use crate::Nshader;
use crate::Nshape;
use crate::Nvertices;
use crate::Ngraphic;

mod platform;

// Position
const GL_ATTRIB_POS: u32 = 0;
// Color
const GL_ATTRIB_COL: u32 = 1;
// Texture Coordinates Begin (May have multiple)
const GL_ATTRIB_TEX: u32 = 2;

const GL_RGBA: u32 = 0x1908;
const GL_TEXTURE_2D: u32 = 0x0DE1;

extern "C" {
    fn glGetError() -> u32;
}

#[allow(unused)]
fn get_error() {
    match unsafe { glGetError() } {
        0 => print!(""),
        0x0500 => panic!("OpenGL: Invalid Enum"),
        0x0501 => panic!("OpenGL: Invalid Value"),
        0x0502 => panic!("OpenGL: Invalid Operation"),
        0x0503 => panic!("OpenGL: Invalid Stack Overflow"),
        0x0504 => panic!("OpenGL: Invalid Stack Underflow"),
        0x0505 => panic!("OpenGL: Invalid Out of Memory"),
        _ => panic!("OpenGL: Unknown Error"),
    }
}

#[cfg(debug_assertions)]
macro_rules! gl_assert {
    () => {
        get_error();
    };
}

#[cfg(not(debug_assertions))]
macro_rules! gl_assert {
    () => {};
}

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
    fn glUniform1i(location: i32, v0: i32) -> ();
    fn glUniform2f(location: i32, v0: f32, v1: f32) -> ();
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
//    fn glDrawArrays(mode: u32, first: i32, count: i32);
    fn glDrawElements(mode: u32, count: i32, draw_type: u32, indices: *const c_void) -> ();
    // fn glDrawElementsInstanced(mode: u32, count: i32, draw_type: u32, indices: *const c_void, instance_count: i32) -> ();
    fn glDisableVertexAttribArray(index: u32) -> ();
    fn glGenBuffers(n: i32, buffers: *mut u32) -> ();
    fn glBindBuffer(target: u32, buffer: u32) -> ();
    fn glBindBufferBase(target: u32, index: u32, buffer: u32) -> ();
    fn glBufferData(
        target: u32,
        size: isize,
        data: *const c_void,
        usage: u32,
    ) -> ();
    fn glDeleteBuffers(n: i32, buffers: *const u32) -> ();
    // fn glGetString(name: u32) -> *const u8;
    fn glGenTextures(n: u32, textures: *mut u32) -> ();
    fn glBindTexture(target: u32, texture: u32) -> ();
    fn glTexParameteri(target: u32, pname: u32, param: i32) -> ();
    fn glTexImage2D(target: u32, level: i32, internalFormat: i32, width: i32,
        height: i32, border: i32, format: u32, stype: u32, pixels: *const u8) -> ();
    fn glGenerateMipmap(target: u32);
    fn glViewport(x: i32, y: i32, width: i32, height: i32) -> ();
}

/// A shader.  Shaders are a program that runs on the GPU to render a `Shape`.
pub struct Shader {
    // An OpenGL shader program ID.
    program: u32,
    // True if OpenGL color vertex attribute exists.
    gradient: bool,
    // Some if OpenGL texture uniform exists.
    graphic: Option<(i32, i32)>,
    // TODO
    transforms: Vec<i32>,
    // True if 3D.
    depth: bool,
    // True if transparency is allowed.
    blending: bool,
    // Maximum number of instances.
    instance_count: u16,
    //
    id: i32,
}

/// 
pub struct Graphic {
    id: u32,
    pixels: Vec<u8>,
    width: i32,
}

impl Graphic {
    pub fn new(pixels: &[u8], width: usize) -> Self {
        let width = width as i32;

        let new_texture = unsafe {
            let mut new_texture = std::mem::MaybeUninit::uninit();
            glGenTextures(1, new_texture.as_mut_ptr());
            get_error();
            new_texture.assume_init()
        };

        unsafe {
            #![allow(unused)]

            const GL_TEXTURE_MAG_FILTER: u32 = 0x2800;
            const GL_TEXTURE_MIN_FILTER: u32 = 0x2801;
            const GL_NEAREST: i32 = 0x2600;
            const GL_LINEAR: i32 = 0x2601;
            const GL_LINEAR_MIPMAP_LINEAR: i32 = 0x2703;
            const GL_NEAREST_MIPMAP_NEAREST: i32 = 0x2700;
            const GL_NEAREST_MIPMAP_LINEAR: i32 = 0x2702;
            const GL_TEXTURE_WRAP_S: u32 = 0x2802;
            const GL_TEXTURE_WRAP_T: u32 = 0x2803;
            const GL_CLAMP_TO_EDGE: i32 = 0x812F;

            glBindTexture(GL_TEXTURE_2D, new_texture);
            get_error();
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_S, GL_CLAMP_TO_EDGE);
            get_error();
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_WRAP_T, GL_CLAMP_TO_EDGE);
            get_error();
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MIN_FILTER, GL_NEAREST_MIPMAP_LINEAR);
            get_error();
            glTexParameteri(GL_TEXTURE_2D, GL_TEXTURE_MAG_FILTER, GL_LINEAR);
            get_error();
            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                width, // w
                ((pixels.len() >> 2) as i32) / width, // h
                0,
                GL_RGBA,
                0x1401 /*GL_UNSIGNED_BYTE*/,
                pixels.as_ptr() as *const _,
            );
            get_error();

            glGenerateMipmap(GL_TEXTURE_2D);
            get_error();
        }

        Graphic {
            id: new_texture,
            pixels: pixels.to_vec(),
            width,
        }
    }
}

/// A list of vertices.
pub struct Vertices {
    vbo: u32,
}

impl Vertices {
    /// Create a new `VertexList`.  `dim`: 0 or 2~4 dimensions.  `gradient` is 3(RGB) or 4(RGBA).  `graphic_coords` is how many graphics need coordinates.
    pub fn new(vertices: &[f32]) -> Vertices {
        Vertices {
            vbo: create_vbo(vertices, 0x8892 /*GL_ARRAY_BUFFER*/),
        }
    }
}

impl Drop for Vertices {
    fn drop(&mut self) {
        unsafe {
            glDeleteBuffers(1, &self.vbo);
        }
    }
}

/// A shape.  Shapes are a list of indices into `Vertices`.
pub struct Shape {
    indices: Vec<u16>,
    instances: Vec<crate::Transform>,
}

impl Shape {
    pub fn new(builder: crate::ShapeBuilder) -> Shape {
        Shape {
            indices: builder.indices.to_vec(), // TODO: use vec??
            instances: Vec::with_capacity(builder.num_instances.into()),
        }
    }
}

impl Shader {
    pub fn new(builder: crate::ShaderBuilder) -> Self {
        create_program(builder)
    }
}

impl Nshader for Shader {
    fn depth(&self) -> bool {
        self.depth
    }

    fn gradient(&self) -> bool {
        self.gradient
    }

    fn graphic(&self) -> Option<(i32, i32)> {
        self.graphic
    }

    fn blending(&self) -> bool {
        self.blending
    }

    fn bind(&self) {
        unsafe {
            debug_assert_ne!(self.program, 0);
            glUseProgram(self.program);
            gl_assert!();
        }
    }

    fn transform(&self, index: usize) -> Option<&i32> {
        self.transforms.get(index)
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn num_instances(&self) -> u16 {
        self.instance_count
    }
}

impl Nshape for Shape {
    fn len(&self) -> i32 {
        self.indices.len() as i32
    }

    fn ptr(&self) -> *const c_void {
        self.indices.as_ptr() as *const _ as *const _
    }

    fn instances(&mut self, transforms: &[crate::Transform]) {
        self.instances = transforms.to_vec();
    }

    fn transform(&mut self, index: u16, transform: crate::Transform) {
        self.instances[index as usize] = transform;
    }

    fn instances_ptr(&self) -> *const c_void {
        self.instances.as_ptr() as *const _ as *const _
/*        debug_assert_ne!(self.instances_vbo, 0);
        unsafe {
            glBindBufferBase(0x8A11 /*GL_UNIFORM_BUFFER*/, 0, self.instances_vbo);
            gl_assert!();
        }*/
    }

    fn instances_num(&self) -> i32 {
        self.instances.len() as i32
    }
}

impl Nvertices for Vertices {
    fn bind(&self) {
        debug_assert_ne!(self.vbo, 0);
        unsafe {
            glBindBuffer(0x8892 /*GL_ARRAY_BUFFER*/, self.vbo);
            gl_assert!();
        }
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
            get_error();

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                width, // w
                ((pixels.len() >> 2) as i32) / width, // h
                0,
                GL_RGBA,
                0x1401 /*GL_UNSIGNED_BYTE*/,
                pixels.as_ptr() as *const _,
            );
            get_error();

            glGenerateMipmap(GL_TEXTURE_2D);
            get_error();
        }
    }

    fn update(&mut self, updater: &mut FnMut(&mut [u8], u16)) {
        updater(self.pixels.as_mut_slice(), self.width as u16);

        unsafe {
            glBindTexture(GL_TEXTURE_2D, self.id);
            get_error();

            glTexImage2D(
                GL_TEXTURE_2D,
                0,
                GL_RGBA as i32,
                self.width, // w
                ((self.pixels.len() >> 2) as i32) / self.width, // h
                0,
                GL_RGBA,
                0x1401 /*GL_UNSIGNED_BYTE*/,
                self.pixels.as_ptr() as *const _,
            );
            get_error();
        }
    }
}

pub struct OpenGL {
    surface: *mut c_void,
    display: *mut c_void,
    context: *mut c_void,
    config: *mut c_void,
    graphic: u32,
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

/*        unsafe {
            let string = glGetString(0x1F03 /*gl extensions*/);
            let slice = std::ffi::CStr::from_ptr(string as *const _ as *const _);
            println!("ext: {}", slice.to_str().unwrap().contains("GL_EXT_base_instance"));
        }*/

        // Set default background for OpenGL.
        self.background(0.0, 0.0, 1.0);
    }

    fn background(&mut self, r: f32, g: f32, b: f32) {
        unsafe {
            glClearColor(r, g, b, 0.5); // TODO ?
            gl_assert!();
        }
    }

    fn shader_new(&mut self, builder: crate::ShaderBuilder) -> Box<Nshader> {
        Box::new(Shader::new(builder))
    }

    fn vertices_new(&mut self, vertices: &[f32]) -> Box<Nvertices> {
        Box::new(Vertices::new(vertices))
    }

    fn shape_new(&mut self, builder: crate::ShapeBuilder) -> Box<Nshape> {
        Box::new(Shape::new(builder))
    }

    fn toolbar(&mut self, w: u16, h: u16, toolbar_height: u16, shader: &Nshader, vertlist: &Nvertices, shape: &Nshape) -> () {
        let w = w as i32;
        let h = h as i32;
        let toolbar_height = toolbar_height as i32;
        unsafe {
            glViewport(0, h - toolbar_height, w, toolbar_height);
            self.draw(shader, vertlist, shape);
            glViewport(0, 0, w, h - toolbar_height);
        }
    }

    fn begin_draw(&mut self) {
        unsafe {
            glClear(0x0000_4000 /*GL_COLOR_BUFFER_BIT*/);
            gl_assert!();
        }
    }

    fn finish_draw(&mut self) {
        unsafe {
            eglSwapBuffers(self.display, self.surface);
        }
    }

    fn draw(&mut self, shader: &Nshader, vertlist: &Nvertices, shape: &Nshape) {
        shader.bind();

        unsafe {
            let mut index = 0;
            while let Some(uniform_id) = shader.transform(index) {
                glUniformMatrix4fv(
                    *uniform_id,
                    shape.instances_num(),
                    0, /*GL_FALSE*/
                    shape.instances_ptr(),
                );
                gl_assert!();
                index += 1;
            }

            let stride = if shader.depth() { 3 } else { 2 } + if shader.gradient() { 3 } else { 0 } + if shader.graphic().is_some() { 2 } else { 0 };
            let stride = (stride * std::mem::size_of::<f32>()) as i32;

            vertlist.bind();

            // Always
            {
                glVertexAttribPointer(
                    GL_ATTRIB_POS,
                    if shader.depth() { 3 } else { 2 },
                    0x1406, /*GL_FLOAT*/
                    0,      /*GL_FALSE*/
                    stride,
                    std::ptr::null(),
                );
                gl_assert!();
                glEnableVertexAttribArray(GL_ATTRIB_POS);
                gl_assert!();
            }

            // Only if Gradient is enabled.
            if shader.gradient() {
                vertlist.bind(); // TODO: is needed?

                let ptr: *const f32 = std::ptr::null();
                glVertexAttribPointer(
                    GL_ATTRIB_COL,
                    3,
                    0x1406, /*GL_FLOAT*/
                    0,      /*GL_FALSE*/
                    stride,
                    ptr.offset(if shader.depth() { 3 } else { 2 }),
                );
                gl_assert!();
                glEnableVertexAttribArray(GL_ATTRIB_COL);
                gl_assert!();
            }

            // Only if Gradient is enabled.
            if shader.graphic().is_some() {
                vertlist.bind(); // TODO: is needed?

                let ptr: *const f32 = std::ptr::null();
                glVertexAttribPointer(
                    GL_ATTRIB_TEX,
                    2,
                    0x1406, /*GL_FLOAT*/
                    0,      /*GL_FALSE*/
                    stride,
                    ptr.offset(if shader.depth() { 3 } else { 2 } + if shader.gradient() { 3 } else { 0 }),
                );
                gl_assert!();
                glEnableVertexAttribArray(GL_ATTRIB_TEX);
                gl_assert!();
            }

            // Draw
            // TODO use glDrawElementsInstanced only if available (when not
            // GLES2).
//            glDrawElementsInstanced(0x0004 /*GL_TRIANGLES*/, shape.len(), 0x1403 /*GL_UNSIGNED_SHORT*/, shape.ptr(), shape.instances_num());
            {
                for i in 0..shape.instances_num() {
                    glUniform1i(shader.id(), i);
                    glDrawElements(0x0004 /*GL_TRIANGLES*/, shape.len(), 0x1403 /*GL_UNSIGNED_SHORT*/, shape.ptr());
                }
            }
            gl_assert!();

            // Disable
            glDisableVertexAttribArray(GL_ATTRIB_POS);
            gl_assert!();
            if shader.gradient() {
                glDisableVertexAttribArray(GL_ATTRIB_COL);
                gl_assert!();
            }
            if shader.graphic().is_some() {
                glDisableVertexAttribArray(GL_ATTRIB_TEX);
                gl_assert!();
            }
        }
    }

    fn instances(&mut self, shape: &mut Nshape, transforms: &[crate::Transform]) {
        shape.instances(transforms);
    }

    fn transform(&mut self, shape: &mut Nshape, instance: u16, transform: crate::Transform) {
        shape.transform(instance, transform);
    }

    fn graphic(&mut self, pixels: &[u8], width: usize) -> Box<Ngraphic> {
        Box::new(Graphic::new(pixels, width))
    }

    fn bind_graphic(&mut self, graphic: &Ngraphic) {
        // Only bind, if it's not already bound.
        if self.graphic != graphic.id() {
            unsafe {
                glBindTexture(0x0DE1 /*GL_TEXTURE_2D*/, graphic.id());
            }
            get_error();
            // Update which graphic is bound.
            self.graphic = graphic.id();
        }
    }

    fn texture_coords(&mut self, shader: &Nshader, coords: ([f32; 2], [f32; 2])) {
        if let Some((a, b)) = shader.graphic() {
            shader.bind();
            unsafe {
                glUniform2f(a, coords.0[0], coords.0[1]);
                glUniform2f(b, coords.1[0], coords.1[1]);
            }
        }
    }
}

// Create an OpenGL vertex buffer object.
fn create_vbo<T>(vertices: &[T], target: u32) -> u32 {
    unsafe {
        let mut buffer = std::mem::MaybeUninit::<u32>::uninit();
        glGenBuffers(1 /*1 buffer*/, buffer.as_mut_ptr());
        gl_assert!();
        let buffer = buffer.assume_init();
        if target == 0x8892 {
            glBindBuffer(target, buffer);
        } else {
            glBindBufferBase(target, 0, buffer);
        }
        gl_assert!();
        // TODO: maybe use glMapBuffer & glUnmapBuffer instead?
        glBufferData(
            target,
            (vertices.len() * std::mem::size_of::<T>()) as isize,
            vertices.as_ptr() as *const _,
            if target == 0x8892 {
                0x88E4 /*GL_STATIC_DRAW - never changes*/
            } else {
                0x88E8 /*GL_DYNAMIC_DRAW*/
            },
        );
        gl_assert!();
        buffer
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
    gl_assert!();
    unsafe {
        glAttachShader(program, frag);
        gl_assert!();
        glAttachShader(program, vert);
        gl_assert!();
        glLinkProgram(program);
        gl_assert!();
    }
    let mut status = std::mem::MaybeUninit::<i32>::uninit();
    let status = unsafe {
        glGetProgramiv(program, 0x8B82, /*GL_LINK_STATUS*/ status.as_mut_ptr());
        gl_assert!();
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
            gl_assert!();
        }
        let log = String::from_utf8_lossy(&log);
        panic!("Error: linking:\n{}", log);
    }
    // Bind the shader program.
    unsafe {
        glUseProgram(program);
        gl_assert!();
    }
    // Vertex attributes
    unsafe {
        // All shader programs have position.
        glBindAttribLocation(
            program,
            GL_ATTRIB_POS,
            b"pos\0".as_ptr() as *const _ as *const _,
        );
        gl_assert!();
        //
        if builder.gradient {
            glBindAttribLocation(
                program,
                GL_ATTRIB_COL,
                b"col\0".as_ptr() as *const _ as *const _,
            );
            gl_assert!();
        }
        //
        if builder.graphic {
            glBindAttribLocation(
                program,
                GL_ATTRIB_TEX,
                b"texpos\0".as_ptr() as *const _ as *const _,
            );
            gl_assert!();
        }
        // 
        glLinkProgram(program);
        gl_assert!();
    }
    // Uniforms
//    let mut groups = Vec::with_capacity(builder.group as usize);
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
        gl_assert!();
        assert!(handle > -1);
        transforms.push(handle);
    }

    let id = unsafe {
        glGetUniformLocation(program, "cala_InstanceID\0".as_ptr() as *const _ as *const _)
    };
    gl_assert!();
    assert!(id > -1);

    let graphic = if builder.graphic {
        let tsc_translate = unsafe {
            glGetUniformLocation(program, "tsc_translate\0".as_ptr() as *const _ as *const _)
        };
        gl_assert!();
        assert!(id > -1);

        let tsc_scale = unsafe {
            glGetUniformLocation(program, "tsc_scale\0".as_ptr() as *const _ as *const _)
        };
        gl_assert!();
        assert!(id > -1);

        Some((tsc_translate, tsc_scale))
    } else {
        None
    };

    Shader {
        program,
        gradient: builder.gradient,
        graphic,
//        groups,
        transforms,
        depth: builder.depth,
        blending: builder.blend,
        instance_count: builder.instance_count,
        id,
    }
}

fn create_shader(source: *const i8, shader_type: u32) -> u32 {
    let shader = unsafe { glCreateShader(shader_type) };
    gl_assert!();
    debug_assert!(shader != 0);

    unsafe {
        glShaderSource(shader, 1, [source].as_ptr(), std::ptr::null());
        gl_assert!();
        glCompileShader(shader);
        gl_assert!();
    }

    let mut status = std::mem::MaybeUninit::<i32>::uninit();
    let status = unsafe {
        glGetShaderiv(shader, 0x8B81 /*GL_COMPILE_STATUS*/, status.as_mut_ptr());
        gl_assert!();
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
            gl_assert!();
        }
        let log = String::from_utf8_lossy(&log);
        panic!(
            "Error: compiling {}: {}\n",
            if shader_type == 0x8B31 /*GL_VERTEX_SHADER*/ {
                "vertex"
            } else {
                "fragment"
            },
            log
        );
    }

    shader
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
        let mut major = std::mem::MaybeUninit::uninit();
        let mut minor = std::mem::MaybeUninit::uninit();
        let ret = eglInitialize(display, major.as_mut_ptr(), minor.as_mut_ptr());
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
                /*EGL_WINDOW_BIT:*/ 0x04, /*EGL_RED_SIZE:*/ 0x3024,
                8, /*EGL_GREEN_SIZE:*/ 0x3023, 8,
                /*EGL_BLUE_SIZE:*/ 0x3022, 8,
                /*EGL_RENDERABLE_TYPE:*/ 0x3040,
                /*EGL_OPENGL_ES2_BIT:*/ 0x0004, /*EGL_NONE:*/ 0x3038,
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
    };

    Some(Box::new(draw))
}
