use std::{char, mem::MaybeUninit, ptr::addr_of};

use gl_bindings::gl43::{
    self as gl, AttributeComponents, Border, BufferTarget, BufferUsage, ElementKind,
    InternalFormat, Primitive, SamplerParameterName, SamplerParameterValue, TextureDataFormat,
    TextureFormat, TextureTarget, TextureUnit, VertexAttributeKind,
};
use glam::{vec2, vec3, vec4, Vec2};

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
    texture: gl::Texture,
    sampler: gl::Sampler,
    program: gl::Program,
    font: Font,
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

const FONT: &[u8] = include_bytes!("../resources/montserrat.ttf");

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

struct Font {
    glyphs: Vec<Glyph>,
    font_size: f32,
    white_space: f32,
}

struct Glyph {
    position: Vec2,
    bitmap_size: Vec2,
    size: Vec2,
    advance: Vec2,
    offset: Vec2,
    char: char,
}

impl Font {
    pub const ATLAS_SIZE: usize = 1024;

    pub fn glyph(&self, char: char) -> Option<&Glyph> {
        self.glyphs.iter().find(|v| v.char == char)
    }

    pub fn load_font(
        font_size: f32,
        data: &[u8],
        char_set: impl Iterator<Item = char>,
    ) -> (Self, Vec<u8>) {
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default()).unwrap();
        let white_space = font.metrics(' ', font_size).advance_width;

        let padding = 2;
        let mut atlas = vec![0; Self::ATLAS_SIZE * Self::ATLAS_SIZE];

        let mut pen_x = 0;
        let mut pen_y = 0;

        let mut glyphs = Vec::with_capacity(Self::ATLAS_SIZE);

        for char in char_set {
            let (metrics, pixel_data) = font.rasterize(char, font_size);

            if pen_x + metrics.width > Self::ATLAS_SIZE {
                pen_x = 0;
                pen_y += font_size as usize;
            }

            for x in 0..metrics.width {
                for y in 0..metrics.height {
                    atlas[pen_x + x + ((pen_y + y) * Self::ATLAS_SIZE)] =
                        pixel_data[x + y * metrics.width];
                }
            }

            let uv_x = pen_x as f32 / Self::ATLAS_SIZE as f32;
            let uv_y = pen_y as f32 / Self::ATLAS_SIZE as f32;

            let width = (metrics.width) as f32 / Self::ATLAS_SIZE as f32;
            let height = (metrics.height) as f32 / Self::ATLAS_SIZE as f32;

            glyphs.push(Glyph {
                position: vec2(uv_x, uv_y),
                advance: vec2(metrics.advance_width, font_size),
                bitmap_size: vec2(width, height),
                size: vec2(metrics.width as f32, metrics.height as f32),
                offset: vec2(metrics.xmin as f32, metrics.ymin as f32),
                char,
            });

            pen_x += metrics.width + padding;
        }
        //TODO: single channl texture support
        let data = atlas.into_iter().flat_map(|v| [v, v, v]).collect();
        let font = Self {
            font_size,
            glyphs,
            white_space: white_space as f32,
        };

        (font, data)
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

        let (font, font_atlas) = Font::load_font(
            72.0,
            FONT,
            ('a'..='z').chain('A'..='Z').chain(".,-_+/=()".chars()),
        );

        let (texture, sampler) = unsafe {
            let mut texture = MaybeUninit::zeroed();
            gl.gen_textures(1, texture.as_mut_ptr());
            let texture = texture.assume_init();

            gl.active_texture(TextureUnit::ZERO);
            gl.bind_texture(TextureTarget::TEXTURE_2D, texture);
            gl.tex_image_2d(
                TextureTarget::TEXTURE_2D,
                0,
                InternalFormat::RGB8,
                Font::ATLAS_SIZE.try_into().unwrap(),
                Font::ATLAS_SIZE.try_into().unwrap(),
                Border::ZERO,
                TextureFormat::RGB,
                TextureDataFormat::U8,
                font_atlas.as_ptr().cast(),
            );

            let mut sampler = MaybeUninit::zeroed();
            gl.gen_samplers(1, sampler.as_mut_ptr());
            let sampler = sampler.assume_init();

            gl.bind_sampler(0, sampler);
            gl.sampler_parameter_i(
                sampler,
                SamplerParameterName::TEXTURE_MIN_FILTER,
                SamplerParameterValue::LINEAR,
            );

            (texture, sampler)
        };

        Ok(Self {
            gl,
            vao: vertex_array,
            vbo: vertex_buffer,
            ibo: index_buffer,
            program,
            texture,
            sampler,
            font,
        })
    }

    pub fn update(&mut self, dt: f32) {
        let gl = &self.gl;

        let text = "The quick brown fox...";

        let projection = glam::Mat4::orthographic_lh(0.0, 1024.0, 768.0, 0.0, -0.01, -100.0);
        let view = glam::Mat4::IDENTITY;

        unsafe {
            gl.clear(gl::ClearMask::ALL);
            gl.use_program(self.program);
        }

        let mut advance_x = 0.0;
        let mut prev_adv = 0.0;

        for char in text.chars() {
            if char.is_whitespace() {
                advance_x += self.font.white_space;
                continue;
            }

            let glyph = self.font.glyph(char).unwrap();

            let m = glam::Mat4::from_translation(vec3(
                advance_x + glyph.offset.x,
                self.font.font_size - glyph.size.y - glyph.offset.y,
                0.0,
            )) * glam::Mat4::from_scale(vec3(glyph.size.x, glyph.size.y, 1.0));

            let view_projection = projection * view * m;
            advance_x += glyph.advance.x;
            prev_adv = glyph.advance.x;

            unsafe {
                gl.uniform_matrix4_fv(
                    1,
                    1,
                    gl::GLboolean::FALSE,
                    std::ptr::addr_of!(view_projection).cast(),
                );
                gl.bind_vertex_array(self.vao);
                gl.uniform1_i(0, 0);

                let uv = vec4(
                    glyph.position.x,
                    glyph.position.y,
                    glyph.bitmap_size.x,
                    glyph.bitmap_size.y,
                );
                gl.uniform4_fv(6, 1, addr_of!(uv).cast());

                gl.draw_arrays(Primitive::TRIANGLE_STRIP, 0, 4);
            }
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
 
layout(location = 1) uniform mat4 vp;
layout(location = 6) uniform vec4 uv;

out vec3 vertex_color;
out vec2 vertex_uv;

void main() {

    vec2 vertices[4] = {
        vec2(0.0, 0.0),  //TL
        vec2(1.0, 0.0),  //TR
        vec2(0.0, 1.0),  //BL
        vec2(1.0, 1.0),  //BR
    };

    vec2 vertex = vertices[gl_VertexID];
    vertex_uv = uv.xy + vertex.xy * uv.zw;

    gl_Position = vp * vec4(vertex.x, vertex.y, -1.0, 1.0);
}";

const FS: &str = "
#version 430

layout(location = 0) uniform sampler2D sampler;

in vec3 vertex_color;
in vec2 vertex_uv;

out vec4 color;

void main() {
    color = texture(sampler, vertex_uv);
}";
