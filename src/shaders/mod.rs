use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::render_graph::{AssetRenderResourcesNode, RenderGraph};
use bevy::render::shader::{ShaderSource, ShaderStage, ShaderStages};

pub mod loader;

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
	let shader_bundle = loader::load_shader("my_material", shaders);

	println!("{:?}", shader_bundle);

	// Create a new shader pipeline
	let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
		vertex: shader_bundle.vertex.shader,
		fragment: shader_bundle.fragment.map(|x| x.shader)
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