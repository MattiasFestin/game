
use std::{collections::HashMap, path::PathBuf};

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

fn load_lib_folder(mut include: &mut glsl_include::Context, path: PathBuf, root_path: &PathBuf) {
	for entry in std::fs::read_dir(path).unwrap() {
		let entry = entry.unwrap();
		let child_path = entry.path();
		if child_path.is_file() && child_path.exists() {
			
			let filename = child_path.strip_prefix(root_path).unwrap().to_str().unwrap().replace("\\", "/").to_string();
			
			let content = std::fs::read_to_string(child_path).unwrap();

			include = include.include(filename.clone(), content.clone());
		} else if child_path.is_dir() {
			load_lib_folder(include, child_path, root_path);
		}
	}
}

pub fn load_shader(
	name: &str,
	mut shaders: ResMut<Assets<Shader>>,
	asset_server: ResMut<AssetServer>,
) -> ShaderConfigBundle {
	asset_server.watch_for_changes().unwrap();

	let cwd = std::env::current_dir().unwrap();
	let base_path = cwd.join("assets/shaders");

	let lib_folder = base_path.join("lib");

	let mut vertex = glsl_include::Context::new();
	let mut fragment = glsl_include::Context::new();
	let mut veretex_include = vertex.include("", "");
	let mut fragment_include = fragment.include("", "");

	load_lib_folder(veretex_include, lib_folder.clone(), &lib_folder.clone());
	load_lib_folder(fragment_include, lib_folder.clone(), &lib_folder.clone());

	
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
	asset_server: ResMut<AssetServer>,
	mut shader_cache: ResMut<super::ShaderCache>,
	mut pipelines: ResMut<Assets<PipelineDescriptor>>,
	mut render_graph: ResMut<RenderGraph>,
	shaders: ResMut<Assets<Shader>>,
) -> Option<RenderPipeline> {
	let name = T::struct_name();

	if shader_cache.cache.contains_key(name) {
		return Some(shader_cache.cache[name].clone());
	}

	let shader_bundle = load_shader(&name, shaders, asset_server);

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