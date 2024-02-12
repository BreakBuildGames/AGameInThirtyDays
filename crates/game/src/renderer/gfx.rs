use std::ops::Range;

use gl_bindings::gl43;

use crate::color32;

#[derive(Clone, Copy, Debug)]
pub struct Sampler {
    pub min_filter: MinFilter,
    pub mag_filter: MagFilter,
    pub wrap_u: Wrap,
    pub wrap_v: Wrap,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum MagFilter {
    #[default]
    Linear,
    Nearest,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum MinFilter {
    #[default]
    Linear,
    Nearest,
    NearestMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
    LinearMipmapNearest,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum Wrap {
    Repeat,
    #[default]
    Clamp,
    Mirrored,
}

#[derive(Clone, Copy, Debug)]
pub struct Material {
    pub metallic_roughness: MetallicRoughness,
    pub normals: Option<Normals>,
    pub occlusion: Option<Occlusion>,
    pub emssissive: Option<Emissive>,
}

#[derive(Clone, Copy, Debug)]
pub struct Emissive {
    pub color: color32::Linear32,
    pub texture: Texture,
}

#[derive(Clone, Copy, Debug)]
pub struct Occlusion {
    pub strength: f32,
    pub texture: Texture,
}

#[derive(Clone, Copy, Debug)]
pub struct Normals {
    pub scale: f32,
    pub texture: Texture,
}

#[derive(Clone, Copy, Debug)]
pub struct MetallicRoughness {
    pub basecolor_factor: color32::Linear32,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub basecolor_texture: Option<Texture>,
    pub metallic_roughness_texture: Option<Texture>,
}

#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub index: usize,
    pub uv: u8,
}

pub struct TextureImage {
    pub image: usize,
    pub sampler: Option<usize>,
}

#[derive(Clone, Debug)]
pub struct Image {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub format: ImageFormat,
}

#[derive(Clone, Copy, Debug)]
pub enum ImageFormat {
    R8,
    Rg8,
    Rgb8,
    Rgba8,
    R16,
    Rg16,
    Rgb16,
    Rgba16,
    Rf32,
    RgF32,
    RgbF32,
    RgbaF32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttributeSemantic {
    Position,
    UV(u8),
}

#[derive(Clone, Copy, Debug)]
pub enum AttributeKind {
    Vec2,
    Vec3,
    Vec4,
}

impl AttributeKind {
    pub const fn component_size(self) -> usize {
        match self {
            AttributeKind::Vec2 | AttributeKind::Vec3 | AttributeKind::Vec4 => {
                std::mem::size_of::<f32>()
            }
        }
    }

    pub fn size(self) -> usize {
        self.component_size() * usize::from(self.components())
    }

    pub const fn components(self) -> u8 {
        match self {
            Self::Vec2 => 2,
            Self::Vec3 => 3,
            Self::Vec4 => 4,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Indices {
    pub kind: IndexKind,
    pub buffer: usize,
    pub count: usize,
    pub data: Vec<u8>,
}

#[derive(Clone, Copy, Debug)]
pub enum IndexKind {
    U8,
    U16,
    U32,
}

#[derive(Clone, Copy, Debug)]
pub enum Primitive {
    Triangles,
    TriangleStrips,
    TriangleFan,
}

#[derive(Clone, Debug)]
pub struct Vertices {
    pub count: usize,
    pub buffers: Vec<Range<usize>>,
}

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub semantic: AttributeSemantic,
    pub location: u8,
    pub kind: AttributeKind,
    pub buffer: usize,
    pub offset: usize,
    pub normalized: bool,
}

#[derive(Clone, Debug)]
pub struct VertexLayout {
    pub attributes: Vec<VertexAttribute>,
    pub buffers: Vec<VertexBuffer>,
}

impl VertexLayout {
    pub fn attribute(&self, semantic: &AttributeSemantic) -> Option<&VertexAttribute> {
        self.attributes
            .iter()
            .find(|attr| attr.semantic == *semantic)
    }

    pub fn buffer_vertex_size(&self, buffer: usize) -> usize {
        self.attributes.iter().fold(0, |size, attr| {
            if attr.buffer == buffer {
                size + attr.kind.size()
            } else {
                size
            }
        })
    }
}

#[derive(Clone, Debug)]
pub struct VertexBuffer {
    pub buffer: usize,
    pub stride: Stride,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Stride {
    Packed,
    Inverleaved(usize),
}

impl Default for VertexLayout {
    fn default() -> Self {
        let attributes = vec![
            VertexAttribute {
                semantic: AttributeSemantic::Position,
                kind: AttributeKind::Vec3,
                buffer: 0,
                offset: 0,
                normalized: false,
                location: 0,
            },
            VertexAttribute {
                semantic: AttributeSemantic::UV(0),
                kind: AttributeKind::Vec2,
                buffer: 0,
                offset: std::mem::size_of::<f32>() * 3,
                normalized: false,
                location: 1,
            },
        ];

        let buffers = vec![VertexBuffer {
            buffer: 0,
            stride: Stride::Inverleaved(
                attributes
                    .iter()
                    .fold(0, |stride, attr| stride + attr.kind.size()),
            ),
        }];

        // validate buffer count
        for attr in &attributes {
            assert!(attr.buffer <= buffers.len(), "invalid vertex layout");
        }

        Self {
            attributes,
            buffers,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Mesh {
    pub name: Option<String>,
    pub mode: Primitive,
    pub material: Option<usize>,
    pub indices: Option<Indices>,
    pub vertices: Vertices,
}

pub struct Scene {
    pub meshes: Option<Vec<Mesh>>,
    pub images: Option<Vec<Image>>,
    pub textures: Option<Vec<TextureImage>>,
    pub materials: Option<Vec<Material>>,
    pub samplers: Option<Vec<Sampler>>,
    pub data: Vec<u8>,
}

impl From<AttributeKind> for gl43::VertexAttributeKind {
    fn from(value: AttributeKind) -> Self {
        match value {
            AttributeKind::Vec3 | AttributeKind::Vec4 | AttributeKind::Vec2 => {
                gl43::VertexAttributeKind::FLOAT
            }
        }
    }
}
