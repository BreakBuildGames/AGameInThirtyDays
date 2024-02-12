use gl_bindings::gl43::{
    self as gl, AttributeComponents, AttributeIndex, BufferBindingIndex, BufferUsage,
};
use glam::Mat4;
use std::{borrow::Borrow, mem::MaybeUninit, usize};

use crate::{color32, GameState};

use self::gfx::VertexLayout;

mod gfx;
mod gltf;

mod mesh;
mod text;

pub struct Renderer {
    gl: gl::Api,
    text_renderer: text::Renderer,
    vao: gl::VertexArray,
    gl_buffers: Vec<gl::Buffer>,
    textures: Vec<gl::Texture>,
    samplers: Vec<gl::Sampler>,
    meshes: Vec<MeshView>,
    materials: Vec<MeshMaterial>,
}

const FONT: &[u8] = include_bytes!("../resources/recursive.ttf");

#[derive(Debug)]
struct MeshView {
    vertices: usize,
    vertex_offset: usize,
    indices: Option<MeshIndices>,
    base_index: usize,
}

#[derive(Debug)]
struct MeshIndices {
    count: usize,
    offset: usize,
    kind: gfx::IndexKind,
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

        gl.use_program(program);

        program
    }
}

fn create_vao(gl: &gl::Api, layout: &VertexLayout) -> gl::VertexArray {
    let vao = unsafe {
        let mut vao = MaybeUninit::zeroed();
        gl.gen_vertex_arrays(1, vao.as_mut_ptr());
        let vao = vao.assume_init();
        gl.bind_vertex_array(vao);
        vao
    };

    for attribute in &layout.attributes {
        let location = gl::AttributeIndex::new(attribute.location.into());

        unsafe {
            gl.enable_vertex_attrib_array(location);
            gl.vertex_attrib_format_ptr(
                location,
                attribute.kind.components().try_into().unwrap(),
                attribute.kind.into(),
                attribute.normalized.into(),
                attribute.offset.try_into().unwrap(),
            );

            gl.vertex_attrib_binding(
                location,
                BufferBindingIndex::new(attribute.buffer.try_into().unwrap()),
            );
        }
    }
    vao
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

        let mut text_renderer = text::Renderer::new(&gl);

        let config = gltf::Config::default();
        let vao = create_vao(&gl, &config.vertex_layout);

        //let scene =
        //gltf::load_from_memory(&config, include_bytes!("../resources/Avocado.glb")).unwrap();

        //let scene = gltf::load_from_memory(
        //&config,
        //include_bytes!("../resources/ferris/ferris3d_v1.0.glb"),
        //)
        //.unwrap();
        let scene =
            gltf::load_from_file(&config, "crates/game/resources/shibahu/scene.gltf").unwrap();

        let Some(meshes) = scene.meshes else {
            panic!();
        };

        let mut vertex_buffers: Vec<Vec<u8>> =
            Vec::with_capacity(config.vertex_layout.buffers.len());

        for i in 0..config.vertex_layout.buffers.len() {
            vertex_buffers.push(Vec::new());
        }

        let mut index_buffer: Vec<u8> = Vec::new();

        let mut base_index = 0;
        let mut mesh_handles = Vec::with_capacity(meshes.len());

        for mesh in meshes {
            println!("{:?}", mesh.name);

            mesh_handles.push(MeshView {
                base_index,
                vertices: mesh.vertices.count,
                indices: mesh.indices.as_ref().map(|indices| MeshIndices {
                    count: indices.count,
                    offset: index_buffer.len(),
                    kind: indices.kind,
                }),
                vertex_offset: 0,
            });

            for (index, buffer_range) in mesh.vertices.buffers.iter().enumerate() {
                let chunk = &scene.data[buffer_range.clone()];
                vertex_buffers[index].extend(chunk);
            }

            if let Some(index) = mesh.indices {
                index_buffer.extend(index.data);
            }

            base_index += mesh.vertices.count;
        }

        let mut gl_buffers: Vec<_> = config
            .vertex_layout
            .buffers
            .iter()
            .map(|buffer| unsafe {
                let mut gl_buffer = MaybeUninit::zeroed();
                gl.gen_buffers(1, gl_buffer.as_mut_ptr());
                let gl_buffer = gl_buffer.assume_init();

                gl.bind_buffer(gl::BufferTarget::ARRAY_BUFFER, gl_buffer);
                gl.buffer_data(
                    gl::BufferTarget::ARRAY_BUFFER,
                    (vertex_buffers[buffer.buffer].len() * std::mem::size_of::<u8>())
                        .try_into()
                        .unwrap(),
                    vertex_buffers[buffer.buffer].as_ptr().cast(),
                    BufferUsage::STATIC_DRAW,
                );

                gl_buffer
            })
            .collect();

        for b in &config.vertex_layout.buffers {
            unsafe {
                let location = gl::BufferBindingIndex::new(b.buffer.try_into().unwrap());
                gl.bind_vertex_buffer(
                    location,
                    gl_buffers[b.buffer],
                    0,
                    match b.stride {
                        gfx::Stride::Packed => config
                            .vertex_layout
                            .buffer_vertex_size(b.buffer)
                            .try_into()
                            .unwrap(),
                        gfx::Stride::Inverleaved(n) => n.try_into().unwrap(),
                    },
                );
            }
        }

        unsafe {
            let mut gl_buffer = MaybeUninit::zeroed();
            gl.gen_buffers(1, gl_buffer.as_mut_ptr());
            let gl_buffer = gl_buffer.assume_init();

            gl.bind_buffer(gl::BufferTarget::ELEMENT_ARRAY_BUFFER, gl_buffer);

            gl.buffer_data(
                gl::BufferTarget::ELEMENT_ARRAY_BUFFER,
                (index_buffer.len() * std::mem::size_of::<u8>())
                    .try_into()
                    .unwrap(),
                index_buffer.as_ptr().cast(),
                BufferUsage::STATIC_DRAW,
            );

            gl_buffers.push(gl_buffer);
        };

        let font_handle = text_renderer.load_font_from_memory(
            &gl,
            72.0,
            FONT,
            ('a'..='z')
                .chain('A'..='Z')
                .chain('0'..='9')
                .chain(".,-_+/=()!".chars()),
        )?;

        let textures = Vec::new();
        let samplers = Vec::new();
        let materials = Vec::new();

        Ok(Self {
            gl,
            text_renderer,
            vao,
            gl_buffers,
            meshes: mesh_handles,
            textures,
            samplers,
            materials,
        })
    }

    pub fn update(&mut self, dt: f32, game_state: &mut GameState) {
        let gl = &self.gl;

        unsafe {
            gl.clear(gl::ClearMask::ALL);
            gl.bind_vertex_array(self.vao);

            let vp = game_state.camera.view_projection();
            gl.uniform_matrix4_fv(1, 1, gl::GLboolean::FALSE, std::ptr::addr_of!(vp).cast());
        }

        for mesh in &self.meshes {
            unsafe {
                match &mesh.indices {
                    Some(index) => {
                        gl.draw_elements_base_vertex(
                            gl::Primitive::TRIANGLES,
                            index.count.try_into().unwrap(),
                            match index.kind {
                                gfx::IndexKind::U8 => gl::ElementKind::UNSIGNED_BYTE,
                                gfx::IndexKind::U16 => gl::ElementKind::UNSIGNED_SHORT,
                                gfx::IndexKind::U32 => gl::ElementKind::UNSIGNED_INT,
                            },
                            index.offset as *const _,
                            mesh.base_index.try_into().unwrap(),
                        );
                    }
                    None => {
                        gl.draw_arrays(
                            gl::Primitive::TRIANGLES,
                            mesh.vertex_offset.try_into().unwrap(),
                            mesh.vertices.try_into().unwrap(),
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
layout(location = 1) in vec2 uv;

layout(location = 1) uniform mat4 vp;

out vec3 vertex_color;
out vec2 vertex_uv;

void main() {

    vec3 vertex = position.xyz;
    vertex_uv = uv;

    gl_Position = vp * vec4(vertex.x, vertex.y, vertex.z, 1.0);
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
