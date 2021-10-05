use std::collections::HashMap;

use bevy::asset::Asset;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::renderer::RenderResources;
use bevy::render::render_graph::RenderGraph;

use crate::utils::reflection::Reflectable;

pub mod loader;

crate::resource!{
    #[uuid = "11c82e72-b7b5-433f-8fa1-440796c714aa"]
    struct MyMaterial {
        value: f32
    }
}


pub struct ShaderCache {
    pub cache: HashMap<String, RenderPipeline>
}

impl Default for ShaderCache {
    fn default() -> Self {
        Self { cache: Default::default() }
    }
}

pub fn add_shader<T:  TypeUuid + Default + RenderResources + Reflectable + Clone + Asset + Sync + Send + 'static>(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    cache: ResMut<ShaderCache>,
	mut materials: ResMut<Assets<T>>,
	pipelines: ResMut<Assets<PipelineDescriptor>>,
	render_graph: ResMut<RenderGraph>,
	shaders: ResMut<Assets<Shader>>,
) {
    if let Some(render_pipeline) = loader::setup_material::<T>(cache, pipelines, render_graph, shaders) {
        commands
            .spawn_bundle(MeshBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere{ radius: 2.0, subdivisions: 2 })),
                render_pipelines: RenderPipelines::from_pipelines(vec![render_pipeline]),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(materials.add(T::default()));
            // .insert(MyMaterial { value: 0.0 });
    } else {
        info!("No shaders found for {}", MyMaterial::struct_name());
    }
}

pub fn test(
    time: Res<Time>,
    mut q: Query<&mut MyMaterial>,
) {
    if let Ok(mut x) = q.single_mut() {
        x.value = time.seconds_since_startup() as f32;
    }
}