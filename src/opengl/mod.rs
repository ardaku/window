use std::ffi::c_void;

use super::Draw;
use super::DrawHandle;
use super::Window;
use crate::Ngraphic;
use crate::Nshader;
use crate::Nshape;
use crate::Nvertices;

mod platform;

// Position
const GL_ATTRIB_POS: u32 = 0;
// Texture Coordinates Begin (May have multiple)
const GL_ATTRIB_TEX: u32 = 1;
// Color
const GL_ATTRIB_COL: u32 = 2;

const GL_RGBA: u32 = 0x1908;
const GL_TEXTURE_2D: u32 = 0x0DE1;

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
    ($x:expr) => {$x};
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
    //    fn glDrawArrays(mode: u32, first: i32, count: i32);
    fn glDrawElements(
        mode: u32,
        count: i32,
        draw_type: u32,
        indices: *const c_void,
    ) -> ();
    // fn glDrawElementsInstanced(mode: u32, count: i32, draw_type: u32, indices: *const c_void, instance_count: i32) -> ();
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
    graphic: Option<(i32, i32)>,
    // TODO
    transforms: Vec<i32>,
    // Some if 3D.
    depth: Option<i32>,
    // Some if tint.
    tint: Option<i32>,
    // True if transparency is allowed.
    blending: bool,
    // Maximum number of instances.
    instance_count: u16,
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
//            const GL_NEAREST_MIPMAP_NEAREST: i32 = 0x2700;

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
    index_buf: u32,
    index_len: usize,
    instances: Vec<crate::Transform>,
}

impl Shape {
    pub fn new(builder: crate::ShapeBuilder) -> Shape {
        const GL_ELEMENT_ARRAY_BUFFER: u32 = 0x8893;

        Shape {
            index_buf: create_vbo(&builder.indices, GL_ELEMENT_ARRAY_BUFFER),
            index_len: builder.indices.len(),
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
    fn tint(&self) -> Option<i32> {
        self.tint
    }

    fn depth(&self) -> Option<i32> {
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
            gl_assert!(&format!("glUseProgram {}", self.program));
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

    fn program(&self) -> u32 {
        self.program
    }
}

impl Nshape for Shape {
    fn len(&self) -> i32 {
        self.index_len as i32
    }

    fn buf(&self) -> u32 {
        self.index_buf
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
            gl_assert!("glBindBufferBase");
        }*/
    }

    fn instances_num(&self) -> i32 {
        self.instances.len() as i32
    }

    fn bind(&self) {
        debug_assert_ne!(self.index_buf, 0);
        unsafe {
            glBindBuffer(0x8893 /*GL_ELEMENT_ARRAY_BUFFER*/, self.index_buf);
            gl_assert!("glBindBuffer#Element");
        }
    }

    fn id(&self) -> u32 {
        self.index_buf
    }
}

impl Nvertices for Vertices {
    fn bind(&self) {
        debug_assert_ne!(self.vbo, 0);
        unsafe {
            glBindBuffer(0x8892 /*GL_ARRAY_BUFFER*/, self.vbo);
            gl_assert!("glBindBuffer");
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
    tex_coords_id: (u32, [f32; 2], [f32; 2]),
    shape_id: u32,
    vaa_col: bool,
    vaa_tex: bool,
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
    }

    fn background(&mut self, r: f32, g: f32, b: f32) {
        unsafe {
            glClearColor(r, g, b, 1.0);
            gl_assert!("glClearColor");
        }
    }

    fn shader_new(&mut self, builder: crate::ShaderBuilder) -> Box<dyn Nshader> {
        Box::new(Shader::new(builder))
    }

    fn vertices_new(&mut self, vertices: &[f32]) -> Box<dyn Nvertices> {
        Box::new(Vertices::new(vertices))
    }

    fn shape_new(&mut self, builder: crate::ShapeBuilder) -> Box<dyn Nshape> {
        Box::new(Shape::new(builder))
    }

    fn toolbar(
        &mut self,
        w: u16,
        h: u16,
        toolbar_height: u16,
        shader: &dyn Nshader,
        vertlist: &dyn Nvertices,
        shape: &dyn Nshape,
    ) -> () {
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
    }

    fn draw(&mut self, shader: &dyn Nshader, vertlist: &dyn Nvertices, shape: &dyn Nshape) {
        if self.bind_shader(shader) {
            if !self.vaa_col && shader.graphic().is_some() {
                unsafe { glEnableVertexAttribArray(GL_ATTRIB_COL) }
                gl_assert!("glEnableVertexAttribArray#2");
            }
            if !self.vaa_tex && shader.gradient() {
                unsafe { glEnableVertexAttribArray(GL_ATTRIB_TEX) }
                gl_assert!("glEnableVertexAttribArray#3");
            }
            if self.vaa_col && shader.graphic().is_none() {
                unsafe { glDisableVertexAttribArray(GL_ATTRIB_COL) }
                gl_assert!("glDisableVertexAttribArray#2");
            }
            if self.vaa_tex && !shader.gradient() {
                unsafe { glDisableVertexAttribArray(GL_ATTRIB_TEX) }
                gl_assert!("glDisableVertexAttribArray#3");
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
                + if shader.graphic().is_some() { 2 } else { 0 };
            let stride = (stride * std::mem::size_of::<f32>()) as i32;

            vertlist.bind();
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
                    ptr.offset(if shader.depth().is_some() { 3 } else { 2 }),
                );
                gl_assert!("glVertexAttribPointer#COL");
            }

            // Only if Texture is enabled.
            if shader.graphic().is_some() {
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
            let mut index = 0;
            while let Some(uniform_id) = shader.transform(index) {
                glUniformMatrix4fv(
                    *uniform_id,
                    shape.instances_num(),
                    0, /*GL_FALSE*/
                    shape.instances_ptr(),
                );
                gl_assert!("glUniformMatrix4fv");
                index += 1;
            }
        }

        unsafe {

            // Draw
            // TODO use glDrawElementsInstanced only if available (when not
            // GLES2).
            //            glDrawElementsInstanced(0x0004 /*GL_TRIANGLES*/, shape.len(), 0x1403 /*GL_UNSIGNED_SHORT*/, shape.ptr(), shape.instances_num());

//            {
//                for i in 0..shape.instances_num() {
//                    glUniform1i(shader.id(), i);
//                    gl_assert!("glUniform1i");
                    glDrawElements(
                        0x0004, /*GL_TRIANGLES*/
                        shape.len(),
                        0x1403, /*GL_UNSIGNED_SHORT*/
                        std::ptr::null(),
                    );
//                    gl_assert!("glDrawElements");
//                }
//            }
        }
    }

    fn instances(
        &mut self,
        shape: &mut dyn Nshape,
        transforms: &[crate::Transform],
    ) {
        shape.instances(transforms);
    }

    fn transform(
        &mut self,
        shape: &mut dyn Nshape,
        instance: u16,
        transform: crate::Transform,
    ) {
        shape.transform(instance, transform);
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

    fn texture_coords(
        &mut self,
        shader: &dyn Nshader,
        coords: (u32, [f32; 2], [f32; 2]),
    ) {
        if let Some((a, b)) = shader.graphic() {
            if coords.0 != self.tex_coords_id.0 {
                self.bind_shader(shader);
                unsafe {
                    glUniform2f(a, coords.1[0], coords.1[1]);
                    glUniform2f(b, coords.2[0], coords.2[1]);
                }
                self.tex_coords_id = coords;
            }
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
                    cam.mat.as_ptr() as *const c_void,
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

// Create an OpenGL vertex buffer object.
fn create_vbo<T>(vertices: &[T], target: u32) -> u32 {
    unsafe {
        let mut buffer = std::mem::MaybeUninit::<u32>::uninit();
        glGenBuffers(1 /*1 buffer*/, buffer.as_mut_ptr());
        gl_assert!("glGenBuffers");
        let buffer = buffer.assume_init();
        if target == 0x8892 || target == 0x8893 {
            glBindBuffer(target, buffer);
            gl_assert!(&format!("glBindBuffer#{:X}", target));
        } else {
            glBindBufferBase(target, 0, buffer);
            gl_assert!(&format!("glBindBufferBase#{:X}", target));
        }
        // TODO: maybe use glMapBuffer & glUnmapBuffer instead?
        glBufferData(
            target,
            (vertices.len() * std::mem::size_of::<T>()) as isize,
            vertices.as_ptr() as *const _,
            if target == 0x8892 || target == 0x8893 {
                0x88E4 /*GL_STATIC_DRAW - never changes*/
            } else {
                0x88E8 /*GL_DYNAMIC_DRAW*/
            },
        );
        gl_assert!("glBufferData");
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
        gl_assert!("glGetUniformLocation");
        assert!(handle > -1);
        transforms.push(handle);
    }

    let graphic = if builder.graphic {
        let tsc_translate = unsafe {
            glGetUniformLocation(
                program,
                "tsc_translate\0".as_ptr() as *const _ as *const _,
            )
        };
        gl_assert!("glGetUniformLocation#tsc_translate");
        assert!(tsc_translate > -1);

        let tsc_scale = unsafe {
            glGetUniformLocation(
                program,
                "tsc_scale\0".as_ptr() as *const _ as *const _,
            )
        };
        gl_assert!("glGetUniformLocation#tsc_scale");
        assert!(tsc_scale > -1);

        Some((tsc_translate, tsc_scale))
    } else {
        None
    };

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

    Shader {
        program,
        gradient: builder.gradient,
        graphic,
        transforms,
        depth,
        tint,
        blending: builder.blend,
        instance_count: builder.instance_count,
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

#[cfg(unix)]
pub(super) fn new(window: &mut Window) -> Option<Box<dyn Draw>> {
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
                /*EGL_WINDOW_BIT:*/ 0x04, /*EGL_RED_SIZE:*/ 0x3024,
                8, /*EGL_GREEN_SIZE:*/ 0x3023, 8,
                /*EGL_BLUE_SIZE:*/ 0x3022, 8,
//                /*EGL_ALPHA_SIZE:*/ 0x3021, 8,
                /*EGL_DEPTH_SIZE*/ 0x3025, 24,
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
        depth: false,
        blending: false,
        shader: 0,
        tex_coords_id: (std::u32::MAX, [0.0, 0.0], [1.0, 1.0]),
        shape_id: std::u32::MAX,
        vaa_col: false,
        vaa_tex: false,
    };

    Some(Box::new(draw))
}
