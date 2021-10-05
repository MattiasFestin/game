
use bevy::{prelude::*, render::shader::ShaderStage};

#[derive(Debug)]
pub struct ShaderConfig {
	name: String,
	pub source: String,
	pub shader: Handle<Shader>
}

#[derive(new, Debug)]
pub struct ShaderConfigBundle {
	pub vertex: ShaderConfig,
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

	let vertex_source = std::fs::read_to_string(base_path.join(format!("{0}/{0}.vert", name))).unwrap();
	let vertex_source = veretex_include.expand(vertex_source).unwrap();

	let mut shader_bundle = ShaderConfigBundle::new(ShaderConfig {
		name: format!("{}.vert", name),
		source: vertex_source.clone(),
		shader: shaders.add(Shader::from_glsl(ShaderStage::Vertex, &vertex_source))
	});

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