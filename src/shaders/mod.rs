use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{AssetRenderResourcesNode, RenderGraph};
use bevy::render::shader::{ShaderStage, ShaderStages};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "945ccb13-4b67-4464-aa75-e8abc436ec29"]
pub struct MyMaterial {
    pub color: Color,
}

pub fn setup_shader(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("my_material.vert"))),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("my_material.frag")))),
    }));

    // Add an AssetRenderResourcesNode to our Render Graph. This will bind MyMaterial resources to
    // our shader
    render_graph.add_system_node(
        "my_material",
        AssetRenderResourcesNode::<MyMaterial>::new(true),
    );

    // Add a Render Graph edge connecting our new "my_material" node to the main pass node. This
    // ensures "my_material" runs before the main pass
    render_graph
        .add_node_edge("my_material", bevy::render::render_graph::base::node::MAIN_PASS)
        .unwrap();

    // Create a new material
    let material = materials.add(MyMaterial {
        color: Color::rgb(0.0, 0.8, 0.0),
    });

    // cube
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        })
        .insert(material);
}