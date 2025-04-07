use bevy::{
    prelude::*,
    reflect::TypePath,
    // render::render_resource::{AsBindGroup, ShaderRef},
    pbr::{MaterialPipeline, MaterialPipelineKey},
    render::{
        mesh::{MeshVertexAttribute, MeshVertexBufferLayoutRef},
        render_resource::{
            AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
            VertexFormat,
        },
    },
};

const ANIMATE_SHADER_PATH: &str = "bevy_examples/shaders/animate_shader.wgsl";
const CUSTOM_SHADER_PATH: &str = "bevy_examples/shaders/custom_vertex_attribute.wgsl";

pub const ATTRIBUTE_BLEND_COLOR: MeshVertexAttribute =
    MeshVertexAttribute::new("BlendColor", 988540917, VertexFormat::Float32x4);

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    pub color: LinearRgba,
}



impl Material for CustomMaterial {
    fn vertex_shader() -> ShaderRef {
        CUSTOM_SHADER_PATH.into()
    }
    fn fragment_shader() -> ShaderRef {
        CUSTOM_SHADER_PATH.into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayoutRef,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.0.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            ATTRIBUTE_BLEND_COLOR.at_shader_location(1),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

// Define the custom material for our shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct DumbyMatrial {}

impl Material for DumbyMatrial {
    fn fragment_shader() -> ShaderRef {
        ANIMATE_SHADER_PATH.into()
    }
}