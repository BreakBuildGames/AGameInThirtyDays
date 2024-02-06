use std::mem::MaybeUninit;

use gl_bindings::gl43::{
    self as gl, AttributeComponents, BufferTarget, BufferUsage, ElementKind, VertexAttributeKind,
};

use crate::color32;

mod attribute {
    use super::gl;

    pub const POSITION: gl::AttributeIndex = gl::AttributeIndex::new(0);
}

pub struct Renderer {
    gl: gl::Api,
    vao: gl::VertexArray,
    vbo: gl::Buffer,
    ibo: gl::Buffer,
    program: gl::Program,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            self.gl.delete_buffer(self.vbo);
            self.gl.delete_buffer(self.ibo);
            self.gl.delete_vertex_arrays(1, [self.vao].as_ptr());
            self.gl.delete_program(self.program);
        }
    }
}

mod cube {
    #[rustfmt::skip]
    pub const VERTICES: &[f32] = &[
        //front
       -0.5,  0.5, -0.5, //0: TL
       -0.5, -0.5, -0.5, //1: BL
        0.5,  0.5, -0.5, //2: TR
        0.5, -0.5, -0.5, //3: BR
        //back
       -0.5,  0.5, 0.5, //4: TL
       -0.5, -0.5, 0.5, //5: BL
        0.5,  0.5, 0.5, //6: TR
        0.5, -0.5, 0.5, //7: BR
    ];

    #[rustfmt::skip]
    pub const INDICES: &[u8] = &[
        //front
        0, 2, 1,
        2, 3, 1,
        //right
        2, 6, 3,
        6, 7, 3,
        //left
        1, 4, 0,
        1, 5, 4,
        //back
        6, 4, 7,
        7, 4, 5,
        //top
        0, 4, 2,
        6, 2, 4,
        //bottom
        1, 3, 5,
        3, 7, 5
    ];
}

fn create_shader_program(gl: &gl::Api, vextex_source: &str, fragment_source: &str) -> gl::Program {
    unsafe {
        let vs = gl.create_shader(gl::ShaderKind::VERTEX);
        let fs = gl.create_shader(gl::ShaderKind::FRAGMENT);

        gl.shader_source(
            vs,
            1,
            [vextex_source.as_ptr().cast()].as_ptr(),
            [vextex_source.len().try_into().expect("should fit")].as_ptr(),
        );
        gl.compile_shader(vs);

        gl.shader_source(
            fs,
            1,
            [fragment_source.as_ptr().cast()].as_ptr(),
            [fragment_source.len().try_into().expect("should fit")].as_ptr(),
        );
        gl.compile_shader(fs);

        let program = gl.create_program();
        gl.attach_shader(program, vs);
        gl.attach_shader(program, fs);
        gl.link_program(program);
        gl.detach_shader(program, vs);
        gl.detach_shader(program, fs);

        gl.delete_shader(vs);
        gl.delete_shader(fs);

        program
    }
}

impl Renderer {
    pub fn new(
        proc_address: &impl Fn(&str) -> *const std::ffi::c_void,
    ) -> anyhow::Result<Self, anyhow::Error> {
        let gl = unsafe { gl::Api::with_loader(proc_address) }?;

        unsafe {
            gl.enable(gl::Capability::DEBUG_OUTPUT);
            gl.debug_message_callback(Some(debug_message_callback), std::ptr::null_mut());
        }

        unsafe {
            let [r, g, b, a] = color32::Linear32::PERSIAN_INDIGO.as_rgba();
            gl.clear_color(r, g, b, a);
        }

        let vertex_array = unsafe {
            let mut vao = MaybeUninit::zeroed();
            gl.gen_vertex_arrays(1, vao.as_mut_ptr());
            vao.assume_init()
        };

        let vertex_buffer = unsafe {
            let mut vbo = MaybeUninit::zeroed();
            gl.gen_buffers(1, vbo.as_mut_ptr());
            let vbo = vbo.assume_init();

            gl.bind_buffer(BufferTarget::ARRAY_BUFFER, vbo);
            gl.buffer_data(
                BufferTarget::ARRAY_BUFFER,
                std::mem::size_of_val(cube::VERTICES)
                    .try_into()
                    .expect("vertex count should never reach the limit"),
                cube::VERTICES.as_ptr().cast(),
                BufferUsage::STATIC_DRAW,
            );

            vbo
        };

        let index_buffer = unsafe {
            let mut ibo = MaybeUninit::zeroed();
            gl.gen_buffers(1, ibo.as_mut_ptr());
            let ibo = ibo.assume_init();

            gl.bind_vertex_array(vertex_array);

            gl.bind_buffer(BufferTarget::ELEMENT_ARRAY_BUFFER, ibo);
            gl.buffer_data(
                BufferTarget::ELEMENT_ARRAY_BUFFER,
                std::mem::size_of_val(cube::INDICES)
                    .try_into()
                    .expect("element count should never reach the limit"),
                cube::INDICES.as_ptr().cast(),
                BufferUsage::STATIC_DRAW,
            );

            ibo
        };

        unsafe {
            gl.bind_vertex_array(vertex_array);
            gl.enable_vertex_attrib_array(attribute::POSITION);

            gl.vertex_attrib_format_ptr(
                attribute::POSITION,
                AttributeComponents::THREE,
                VertexAttributeKind::FLOAT,
                gl::GLboolean::FALSE,
                0,
            );

            let vbo_index = gl::BufferBindingIndex::new(0);

            gl.bind_vertex_buffer(
                vbo_index,
                vertex_buffer,
                0,
                (std::mem::size_of::<f32>() * 3)
                    .try_into()
                    .expect("should always fit"),
            );
            gl.vertex_attrib_binding(attribute::POSITION, vbo_index);
        }

        unsafe {
            gl.enable(gl::Capability::DEPTH);
            gl.disable(gl::Capability::CULL_FACE);
            gl.depth_func(gl::DepthFunc::LEQUAL);
        }

        let program = create_shader_program(&gl, VS, FS);

        Ok(Self {
            gl,
            vao: vertex_array,
            vbo: vertex_buffer,
            ibo: index_buffer,
            program,
        })
    }

    pub fn update(&mut self, dt: f32) {
        let gl = &self.gl;

        let m = glam::Mat4::from_euler(glam::EulerRot::XYZ, 0.0, dt, 0.0);
        let view =
            glam::Mat4::look_at_lh(glam::vec3(3.0, 3.0, 3.0), glam::Vec3::ZERO, glam::Vec3::Y);

        let projection = glam::Mat4::perspective_lh(0.6, 16f32 / 9f32, 0.01, 100.0);
        let view_projection = projection * view * m;

        unsafe {
            gl.clear(gl::ClearMask::ALL);
            gl.use_program(self.program);

            gl.uniform_matrix4_fv(
                0,
                1,
                gl::GLboolean::FALSE,
                std::ptr::addr_of!(view_projection).cast(),
            );
            gl.bind_vertex_array(self.vao);

            gl.draw_elements(
                gl::Primitive::TRIANGLES,
                cube::INDICES
                    .len()
                    .try_into()
                    .expect("indices should fit the count"),
                ElementKind::UNSIGNED_BYTE,
                std::ptr::null(),
            );
        }
    }
}

extern "system" fn debug_message_callback(
    source: gl::DebugSource,
    kind: gl::DebugType,
    id: gl::GLuint,
    severity: gl::DebugSeverity,
    _length: gl::GLsizei,
    message: *const gl::GLchar,
    _user_param: *mut std::ffi::c_void,
) {
    let error_message = unsafe {
        std::ffi::CStr::from_ptr(message.cast())
            .to_str()
            .unwrap_or("[FAILED TO READ GL ERROR MESSAGE]")
    };

    match severity {
        gl::DebugSeverity::HIGH => log::error!("{id}: {kind} from {source}: {error_message}"),
        gl::DebugSeverity::MEDIUM => log::warn!("{id}: {kind} from {source}: {error_message}"),
        gl::DebugSeverity::LOW => log::info!("{id}: {kind} from {source}: {error_message}"),
        _ => log::trace!("{id}: {kind} from {source}: {error_message}"),
    }
}

const VS: &str = "
#version 430
layout(location = 0) in vec3 pos;
 
layout(location = 0) uniform mat4 vp;

out vec3 vertex_color;
void main() {
    vertex_color = pos + vec3(0.5, 0.5, 0.5);
    gl_Position = vp * vec4(pos.x, pos.y, pos.z, 1.0);
}";

const FS: &str = "
#version 430

in vec3 vertex_color;

out vec4 color;

void main() {
    color = vec4(vertex_color, 1.0);
}";
