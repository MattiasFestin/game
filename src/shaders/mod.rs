use std::fs;
use std::path::Path;

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{AssetRenderResourcesNode, RenderGraph};
use bevy::render::shader::{ShaderSource, ShaderStage, ShaderStages};

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "945ccb13-4b67-4464-aa75-e8abc436ec29"]
pub struct MyMaterial {
    pub color: Color,
}

pub fn setup_shader(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<MyMaterial>>,
    mut render_graph: ResMut<RenderGraph>,
    mut shaders: ResMut<Assets<Shader>>,
    // server: Res<AssetServer>,
) {

    
    // let cwd = Path::new(".");
    let cwd = std::env::current_dir().unwrap();
    let base_path = cwd.join("assets/shaders");

    

    let vertex_source = fs::read_to_string(base_path.join("my_material.vert")).unwrap();
    let fragment_source = fs::read_to_string(base_path.join("my_material.frag")).unwrap();

    let lib_source = fs::read_to_string(base_path.join("lib.glsl")).unwrap();

    //TODO: Read libs from source
    let veretex_source = glsl_include::Context::new()
        // .include("lib.glsl", &lib_source)
        .expand(vertex_source)
        .unwrap();

    let fragment_source = glsl_include::Context::new()
        .include("lib.glsl", &lib_source)
        .expand(fragment_source)
        .unwrap();

    info!("veretex_source: {}", veretex_source);
    info!("fragment_source: {}", fragment_source);

    //TODO: Crashes
    // server.watch_for_changes().unwrap();

    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, &veretex_source)),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, &fragment_source))),
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