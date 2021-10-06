
use std::collections::HashMap;

use bevy::{prelude::*, reflect::TypeUuid, render::{pipeline::{PipelineDescriptor, RenderPipeline}, render_graph::{AssetRenderResourcesNode, RenderGraph, RenderResourcesNode}, renderer::RenderResources, shader::{ShaderStage, ShaderStages}}};
use crate::utils::reflection::Reflectable;


#[derive(Debug)]
pub struct ShaderConfig {
	name: String,
	pub source: String,
	pub shader: Handle<Shader>
}

#[derive(new, Debug)]
pub struct ShaderConfigBundle {
	#[new(value = "None")]
	pub vertex: Option<ShaderConfig>,
	#[new(value = "None")]
	pub fragment: Option<ShaderConfig>,
	#[new(value = "None")]
	pub compute: Option<ShaderConfig>,
}

pub fn load_shader(
	name: &str,
	mut shaders: ResMut<Assets<Shader>>,
) -> ShaderConfigBundle {
	let cwd = std::env::current_dir().unwrap();
	let base_path = cwd.join("assets/shaders");

	let lib_folder = base_path.join("lib");

	let mut vertex = glsl_include::Context::new();
	let mut fragment = glsl_include::Context::new();
	let mut veretex_include = vertex.include("", "");
	let mut fragment_include = fragment.include("", "");

	for entry in std::fs::read_dir(lib_folder).unwrap() {
		let entry = entry.unwrap();
		let path = entry.path();
		if path.is_file() && path.exists() {
			let filename = path.file_name().unwrap().to_str().unwrap().to_string();
			
			let content = std::fs::read_to_string(path).unwrap();

			veretex_include = veretex_include.include(filename.clone(), content.clone());
			fragment_include = fragment_include.include(filename, content);
		}
	}

	
	let mut shader_bundle = ShaderConfigBundle::new();
	
	let vertex_path = base_path.join(format!("{0}/{0}.vert", name));
	if vertex_path.exists() {
		let vertex_source = std::fs::read_to_string(vertex_path).unwrap();
		let vertex_source = veretex_include.expand(vertex_source).unwrap();

		shader_bundle.vertex = Some(ShaderConfig {
			name: format!("{}.vert", name),
			source: vertex_source.clone(),
			shader: shaders.add(Shader::from_glsl(ShaderStage::Vertex, &vertex_source))
		});
	}
	

	let fragment_path = base_path.join(format!("{0}/{0}.frag", name));
	if fragment_path.exists() {
		let fragment_source = std::fs::read_to_string(fragment_path).unwrap();
		let fragment_source = fragment_include.expand(fragment_source).unwrap();

		info!("{}", fragment_source.clone());

		shader_bundle.fragment = Some(ShaderConfig {
			name: format!("{}.frag", name),
			source: fragment_source.clone(),
			shader: shaders.add(Shader::from_glsl(ShaderStage::Fragment, &fragment_source))
		});
	}

	let compute_path = base_path.join(format!("{0}/{0}.comp", name));
	if compute_path.exists() {
		let compute_source = std::fs::read_to_string(compute_path).unwrap();
		let compute_source = fragment_include.expand(compute_source).unwrap();

		shader_bundle.compute = Some(ShaderConfig {
			name: format!("{}.comp", name),
			source: compute_source.clone(),
			shader: shaders.add(Shader::from_glsl(ShaderStage::Compute, &compute_source))
		});
	}

	return shader_bundle;
}

pub fn setup_material<T: TypeUuid + RenderResources + Reflectable>(
	mut shader_cache: ResMut<super::ShaderCache>,
	mut pipelines: ResMut<Assets<PipelineDescriptor>>,
	mut render_graph: ResMut<RenderGraph>,
	shaders: ResMut<Assets<Shader>>,
) -> Option<RenderPipeline> {
	let name = T::struct_name();

	if shader_cache.cache.contains_key(name) {
		return Some(shader_cache.cache[name].clone());
	}

	let shader_bundle = load_shader(&name, shaders);

    if shader_bundle.vertex.is_some() {
        let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
            vertex: shader_bundle.vertex.unwrap().shader,
            fragment: shader_bundle.fragment.map(|x| x.shader)
        }));

        render_graph.add_system_node( 
            name,
            RenderResourcesNode::<T>::new(true),
        );

        render_graph
            .add_node_edge(name, bevy::render::render_graph::base::node::MAIN_PASS)
            .unwrap();


		let render_pipe = RenderPipeline::new(
			pipeline_handle,
		);

		shader_cache.cache.insert(name.to_string(), render_pipe.clone());

		return Some(render_pipe);
    }

	return None;
}