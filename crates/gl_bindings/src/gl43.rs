use std::fmt::Display;

pub use super::types::{
    GLbitField, GLboolean, GLchar, GLenum, GLfloat, GLint, GLsizei, GLsizeiptr, GLuint,
};
use crate::{types::GLintptr, Error, Loader};

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Shader(GLuint);

impl Shader {}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Program(GLuint);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Sampler(GLuint);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Texture(GLuint);

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VertexArray(GLuint);
impl VertexArray {}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Buffer(GLuint);
impl Buffer {}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Capability(GLenum);
impl Capability {
    pub const DEBUG_OUTPUT: Self = Self(0x92E0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferTarget(GLenum);
impl BufferTarget {
    //GL 3.3
    pub const ARRAY_BUFFER: Self = Self(0x8892);
    pub const COPY_READ_BUFFER: Self = Self(0x8F36);
    pub const COPY_WRITE_BUFFER: Self = Self(0x8F37);
    pub const ELEMENT_ARRAY_BUFFER: Self = Self(0x8893);
    pub const UNIFORM_BUFFER: Self = Self(0x8A11);
    pub const TEXTURE_BUFFER: Self = Self(0x8C2A);

    //since 4.3
    pub const SHADER_STORAGE_BUFFER: Self = Self(0x90D2);
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferUsage(GLenum);
impl BufferUsage {
    pub const STREAM_DRAW: Self = Self(0x88E0);
    pub const STREAM_READ: Self = Self(0x88E1);
    pub const STREAM_COPY: Self = Self(0x88E2);
    pub const STATIC_DRAW: Self = Self(0x88E4);
    pub const STATIC_READ: Self = Self(0x88E5);
    pub const STATIC_COPY: Self = Self(0x88E6);
    pub const DYNAMIC_DRAW: Self = Self(0x88E8);
    pub const DYNAMIC_READ: Self = Self(0x88E9);
    pub const DYNAMIC_COPY: Self = Self(0x88EA);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VertexAttributeKind(GLenum);

impl VertexAttributeKind {
    pub const BYTE: Self = Self(0x1400);
    pub const UNSIGNED_BYTE: Self = Self(0x1401);
    pub const SHORT: Self = Self(0x1402);
    pub const UNSIGNED_SHORT: Self = Self(0x1403);
    pub const INT: Self = Self(0x1404);
    pub const UNSIGNED_INT: Self = Self(0x1405);
    pub const FLOAT: Self = Self(0x1406);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ClearMask(GLbitField);

impl ClearMask {
    pub const NONE: Self = Self(0);
    pub const COLOR: Self = Self(0x4000);
    pub const DEPTH: Self = Self(0x100);
    pub const STENCIL: Self = Self(0x400);
    //TODO: replace with const impl of bitor once it stabilizes
    pub const ALL: Self = Self(Self::COLOR.0 | Self::DEPTH.0 | Self::STENCIL.0);

    #[must_use]
    pub const fn is_some(&self) -> bool {
        self.0 != 0
    }
}

impl std::ops::BitOrAssign for ClearMask {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Primitive(GLenum);
impl Primitive {
    pub const TRIANGLES: Self = Self(0x4);
    pub const TRIANGLE_STRIP: Self = Self(0x0005);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ShaderKind(GLenum);
impl ShaderKind {
    pub const FRAGMENT: Self = Self(0x8B30);
    pub const VERTEX: Self = Self(0x8B31);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ShaderParameterName(GLenum);
impl ShaderParameterName {
    pub const COMPILE_STATUS: Self = Self(0x8B81);
    pub const INFO_LOG_LENGTH: Self = Self(0x8B84);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ProgramParameterName(GLenum);
impl ProgramParameterName {
    pub const LINK_STATUS: Self = Self(0x8B82);
    pub const INFO_LOG_LENGTH: Self = Self(0x8B84);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BufferBindingIndex(GLuint);
impl BufferBindingIndex {
    #[must_use]
    pub const fn new(index: GLuint) -> Self {
        Self(index)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AttributeIndex(GLuint);
impl AttributeIndex {
    #[must_use]
    pub const fn new(index: GLuint) -> Self {
        Self(index)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AttributeComponents(GLint);
impl AttributeComponents {
    pub const ONE: Self = Self(1);
    pub const TWO: Self = Self(2);
    pub const THREE: Self = Self(3);
    pub const FOUR: Self = Self(4);
    //Note: Intentionally omitting GL_BGRA special case until I have an actual use case for it.
    //It seems kinda wonky when it comes to support(e.g. glGetAttribiv not listing it as valid
    //return value).
}

/// Bindings to a curated subset of OpenGL 4.3
unsafe impl Send for Api {}
unsafe impl Sync for Api {}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ProgramInterfaceParameterName(GLenum);

impl ProgramInterfaceParameterName {
    pub const ACTIVE_RESOURCES: Self = Self(0x92F5);
    pub const MAX_NAME_LENGTH: Self = Self(0x92F6);
    pub const MAX_NUM_ACTIVE_VARIABLES: Self = Self(0x92F7);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ProgramInterface(GLenum);

impl ProgramInterface {
    pub const UNIFORM: Self = Self(0x92E1);
    pub const UNIFORM_BLOCK: Self = Self(0x92E2);
    pub const PROGRAM_INPUT: Self = Self(0x92E3);
    pub const PROGRAM_OUTPUT: Self = Self(0x92E4);
    pub const SHADER_STORAGE_BLOCK: Self = Self(0x92E6);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct ProgramResourceProperty(GLenum);

impl ProgramResourceProperty {
    pub const NAME_LENGTH: Self = Self(0x92F9);
    pub const ARRAY_SIZE: Self = Self(0x92FB);
    pub const KIND: Self = Self(0x92FA);
    pub const OFFSET: Self = Self(0x92FC);
    pub const BLOCK_INDEX: Self = Self(0x92FD);
    pub const LOCATION: Self = Self(0x930E);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureTarget(GLenum);

impl TextureTarget {
    pub const TEXTURE_2D: Self = Self(0x0DE1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct InternalFormat(GLenum);

impl InternalFormat {
    pub const R8: Self = Self(0x8229);
    pub const RG8: Self = Self(0x822B);
    pub const RGB8: Self = Self(0x8051);
    pub const RGBA8: Self = Self(0x8058);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureFormat(GLenum);

impl TextureFormat {
    pub const RED: Self = Self(0x1903);
    pub const RG: Self = Self(0x8227);
    pub const RGB: Self = Self(0x1907);
    pub const BGR: Self = Self(0x80E0);
    pub const RGBA: Self = Self(0x1908);
    pub const BGRA: Self = Self(0x80E1);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureDataFormat(GLenum);
impl TextureDataFormat {
    pub const U8: Self = Self(0x1401);
    pub const F32: Self = Self(0x1406);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Border(GLint);
impl Border {
    pub const ZERO: Self = Self(0);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct TextureUnit(GLenum);
impl TextureUnit {
    pub const ZERO: Self = Self(0x84C0);

    #[must_use]
    pub const fn new(unit: GLenum) -> Self {
        Self(Self::ZERO.0 + unit)
    }
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SamplerParameterName(GLenum);

impl SamplerParameterName {
    pub const TEXTURE_MAG_FILTER: Self = Self(0x2800);
    pub const TEXTURE_MIN_FILTER: Self = Self(0x2801);
    pub const TEXTURE_WRAP_S: Self = Self(0x2802);
    pub const TEXTURE_WRAP_T: Self = Self(0x2803);
}

#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct SamplerParameterValue(GLint);

impl SamplerParameterValue {
    pub const NEAREST: Self = Self(0x2600);
    pub const LINEAR: Self = Self(0x2601);
}

impl std::fmt::Display for ProgramResourceProperty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self == &Self::NAME_LENGTH {
            write!(f, "NAME_LENGTH")
        } else if self == &Self::ARRAY_SIZE {
            write!(f, "ARRAY_SIZE")
        } else if self == &Self::KIND {
            write!(f, "TYPE")
        } else if self == &Self::OFFSET {
            write!(f, "OFFSET")
        } else if self == &Self::BLOCK_INDEX {
            write!(f, "BLOCK_INDEX")
        } else if self == &Self::LOCATION {
            write!(f, "LOCATION")
        } else {
            write!(f, "UNKNOWN")
        }
    }
}

pub struct Api {
    //4.3 API
    debug_message_callback_ptr: unsafe extern "system" fn(
        callback: Option<DebugMessageCallback>,
        user_param: *mut std::ffi::c_void,
    ),
    //vertex arrays
    vertex_attrib_format_ptr: unsafe extern "system" fn(
        index: AttributeIndex,
        size: AttributeComponents,
        kind: VertexAttributeKind,
        normalized: GLboolean,
        relative_offset: GLuint,
    ),
    bind_vertex_buffer_ptr: unsafe extern "system" fn(
        binding_index: BufferBindingIndex,
        buffer: Buffer,
        offset: GLintptr,
        stride: GLsizei,
    ),
    vertex_attrib_binding_ptr:
        unsafe extern "system" fn(attib_index: AttributeIndex, binding_index: BufferBindingIndex),
    //programs
    get_program_interface_iv_ptr: unsafe extern "system" fn(
        program: Program,
        program_interface: ProgramInterface,
        pname: ProgramInterfaceParameterName,
        params: *mut GLint,
    ),
    get_program_resource_iv_ptr: unsafe extern "system" fn(
        program: Program,
        interface: ProgramInterface,
        index: GLuint,
        pop_count: GLsizei,
        props: *const ProgramResourceProperty,
        buf_size: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint,
    ),
    get_program_resource_name: unsafe extern "system" fn(
        program: Program,
        interface: ProgramInterface,
        index: GLuint,
        buf_sise: GLsizei,
        length: *const GLsizei,
        name: *mut GLchar,
    ),
    uniform1_i_ptr: unsafe extern "system" fn(location: GLint, value: GLint),
    uniform1_fv_ptr:
        unsafe extern "system" fn(location: GLint, count: GLsizei, values: *const GLfloat),
    uniform2_fv_ptr:
        unsafe extern "system" fn(location: GLint, count: GLsizei, values: *const GLfloat),
    uniform3_fv_ptr:
        unsafe extern "system" fn(location: GLint, count: GLsizei, values: *const GLfloat),
    uniform4_fv_ptr:
        unsafe extern "system" fn(location: GLint, count: GLsizei, values: *const GLfloat),
    uniform_matrix4_fv_ptr: unsafe extern "system" fn(
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        values: *const GLfloat,
    ),

    // previous versions
    //
    //state
    enable_ptr: unsafe extern "system" fn(cap: Capability),
    clear_ptr: unsafe extern "system" fn(mask: ClearMask),
    clear_color_ptr: unsafe extern "system" fn(r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat),
    viewport_ptr: unsafe extern "system" fn(x: GLint, y: GLint, width: GLsizei, height: GLsizei),
    //draw
    draw_arrays_ptr: unsafe extern "system" fn(mode: Primitive, first: GLint, count: GLsizei),
    //vertex arrays
    bind_vertex_array_ptr: unsafe extern "system" fn(array: VertexArray),
    gen_vertex_arrays_ptr: unsafe extern "system" fn(n: GLsizei, arrays: *mut VertexArray),
    enable_vertex_attrib_array_ptr: unsafe extern "system" fn(index: AttributeIndex),
    delete_vertex_arrays_ptr: unsafe extern "system" fn(n: GLsizei, arrays: *const VertexArray),
    //buffers
    gen_buffers_ptr: unsafe extern "system" fn(n: GLsizei, buffers: *mut Buffer),
    bind_buffer_ptr: unsafe extern "system" fn(target: BufferTarget, buffer: Buffer),
    buffer_data_ptr: unsafe extern "system" fn(
        target: BufferTarget,
        size: GLsizeiptr,
        data: *const std::ffi::c_void,
        usage: BufferUsage,
    ),
    buffer_sub_data_ptr: unsafe extern "system" fn(
        target: BufferTarget,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const std::ffi::c_void,
    ),
    delete_buffer_ptr: unsafe extern "system" fn(buffer: Buffer),
    //shaders
    create_shader_ptr: unsafe extern "system" fn(kind: ShaderKind) -> Shader,
    shader_source_ptr: unsafe extern "system" fn(
        shader: Shader,
        count: GLsizei,
        source: *const *const GLchar,
        lenght: *const GLint,
    ),
    compile_shader_ptr: unsafe extern "system" fn(shader: Shader),
    delete_shader_ptr: unsafe extern "system" fn(shader: Shader),
    get_shader_iv_ptr:
        unsafe extern "system" fn(shader: Shader, pname: ShaderParameterName, params: *mut GLint),
    get_shader_info_log_ptr: unsafe extern "system" fn(
        shader: Shader,
        max_length: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ),

    //programs
    create_program_ptr: unsafe extern "system" fn() -> Program,
    attach_shader_ptr: unsafe extern "system" fn(program: Program, shader: Shader),
    link_program_ptr: unsafe extern "system" fn(program: Program),
    get_program_iv_ptr: unsafe extern "system" fn(
        program: Program,
        pname: ProgramParameterName,
        params: *mut GLint,
    ),
    get_program_info_log_ptr: unsafe extern "system" fn(
        program: Program,
        max_length: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ),
    detach_shader_ptr: unsafe extern "system" fn(program: Program, Shader: Shader),
    use_program_ptr: unsafe extern "system" fn(program: Program),
    delete_program_ptr: unsafe extern "system" fn(program: Program),
    //textures
    gen_textures_ptr: unsafe extern "system" fn(n: GLsizei, textures: *mut Texture),
    active_texture_ptr: unsafe extern "system" fn(unit: TextureUnit),
    bind_texture_ptr: unsafe extern "system" fn(target: TextureTarget, texture: Texture),
    tex_image_2d_ptr: unsafe extern "system" fn(
        target: TextureTarget,
        level: GLint,
        internal_format: InternalFormat,
        width: GLsizei,
        height: GLsizei,
        border: Border,
        format: TextureFormat,
        kind: TextureDataFormat,
        data: *const std::ffi::c_void,
    ),
    delete_textures_ptr: unsafe extern "system" fn(n: GLsizei, *const Texture),
    //samplers
    gen_samplers_ptr: unsafe extern "system" fn(n: GLsizei, samplers: *mut Sampler),
    bind_samplers_ptr: unsafe extern "system" fn(unit: GLuint, sampler: Sampler),
    sampler_parameter_i_ptr: unsafe extern "system" fn(
        sampler: Sampler,
        pname: SamplerParameterName,
        pvalue: SamplerParameterValue,
    ),
    delete_samplers_ptr: unsafe extern "system" fn(n: GLsizei, samplers: *const Sampler),
}

impl Api {
    /// Loads all function pointers using a context function loader,
    /// also called `ProcAddress`. Works with GLFW, SDL2 any pretty much any OpenGL context.
    ///
    /// # Errors
    /// This function will return an error if any function pointer returns a null pointer.
    ///
    /// # Safety
    /// Unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers.
    ///
    pub unsafe fn with_loader(
        loader: &impl Fn(&str) -> *const std::ffi::c_void,
    ) -> Result<Self, Error> {
        Ok(Self {
            // OpenGL 4.3
            debug_message_callback_ptr: loader.load("glDebugMessageCallback")?,

            //vertex arrays
            vertex_attrib_format_ptr: loader.load("glVertexAttribFormat")?,
            bind_vertex_buffer_ptr: loader.load("glBindVertexBuffer")?,
            vertex_attrib_binding_ptr: loader.load("glVertexAttribBinding")?,

            //programs
            get_program_resource_iv_ptr: loader.load("glGetProgramResourceiv")?,
            get_program_interface_iv_ptr: loader.load("glGetProgramInterfaceiv")?,
            get_program_resource_name: loader.load("glGetProgramResourceName")?,

            // OpenGL 3.3
            //
            //state
            enable_ptr: loader.load("glEnable")?,
            clear_ptr: loader.load("glClear")?,
            clear_color_ptr: loader.load("glClearColor")?,
            viewport_ptr: loader.load("glViewport")?,
            //draw
            draw_arrays_ptr: loader.load("glDrawArrays")?,
            //vertex arrays
            gen_vertex_arrays_ptr: loader.load("glGenVertexArrays")?,
            bind_vertex_array_ptr: loader.load("glBindVertexArray")?,
            enable_vertex_attrib_array_ptr: loader.load("glEnableVertexAttribArray")?,
            delete_vertex_arrays_ptr: loader.load("glDeleteVertexArrays")?,
            //buffers
            gen_buffers_ptr: loader.load("glGenBuffers")?,
            bind_buffer_ptr: loader.load("glBindBuffer")?,
            buffer_data_ptr: loader.load("glBufferData")?,
            buffer_sub_data_ptr: loader.load("glBufferSubData")?,
            delete_buffer_ptr: loader.load("glDeleteBuffer")?,
            //shaders
            create_shader_ptr: loader.load("glCreateShader")?,
            shader_source_ptr: loader.load("glShaderSource")?,
            compile_shader_ptr: loader.load("glCompileShader")?,
            delete_shader_ptr: loader.load("glDeleteShader")?,
            get_shader_iv_ptr: loader.load("glGetShaderiv")?,
            get_shader_info_log_ptr: loader.load("glGetShaderInfoLog")?,
            //program
            create_program_ptr: loader.load("glCreateProgram")?,
            attach_shader_ptr: loader.load("glAttachShader")?,
            link_program_ptr: loader.load("glLinkProgram")?,
            get_program_iv_ptr: loader.load("glGetProgramiv")?,
            get_program_info_log_ptr: loader.load("glGetProgramInfoLog")?,
            use_program_ptr: loader.load("glUseProgram")?,
            uniform1_i_ptr: loader.load("glUniform1i")?,
            uniform1_fv_ptr: loader.load("glUniform1fv")?,
            uniform2_fv_ptr: loader.load("glUniform2fv")?,
            uniform3_fv_ptr: loader.load("glUniform3fv")?,
            uniform4_fv_ptr: loader.load("glUniform4fv")?,
            uniform_matrix4_fv_ptr: loader.load("glUniformMatrix4fv")?,
            detach_shader_ptr: loader.load("glDetachShader")?,
            delete_program_ptr: loader.load("glDeleteProgram")?,
            //textures
            gen_textures_ptr: loader.load("glGenTextures")?,
            active_texture_ptr: loader.load("glActiveTexture")?,
            bind_texture_ptr: loader.load("glBindTexture")?,
            tex_image_2d_ptr: loader.load("glTexImage2D")?,
            delete_textures_ptr: loader.load("glDeleteTexture")?,

            //samplers
            gen_samplers_ptr: loader.load("glGenSamplers")?,
            bind_samplers_ptr: loader.load("glBindSampler")?,
            sampler_parameter_i_ptr: loader.load("glSamplerParameteri")?,
            delete_samplers_ptr: loader.load("glDeleteSamplers")?,
        })
    }

    /// Set the debug message callback.
    /// Make sure to `enable` `Capability::DEBUG` and to use a debug context.
    ///
    /// # Safety
    /// The caller has to make sure that the `DebugMessageCallback` and `user_data` will be valid for the
    /// entirety they are bound.
    ///
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn debug_message_callback(
        &self,
        callback: Option<DebugMessageCallback>,
        user_data: *mut std::ffi::c_void,
    ) {
        unsafe { (self.debug_message_callback_ptr)(callback, user_data) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the fuctions.
    ///
    /// Also, unfortunately, some drivers return wrongaddresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn vertex_attrib_format_ptr(
        &self,
        index: AttributeIndex,
        components: AttributeComponents,
        kind: VertexAttributeKind,
        normalized: GLboolean,
        local_offset: GLuint,
    ) {
        (self.vertex_attrib_format_ptr)(index, components, kind, normalized, local_offset);
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn bind_vertex_buffer(
        &self,
        binding_index: BufferBindingIndex,
        buffer: Buffer,
        offset: GLintptr,
        stride: GLsizei,
    ) {
        (self.bind_vertex_buffer_ptr)(binding_index, buffer, offset, stride);
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn vertex_attrib_binding(
        &self,
        attribute_index: AttributeIndex,
        binding_index: BufferBindingIndex,
    ) {
        (self.vertex_attrib_binding_ptr)(attribute_index, binding_index);
    }

    // OpenGL 3.3

    /// Enables certain state or context capabilities.
    ///
    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    // STATE
    #[inline]
    pub unsafe fn enable(&self, cap: Capability) {
        unsafe { (self.enable_ptr)(cap) }
    }

    /// Sets the clear color
    ///
    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn clear_color(&self, r: GLfloat, g: GLfloat, b: GLfloat, a: GLfloat) {
        unsafe { (self.clear_color_ptr)(r, g, b, a) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn clear(&self, mask: ClearMask) {
        unsafe { (self.clear_ptr)(mask) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn viewport(&self, x: GLint, y: GLint, width: GLsizei, height: GLsizei) {
        unsafe { (self.viewport_ptr)(x, y, width, height) }
    }

    // DRAW
    //
    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn draw_arrays(&self, mode: Primitive, start: GLint, count: GLsizei) {
        unsafe { (self.draw_arrays_ptr)(mode, start, count) }
    }

    // VERTEX ARRAYS
    //
    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn gen_vertex_arrays(&self, n: GLsizei, arrays: *mut VertexArray) {
        unsafe { (self.gen_vertex_arrays_ptr)(n, arrays) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn bind_vertex_array(&self, array: VertexArray) {
        unsafe { (self.bind_vertex_array_ptr)(array) };
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn enable_vertex_attrib_array(&self, index: AttributeIndex) {
        unsafe { (self.enable_vertex_attrib_array_ptr)(index) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn delete_vertex_arrays(&self, n: GLsizei, arrays: *const VertexArray) {
        unsafe { (self.delete_vertex_arrays_ptr)(n, arrays) }
    }

    // BUFFERS

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn gen_buffers(&self, n: GLsizei, buffers: *mut Buffer) {
        unsafe { (self.gen_buffers_ptr)(n, buffers) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn bind_buffer(&self, target: BufferTarget, buffer: Buffer) {
        unsafe { (self.bind_buffer_ptr)(target, buffer) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn buffer_data(
        &self,
        target: BufferTarget,
        size: GLsizeiptr,
        data: *const std::ffi::c_void,
        usage: BufferUsage,
    ) {
        unsafe { (self.buffer_data_ptr)(target, size, data, usage) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn buffer_sub_data(
        &self,
        target: BufferTarget,
        offset: GLintptr,
        size: GLsizeiptr,
        data: *const std::ffi::c_void,
    ) {
        unsafe { (self.buffer_sub_data_ptr)(target, offset, size, data) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn delete_buffer(&self, buffer: Buffer) {
        unsafe { (self.delete_buffer_ptr)(buffer) }
    }

    // SHADERS

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    #[must_use]
    pub unsafe fn create_shader(&self, kind: ShaderKind) -> Shader {
        unsafe { (self.create_shader_ptr)(kind) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn shader_source(
        &self,
        shader: Shader,
        count: GLsizei,
        source: *const *const GLchar,
        length: *const GLint,
    ) {
        unsafe { (self.shader_source_ptr)(shader, count, source, length) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn get_shader_iv(
        &self,
        shader: Shader,
        pname: ShaderParameterName,
        params: *mut GLint,
    ) {
        unsafe { (self.get_shader_iv_ptr)(shader, pname, params) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn get_shader_info_log(
        &self,
        shader: Shader,
        max_length: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ) {
        unsafe { (self.get_shader_info_log_ptr)(shader, max_length, length, info_log) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn compile_shader(&self, shader: Shader) {
        unsafe { (self.compile_shader_ptr)(shader) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn delete_shader(&self, shader: Shader) {
        unsafe { (self.delete_shader_ptr)(shader) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    #[must_use]
    pub unsafe fn create_program(&self) -> Program {
        unsafe { (self.create_program_ptr)() }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn attach_shader(&self, program: Program, shader: Shader) {
        unsafe { (self.attach_shader_ptr)(program, shader) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn link_program(&self, program: Program) {
        unsafe { (self.link_program_ptr)(program) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn get_program_iv(
        &self,
        program: Program,
        pname: ProgramParameterName,
        params: *mut GLint,
    ) {
        unsafe { (self.get_program_iv_ptr)(program, pname, params) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn get_program_info_log(
        &self,
        program: Program,
        max_length: GLsizei,
        length: *mut GLsizei,
        info_log: *mut GLchar,
    ) {
        unsafe { (self.get_program_info_log_ptr)(program, max_length, length, info_log) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn get_program_interface_iv(
        &self,
        program: Program,
        interface: ProgramInterface,
        pname: ProgramInterfaceParameterName,
        params: *mut GLint,
    ) {
        unsafe { (self.get_program_interface_iv_ptr)(program, interface, pname, params) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn get_program_resource_iv(
        &self,
        program: Program,
        interface: ProgramInterface,
        index: GLuint,
        prop_count: GLsizei,
        props: *const ProgramResourceProperty,
        buf_size: GLsizei,
        length: *mut GLsizei,
        params: *mut GLint,
    ) {
        unsafe {
            (self.get_program_resource_iv_ptr)(
                program, interface, index, prop_count, props, buf_size, length, params,
            );
        }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn get_program_resource_name(
        &self,
        program: Program,
        interface: ProgramInterface,
        index: GLuint,
        buf_size: GLsizei,
        length: *mut GLsizei,
        name: *mut GLchar,
    ) {
        unsafe {
            (self.get_program_resource_name)(program, interface, index, buf_size, length, name);
        }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn detach_shader(&self, program: Program, shader: Shader) {
        unsafe { (self.detach_shader_ptr)(program, shader) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn use_program(&self, program: Program) {
        unsafe { (self.use_program_ptr)(program) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn uniform1_i(&self, location: GLint, value: GLint) {
        unsafe { (self.uniform1_i_ptr)(location, value) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn uniform1_fv(&self, location: GLint, count: GLsizei, values: *const GLfloat) {
        unsafe { (self.uniform1_fv_ptr)(location, count, values) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn uniform2_fv(&self, location: GLint, count: GLsizei, values: *const GLfloat) {
        unsafe { (self.uniform2_fv_ptr)(location, count, values) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn uniform3_fv(&self, location: GLint, count: GLsizei, values: *const GLfloat) {
        unsafe { (self.uniform3_fv_ptr)(location, count, values) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn uniform4_fv(&self, location: GLint, count: GLsizei, values: *const GLfloat) {
        unsafe { (self.uniform4_fv_ptr)(location, count, values) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn uniform_matrix4_fv(
        &self,
        location: GLint,
        count: GLsizei,
        transpose: GLboolean,
        values: *const GLfloat,
    ) {
        unsafe { (self.uniform_matrix4_fv_ptr)(location, count, transpose, values) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn delete_program(&self, program: Program) {
        unsafe { (self.delete_program_ptr)(program) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn gen_textures(&self, n: GLsizei, textures: *mut Texture) {
        unsafe { (self.gen_textures_ptr)(n, textures) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn active_texture(&self, unit: TextureUnit) {
        unsafe { (self.active_texture_ptr)(unit) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn bind_texture(&self, target: TextureTarget, texture: Texture) {
        unsafe { (self.bind_texture_ptr)(target, texture) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub unsafe fn tex_image_2d(
        &self,
        target: TextureTarget,
        level: GLint,
        internal_format: InternalFormat,
        width: GLsizei,
        height: GLsizei,
        border: Border,
        format: TextureFormat,
        kind: TextureDataFormat,
        data: *const std::ffi::c_void,
    ) {
        unsafe {
            (self.tex_image_2d_ptr)(
                target,
                level,
                internal_format,
                width,
                height,
                border,
                format,
                kind,
                data,
            );
        }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn delete_textures(&self, n: GLsizei, textures: *const Texture) {
        unsafe { (self.delete_textures_ptr)(n, textures) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn gen_samplers(&self, n: GLsizei, samplers: *mut Sampler) {
        unsafe { (self.gen_samplers_ptr)(n, samplers) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn bind_sampler(&self, unit: u32, sampler: Sampler) {
        unsafe { (self.bind_samplers_ptr)(unit, sampler) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn sampler_parameter_i(
        &self,
        sampler: Sampler,
        pname: SamplerParameterName,
        pvalue: SamplerParameterValue,
    ) {
        unsafe { (self.sampler_parameter_i_ptr)(sampler, pname, pvalue) }
    }

    /// # Safety
    /// The caller has to make sure that the OpenGL context that loaded the functions
    /// is made current for the thread calling the functions.
    ///
    /// Also, unfortunately, some drivers return wrong addresses that are indistinguishable from correct
    /// ones, instead of being null pointers. That means if the context doesn't support certain
    /// OpenGL functions, there is no way to figure that out during load time.
    #[inline]
    pub unsafe fn delete_samplers(&self, n: GLsizei, samplers: *const Sampler) {
        unsafe { (self.delete_samplers_ptr)(n, samplers) }
    }
}

type DebugMessageCallback = extern "system" fn(
    source: DebugSource,
    kind: DebugType,
    id: GLuint,
    severity: DebugSeverity,
    length: GLsizei,
    message: *const GLchar,
    user_param: *mut std::ffi::c_void,
);

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugSource(GLenum);

impl DebugSource {
    pub const API: Self = Self(0x8246);
    pub const WINDOW_SYSTEM: Self = Self(0x8247);
    pub const SHADER_COMPILER: Self = Self(0x8248);
    pub const THIRD_PARTY: Self = Self(0x8249);
    pub const APPLICATION: Self = Self(0x824A);
    pub const OTHER: Self = Self(0x824B);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugType(GLenum);

impl DebugType {
    pub const ERROR: Self = Self(0x824C);
    pub const DEPRECATED_BEHAVIOUR: Self = Self(0x824D);
    pub const UNDEFINED_BEHAVIOUR: Self = Self(0x824E);
    pub const PORTABILITY: Self = Self(0x824F);
    pub const PERFORMANCE: Self = Self(0x8250);
    pub const OTHER: Self = Self(0x8251);
}

#[repr(transparent)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DebugSeverity(GLenum);

impl DebugSeverity {
    pub const HIGH: Self = Self(0x9146);
    pub const MEDIUM: Self = Self(0x9147);
    pub const LOW: Self = Self(0x9148);
    pub const NOTIFICATION: Self = Self(0x826B);
}

impl Display for DebugSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::SHADER_COMPILER => write!(f, "SHADER_COMPILER"),
            Self::API => write!(f, "API"),
            Self::OTHER => write!(f, "OTHER"),
            Self::THIRD_PARTY => write!(f, "THIRD_PARTY"),
            Self::APPLICATION => write!(f, "APPLICATION"),
            Self::WINDOW_SYSTEM => write!(f, "WINDOW_SYSTEM"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}

impl Display for DebugType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::OTHER => write!(f, "OTHER"),
            Self::ERROR => write!(f, "ERROR"),
            Self::PORTABILITY => write!(f, "PORTABILITY"),
            Self::PERFORMANCE => write!(f, "PERFORMANCE"),
            Self::UNDEFINED_BEHAVIOUR => write!(f, "UNDEFINED_BEHAVIOUR"),
            Self::DEPRECATED_BEHAVIOUR => write!(f, "DEPRECATED"),
            _ => write!(f, "UNKNOWN"),
        }
    }
}
