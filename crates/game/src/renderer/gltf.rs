use core::panic;
use std::path::Path;

#[derive(Debug)]
pub enum Error {
    UnsupportedIndexFormat,
    UnsupportedPrimitive,
    SparseDataNotSupported,
    VerticesOutOfBounds,
    AttributeKindNotSupported,
    NoVertexData,
}

pub struct UnsupportedAttribute(gltf::Semantic);

use crate::renderer::gfx::{Primitive, VertexAttribute};

use super::gfx::{
    AttributeKind, AttributeSemantic, Image, ImageFormat, IndexKind, Indices, MagFilter, Material,
    Mesh, MetallicRoughness, MinFilter, Sampler, Scene, Texture, TextureImage, VertexLayout, Wrap,
};

pub struct Config {
    pub vertex_layout: VertexLayout, //TODO: what to load..
}

impl Default for Config {
    fn default() -> Self {
        Self {
            vertex_layout: VertexLayout::default(),
        }
    }
}

pub fn load_from_file<P: AsRef<Path>>(config: &Config, path: P) -> Result<Scene, Error> {
    let (document, buffers, images) = gltf::import(path).unwrap();
    load(config, document, buffers, images)
}

pub fn load_from_memory(config: &Config, data: &[u8]) -> Result<Scene, Error> {
    let (document, buffers, images) = gltf::import_slice(data).unwrap();
    load(config, document, buffers, images)
}

fn load(
    config: &Config,
    document: gltf::Document,
    buffers: Vec<gltf::buffer::Data>,
    images: Vec<gltf::image::Data>,
) -> Result<Scene, Error> {
    //let images = images.into_iter().map(Into::into);
    let mut data = Vec::new();

    let meshes = process_meshes(
        &mut data,
        &config.vertex_layout,
        document.meshes(),
        &buffers,
    )?;
    //scene.process_materials(document.materials());
    //scene.process_samplers(document.samplers());
    //scene.process_textures(document.textures());

    Ok(Scene {
        meshes: Some(meshes),
        images: None,
        textures: None,
        materials: None,
        samplers: None,
        data,
    })
}

//fn process_textures(&mut self, textures: gltf::iter::Textures) {
//for texture in textures {
//self.textures.push(TextureImage {
//sampler: texture.sampler().index(),
//image: texture.source().index(),
//});
//}
//}

//fn process_samplers(&mut self, samplers: gltf::iter::Samplers) {
//for sampler in samplers {
//let mag_filter = sampler
//.mag_filter()
//.map(|s| match s {
//gltf::texture::MagFilter::Nearest => MagFilter::Nearest,
//gltf::texture::MagFilter::Linear => MagFilter::Linear,
//})
//.unwrap_or_default();
//let min_filter = sampler
//.min_filter()
//.map(|s| match s {
//gltf::texture::MinFilter::Nearest => MinFilter::Nearest,
//gltf::texture::MinFilter::Linear => MinFilter::Linear,
//gltf::texture::MinFilter::NearestMipmapNearest => MinFilter::NearestMipmapNearest,
//gltf::texture::MinFilter::LinearMipmapNearest => MinFilter::LinearMipmapNearest,
//gltf::texture::MinFilter::NearestMipmapLinear => MinFilter::NearestMipmapLinear,
//gltf::texture::MinFilter::LinearMipmapLinear => MinFilter::LinearMipmapLinear,
//})
//.unwrap_or_default();

//let wrap_u = match sampler.wrap_s() {
//gltf::texture::WrappingMode::ClampToEdge => Wrap::Clamp,
//gltf::texture::WrappingMode::MirroredRepeat => Wrap::Mirrored,
//gltf::texture::WrappingMode::Repeat => Wrap::Repeat,
//};
//let wrap_v = match sampler.wrap_t() {
//gltf::texture::WrappingMode::ClampToEdge => Wrap::Clamp,
//gltf::texture::WrappingMode::MirroredRepeat => Wrap::Mirrored,
//gltf::texture::WrappingMode::Repeat => Wrap::Repeat,
//};

//self.samplers.push(Sampler {
//min_filter,
//mag_filter,
//wrap_u,
//wrap_v,
//});
//}
//}

//fn process_materials(&mut self, materials: gltf::iter::Materials) {
//for material in materials {
//let gltf_mr = material.pbr_metallic_roughness();
//let metallic_roughness = MetallicRoughness {
//basecolor_factor: gltf_mr.base_color_factor().into(),
//metallic_factor: gltf_mr.metallic_factor(),
//roughness_factor: gltf_mr.roughness_factor(),
//basecolor_texture: gltf_mr.base_color_texture().map(|m| Texture {
//index: m.texture().index(),
//uv: m.tex_coord().try_into().unwrap_or(u8::MAX),
//}),
//metallic_roughness_texture: gltf_mr.metallic_roughness_texture().map(|m| Texture {
//index: m.texture().index(),
//uv: m.tex_coord().try_into().unwrap_or(u8::MAX),
//}),
//};

//let mat = Material {
//metallic_roughness,
//normals: None,
//occlusion: None,
//emssissive: None,
//};
//self.materials.push(mat);
//}
//}

#[allow(clippy::too_many_lines)]
fn process_meshes(
    data: &mut Vec<u8>,
    layout: &VertexLayout,
    gltf_meshes: gltf::iter::Meshes,
    data_buffers: &[gltf::buffer::Data],
) -> Result<Vec<Mesh>, Error> {
    let mut meshes = Vec::with_capacity(gltf_meshes.len());

    for gltf_mesh in gltf_meshes {
        for gltf_primitive in gltf_mesh.primitives() {
            let indices = match gltf_primitive.indices() {
                Some(indices) => {
                    if let Some(view) = indices.view() {
                        let indices = Indices {
                            kind: match indices.data_type() {
                                gltf::accessor::DataType::U8 => IndexKind::U8,
                                gltf::accessor::DataType::U16 => IndexKind::U16,
                                gltf::accessor::DataType::U32 => IndexKind::U32,
                                _ => return Err(Error::UnsupportedIndexFormat),
                            },
                            buffer: view.buffer().index(),
                            count: indices.count(),
                            data: data_buffers[view.buffer().index()]
                                [view.offset() + indices.offset()..]
                                [..(indices.count() * indices.data_type().size())]
                                .to_vec(),
                        };
                        Some(indices)
                    } else {
                        return Err(Error::SparseDataNotSupported);
                    }
                }
                None => None,
            };

            let mut count = None;
            let mut buffers = None;

            for (semantic, attribute) in gltf_primitive.attributes() {
                if let Some(view) = attribute.view() {
                    let buffers = buffers.get_or_insert_with(|| {
                        count = Some(attribute.count());
                        let mut buffers = Vec::with_capacity(layout.buffers.len());
                        for b in &layout.buffers {
                            buffers.push(vec![
                                0u8;
                                layout.buffer_vertex_size(b.buffer)
                                    * attribute.count()
                            ]);
                        }

                        buffers
                    });

                    let Ok(semantic) = semantic.try_into() else {
                        continue;
                    };

                    if let Some(vertex_attribute) = layout.attribute(&semantic) {
                        let buffer = buffers
                            .get_mut(vertex_attribute.buffer)
                            .ok_or(Error::VerticesOutOfBounds)?;

                        let buffer_view = data_buffers
                            .get(view.buffer().index())
                            .ok_or(Error::VerticesOutOfBounds)?;

                        let components = match attribute.dimensions() {
                            gltf::accessor::Dimensions::Scalar => 1,
                            gltf::accessor::Dimensions::Vec2 => 2,
                            gltf::accessor::Dimensions::Vec3 => 3,
                            gltf::accessor::Dimensions::Vec4 | gltf::accessor::Dimensions::Mat2 => {
                                4
                            }
                            gltf::accessor::Dimensions::Mat3 => 9,
                            gltf::accessor::Dimensions::Mat4 => 16,
                        };

                        if attribute.data_type() != gltf::accessor::DataType::F32 {
                            return Err(Error::AttributeKindNotSupported);
                        }

                        buffer[vertex_attribute.offset..]
                            .chunks_exact_mut(
                                match layout.buffers[vertex_attribute.buffer].stride {
                                    super::gfx::Stride::Packed => vertex_attribute.kind.size(),
                                    super::gfx::Stride::Inverleaved(n) => n,
                                },
                            )
                            .zip(
                                buffer_view[view.offset() + attribute.offset()..].chunks_exact(
                                    view.stride().unwrap_or_else(|| attribute.size()),
                                ),
                            )
                            .take(attribute.count())
                            .for_each(|(target, source)| {
                                let min_components =
                                    vertex_attribute.kind.components().min(components).into();

                                let target_comp_size = vertex_attribute.kind.component_size();
                                let source_comp_size = attribute.data_type().size();

                                for i in 0..min_components {
                                    for byte in 0..target_comp_size {
                                        target[byte + i * target_comp_size] =
                                            source[byte + i * source_comp_size];
                                    }
                                }
                            });
                    }
                }
            }
            let Some(buffers) = &buffers else {
                return Err(Error::NoVertexData);
            };

            let mut offset = 0;
            let mut ranges = Vec::with_capacity(buffers.len());

            for buffer in buffers {
                assert!(!buffer.is_empty());
                let start = offset + data.len();
                let range = start..start + buffer.len();

                data.extend(buffer);

                ranges.push(range);
                offset += buffer.len();
            }
            let vertices = super::gfx::Vertices {
                count: count.unwrap_or(0),
                buffers: ranges,
            };

            meshes.push(Mesh {
                name: gltf_mesh.name().map(Into::into),
                mode: gltf_primitive.mode().try_into()?,
                material: gltf_primitive.material().index(),
                indices,
                vertices,
            });
        }
    }
    Ok(meshes)
}

impl From<(gltf::accessor::DataType, gltf::accessor::Dimensions)> for AttributeKind {
    fn from(value: (gltf::accessor::DataType, gltf::accessor::Dimensions)) -> Self {
        use gltf::accessor::DataType as E;
        use gltf::accessor::Dimensions as D;

        match value {
            (E::F32, D::Vec2) => Self::Vec2,
            (E::F32, D::Vec3) => Self::Vec3,
            (E::F32, D::Vec4) => Self::Vec4,
            _ => unimplemented!(),
        }
    }
}

impl TryFrom<gltf::mesh::Mode> for Primitive {
    type Error = Error;

    fn try_from(value: gltf::mesh::Mode) -> Result<Self, Self::Error> {
        match value {
            gltf::mesh::Mode::Triangles => Ok(Self::Triangles),
            gltf::mesh::Mode::TriangleStrip => Ok(Self::TriangleStrips),
            gltf::mesh::Mode::TriangleFan => Ok(Self::TriangleFan),
            _ => Err(Self::Error::UnsupportedPrimitive),
        }
    }
}

impl TryFrom<gltf::Semantic> for AttributeSemantic {
    type Error = UnsupportedAttribute;

    fn try_from(value: gltf::Semantic) -> Result<Self, Self::Error> {
        use gltf::Semantic as E;
        match value {
            E::Positions => Ok(Self::Position),
            E::TexCoords(n) => Ok(Self::UV(n.try_into().unwrap_or(u8::MAX))),
            _ => Err(UnsupportedAttribute(value)),
        }
    }
}

impl From<gltf::image::Data> for Image {
    fn from(value: gltf::image::Data) -> Self {
        Self {
            data: value.pixels,
            width: value.width,
            height: value.height,
            format: match value.format {
                gltf::image::Format::R8 => ImageFormat::R8,
                gltf::image::Format::R8G8 => ImageFormat::Rg8,
                gltf::image::Format::R8G8B8 => ImageFormat::Rgb8,
                gltf::image::Format::R8G8B8A8 => ImageFormat::Rgba8,
                gltf::image::Format::R16 => ImageFormat::R16,
                gltf::image::Format::R16G16 => ImageFormat::Rg16,
                gltf::image::Format::R16G16B16 => ImageFormat::Rgb16,
                gltf::image::Format::R16G16B16A16 => ImageFormat::Rgba16,
                gltf::image::Format::R32G32B32FLOAT => ImageFormat::RgbF32,
                gltf::image::Format::R32G32B32A32FLOAT => ImageFormat::RgbaF32,
            },
        }
    }
}
