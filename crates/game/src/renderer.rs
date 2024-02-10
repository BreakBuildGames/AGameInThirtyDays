use std::mem::MaybeUninit;

use gl_bindings::gl43::{self as gl, AttributeComponents, AttributeIndex, BufferUsage};

use ::gltf::texture;
use glam::{vec3, Vec3};

use crate::color32;

mod gfx;
mod gltf;
mod text;

pub struct Renderer {
    gl: gl::Api,
    text_renderer: text::Renderer,
    text: usize,
    vaos: Vec<gl::VertexArray>,
    vbos: Vec<gl::Buffer>,
    textures: Vec<gl::Texture>,
    samplers: Vec<gl::Sampler>,
    meshes: Vec<MeshView>,
    materials: Vec<MeshMaterial>,
}

const FONT: &[u8] = include_bytes!("../resources/recursive.ttf");

#[derive(Debug)]
struct MeshView {
    vao_index: usize,
    count: usize,
    indices: usize,
    offset: usize,
    index_buffer_index: Option<usize>,
    base_index: usize,
}

struct MeshMaterial {}

mod cube {
    #[rustfmt::skip]
    pub const VERTICES: &[f32] = &[
        //front
       0.5,  0.5, -0.5, //0: TL
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
    #[allow(clippy::too_many_lines)]
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

        let mut text_renderer = text::Renderer::new(&gl);

        let scene = gltf::Scene::load_from_memory(include_bytes!("../resources/Avocado.glb"));
        let scene =
            gltf::Scene::load_from_memory(include_bytes!("../resources/ferris/ferris3d_v1.0.glb"));
        //let scene = gltf::Scene::load_from_file("crates/game/resources/gura/scene.gltf");

        let mut buffers = Vec::with_capacity(scene.buffers().len());

        let mut vaos = Vec::with_capacity(scene.meshes().len());
        let mut meshes = Vec::with_capacity(scene.meshes().len());

        let vao = unsafe {
            let mut vao = MaybeUninit::zeroed();
            gl.gen_vertex_arrays(1, vao.as_mut_ptr());
            let vao = vao.assume_init();
            gl.bind_vertex_array(vao);
            vao
        };
        vaos.push(vao);

        let mut buffer_data: Vec<u8> = Vec::new();
        let mut index_data: Vec<u8> = Vec::new();
        let mut base_index = 0;

        for mesh in scene.meshes() {
            println!("{:?}", mesh.name);
            meshes.push(MeshView {
                vao_index: 0,
                count: mesh.indices.as_ref().unwrap().count,
                indices: index_data.len(),
                offset: buffer_data.len(),
                index_buffer_index: mesh.indices.as_ref().map(|v| 1),
                base_index,
            });

            buffer_data.extend(&mesh.position_buffer);
            index_data.extend(&mesh.indices.as_ref().unwrap().data);
            base_index += mesh.vertex_count;
        }

        let mut gl_buffer = unsafe {
            let mut buffer = MaybeUninit::zeroed();
            gl.gen_buffers(1, buffer.as_mut_ptr());
            buffer.assume_init()
        };

        unsafe {
            gl.bind_buffer(gl::BufferTarget::ARRAY_BUFFER, gl_buffer);
            gl.buffer_data(
                gl::BufferTarget::ARRAY_BUFFER,
                (buffer_data.len() * std::mem::size_of::<u8>())
                    .try_into()
                    .unwrap(),
                buffer_data.as_ptr().cast(),
                BufferUsage::STATIC_DRAW,
            );
        }
        buffers.push(gl_buffer);

        let gl_buffer = unsafe {
            let mut buffer = MaybeUninit::zeroed();
            gl.gen_buffers(1, buffer.as_mut_ptr());
            buffer.assume_init()
        };

        unsafe {
            gl.bind_buffer(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, gl_buffer);
            gl.buffer_data(
                gl::BufferTarget::ELEMENT_ARRAY_BUFFER,
                (index_data.len() * std::mem::size_of::<u8>())
                    .try_into()
                    .unwrap(),
                index_data.as_ptr().cast(),
                BufferUsage::STATIC_DRAW,
            );
        }
        buffers.push(gl_buffer);

        let location = AttributeIndex::new(0);
        let components = AttributeComponents::THREE;

        unsafe {
            gl.enable_vertex_attrib_array(location);
            gl.vertex_attrib_format_ptr(
                location,
                components,
                gl::VertexAttributeKind::FLOAT,
                gl::GLboolean::FALSE,
                0,
            );

            let buffer_binding = gl::BufferBindingIndex::new(0);
            gl.vertex_attrib_binding(location, buffer_binding);
            gl.bind_vertex_buffer(
                buffer_binding,
                buffers[0],
                0,
                (3 * std::mem::size_of::<f32>()).try_into().unwrap(),
            );
        }

        let font_handle = text_renderer.load_font_from_memory(
            &gl,
            72.0,
            FONT,
            ('a'..='z')
                .chain('A'..='Z')
                .chain('0'..='9')
                .chain(".,-_+/=()!".chars()),
        )?;

        let text = text_renderer.create_text(font_handle, Vec3::ZERO, "nrseitrnie")?;

        let textures = Vec::new();
        let samplers = Vec::new();
        let materials = Vec::new();

        Ok(Self {
            gl,
            text,
            text_renderer,
            vaos,
            vbos: buffers,
            meshes,
            textures,
            samplers,
            materials,
        })
    }

    pub fn update(&mut self, dt: f32) {
        let gl = &self.gl;

        unsafe {
            gl.clear(gl::ClearMask::ALL);
            gl.bind_vertex_array(self.vaos[0]);
        }

        for mesh in &self.meshes {
            unsafe {
                match mesh.index_buffer_index {
                    Some(index) => {
                        gl.draw_elements_base_vertex(
                            gl::Primitive::TRIANGLES,
                            mesh.count.try_into().unwrap(),
                            gl::ElementKind::UNSIGNED_SHORT,
                            mesh.indices as *const _,
                            mesh.base_index.try_into().unwrap(),
                        );
                    }
                    None => {
                        gl.draw_arrays(
                            gl::Primitive::TRIANGLES,
                            mesh.offset.try_into().unwrap(),
                            mesh.count.try_into().unwrap(),
                        );
                    }
                }
            }
        }

        self.text_renderer.update(gl);
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
layout(location = 0) in vec3 position;

layout(location = 1) uniform mat4 vp;
layout(location = 6) uniform vec4 uv;

out vec3 vertex_color;
out vec2 vertex_uv;

void main() {

    vec3 vertex = position.xyz;
    vertex_uv = uv.xy + vertex.xy * uv.zw;

    gl_Position = vec4(vertex.x, vertex.y, vertex.z, 1.0);
}";

const FS: &str = "
#version 430

layout(location = 0) uniform sampler2D sampler;
layout(location = 7) uniform vec4 in_color;

in vec3 vertex_color;
in vec2 vertex_uv;

out vec4 color;

void main() {
    color = texture(sampler, vertex_uv).r * in_color;
}";
