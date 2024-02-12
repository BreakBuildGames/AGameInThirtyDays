use std::{mem::MaybeUninit, ptr::addr_of};

use gl_bindings::gl43::{
    self as gl, Border, InternalFormat, Primitive, Program, SamplerParameterName,
    SamplerParameterValue, Texture, TextureDataFormat, TextureFormat, TextureTarget, TextureUnit,
    VertexArray,
};
use glam::{vec2, vec3, vec4, Vec2, Vec3};

use crate::color32;

use super::{create_shader_program, FS, VS};

pub struct Renderer {
    vao: VertexArray,
    program: Program,
    textures: Vec<Texture>,
    fonts: Vec<Font>,
    texts: Vec<Text>,
    draw_list: Vec<(usize, color32::Linear32, Vec3)>,
}

impl Renderer {
    pub fn new(gl: &gl::Api) -> Self {
        let vao = unsafe {
            let mut vao = MaybeUninit::zeroed();
            gl.gen_vertex_arrays(1, vao.as_mut_ptr());
            vao.assume_init()
        };
        let program = create_shader_program(gl, VS, FS);

        Self {
            fonts: Vec::with_capacity(10),
            texts: Vec::with_capacity(100),
            textures: Vec::with_capacity(10),
            draw_list: Vec::with_capacity(100),
            vao,
            program,
        }
    }

    pub fn load_font_from_memory(
        &mut self,
        gl: &gl::Api,
        font_size: f32,
        data: &[u8],
        char_set: impl Iterator<Item = char>,
    ) -> Result<usize, FontLoadingError> {
        let (font, atlas) = Font::load_font(font_size, data, char_set)?;
        self.fonts.push(font);

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
                atlas.as_ptr().cast(),
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
        Ok(self.fonts.len() - 1)
    }

    pub fn create_text(
        &mut self,
        font_handle: usize,
        position: Vec3,
        content: &str,
    ) -> Result<usize, CharacterNotFound> {
        let Some(font) = self.fonts.get(font_handle) else {
            return Err(todo!());
        };

        let text = Text::new(font, position, content)?;
        self.texts.push(text);
        Ok(self.texts.len() - 1)
    }

    pub fn draw(&mut self, text_handle: usize, color: color32::Linear32, position: Vec3) {
        //TODO: error handling for resources
        self.draw_list.push((text_handle, color, position));
    }

    pub fn update(&mut self, gl: &gl::Api) {
        unsafe {
            gl.enable(gl::Capability::DEPTH);
            gl.disable(gl::Capability::CULL_FACE);
            gl.depth_func(gl::DepthFunc::LEQUAL);
        }

        let projection = glam::Mat4::orthographic_lh(0.0, 1024.0, 768.0, 0.0, 0.01, 100.0);
        let view = glam::Mat4::IDENTITY;

        unsafe {
            gl.use_program(self.program);
        }

        for (handle, color, offset) in self.draw_list.drain(..) {
            let text = self.texts.get(handle).unwrap();

            for i in 0..text.positions.len() {
                let position = text.positions[i];
                let uv = text.uvs[i];
                let scale = text.scales[i];

                let model =
                    glam::Mat4::from_translation(position + offset) * glam::Mat4::from_scale(scale);

                let view_projection = projection * view * model;

                unsafe {
                    gl.uniform_matrix4_fv(
                        1,
                        1,
                        gl::GLboolean::FALSE,
                        std::ptr::addr_of!(view_projection).cast(),
                    );
                    gl.uniform4_fv(6, 1, addr_of!(uv).cast());
                    gl.uniform4_fv(7, 1, addr_of!(color).cast());
                }

                unsafe {
                    gl.bind_vertex_array(self.vao);
                    gl.uniform1_i(0, 0);
                    gl.draw_arrays(Primitive::TRIANGLE_STRIP, 0, 4);
                }
            }
        }
    }
}

pub struct Font {
    glyphs: Vec<Glyph>,
    size: f32,
    white_space: f32,
}

pub struct Text {
    global: Vec3,
    positions: Vec<glam::Vec3>,
    uvs: Vec<glam::Vec4>,
    scales: Vec<glam::Vec3>,
}

#[derive(Debug)]
pub struct CharacterNotFound(char);

#[derive(Debug)]
pub enum FontLoadingError {
    CharacterNotFound(char),
    FontInvalid,
}

impl std::error::Error for FontLoadingError {}
impl std::error::Error for CharacterNotFound {}

impl std::fmt::Display for FontLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CharacterNotFound(char) => {
                write!(f, "character {char} not found")
            }
            FontLoadingError::FontInvalid => write!(f, "invalid font"),
        }
    }
}

impl std::fmt::Display for CharacterNotFound {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "character {} not found", self.0)
    }
}

impl Text {
    pub fn new(font: &Font, position: Vec3, content: &str) -> Result<Self, CharacterNotFound> {
        let mut advance_x = 0.0;
        let mut advance_y = 0.0;

        let mut positions = Vec::with_capacity(content.len());
        let mut scales = Vec::with_capacity(content.len());
        let mut uvs = Vec::with_capacity(content.len());

        for char in content.chars() {
            if char.is_whitespace() {
                if char == ' ' {
                    advance_x += font.white_space;
                } else if char == '\n' {
                    advance_x = 0.0;
                    advance_y += font.size;
                }
                continue;
            }

            let glyph = font.glyph(char).ok_or(CharacterNotFound(char))?;

            let position = vec3(
                advance_x + glyph.offset.x,
                advance_y + font.size - glyph.size.y - glyph.offset.y,
                0.0,
            );

            let scale = vec3(glyph.size.x, glyph.size.y, 1.0);
            let uv = vec4(
                glyph.position.x,
                glyph.position.y,
                glyph.bitmap_size.x,
                glyph.bitmap_size.y,
            );

            positions.push(position);
            scales.push(scale);
            uvs.push(uv);

            advance_x += glyph.advance.x;
        }
        Ok(Self {
            positions,
            uvs,
            scales,
            global: position,
        })
    }
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

    #[allow(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss
    )]
    pub fn load_font(
        font_size: f32,
        data: &[u8],
        char_set: impl Iterator<Item = char>,
    ) -> Result<(Self, Vec<u8>), FontLoadingError> {
        let font = fontdue::Font::from_bytes(data, fontdue::FontSettings::default())
            .map_err(|e| FontLoadingError::FontInvalid)?;

        let white_space = font.metrics(' ', font_size).advance_width;

        let padding = 2;
        let mut atlas = vec![0; Self::ATLAS_SIZE * Self::ATLAS_SIZE];

        let mut pen_x = 0;
        let mut pen_y = 0;

        let mut glyphs = Vec::with_capacity(Self::ATLAS_SIZE);

        for char in char_set {
            if !font.has_glyph(char) {
                return Err(FontLoadingError::CharacterNotFound(char));
            }
            let (metrics, pixel_data) = font.rasterize(char, font_size);

            if pen_x + metrics.width > Self::ATLAS_SIZE {
                pen_x = 0;
                pen_y += font_size as usize;
            }

            for y in 0..metrics.height {
                let start = pen_x + (pen_y + y) * Self::ATLAS_SIZE;
                atlas.splice(
                    start..(start + metrics.width),
                    pixel_data[y * metrics.width..][..metrics.width]
                        .iter()
                        .copied(),
                );
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
        //TODO: single channel texture support
        let data = atlas.into_iter().flat_map(|v| [v, v, v]).collect();
        let font = Self {
            glyphs,
            size: font_size,
            white_space,
        };

        Ok((font, data))
    }
}
