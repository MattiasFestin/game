use std::collections::HashMap;

use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::{PipelineDescriptor, RenderPipeline};
use bevy::render::renderer::RenderResources;
use bevy::render::render_graph::RenderGraph;

use crate::utils::reflection::Reflectable;

pub mod loader;

#[derive(Default)]
pub struct ShaderCache {
    pub cache: HashMap<String, RenderPipeline>
}

pub fn register_shaders<T:  TypeUuid + Default + RenderResources + Reflectable>(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
    cache: ResMut<ShaderCache>,
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
            .insert(T::default());
    } else {
        info!("No shaders found for {}", T::struct_name());
    }
}

pub fn setup_shader(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    cache: ResMut<ShaderCache>,
	pipelines: ResMut<Assets<PipelineDescriptor>>,
	render_graph: ResMut<RenderGraph>,
	shaders: ResMut<Assets<Shader>>,
) {
    let entity = commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere{ radius: 2.0, subdivisions: 2 })),
            // render_pipelines: ,
            transform: Transform::from_xyz(10.0, 0.0, 0.0),
            ..Default::default()
        });

    add_shader::<crate::physics::BlackBody>(entity, cache, pipelines, render_graph, shaders);
}

pub fn add_shader<'a, 'b, T:  TypeUuid + Default + RenderResources + Reflectable>(
	mut commands: EntityCommands<'a, 'b>,
	// mut meshes: ResMut<Assets<Mesh>>,
    cache: ResMut<ShaderCache>,
	pipelines: ResMut<Assets<PipelineDescriptor>>,
	render_graph: ResMut<RenderGraph>,
	shaders: ResMut<Assets<Shader>>,
) {
    if let Some(render_pipeline) = loader::setup_material::<T>(cache, pipelines, render_graph, shaders) {
        commands
            .insert(RenderPipelines::from_pipelines(vec![render_pipeline]))
            .insert(T::default());
    } else {
        info!("No shaders found for {}", T::struct_name());
    }
}

// pub fn animate_shader(
//     time: Res<Time>,
//     mut q: Query<&mut crate::physics::BlackBody>,
// ) {
//     if let Ok(mut x) = q.single_mut() {
//         x.temperature = (1000.0 * time.seconds_since_startup() as f32) % 10000.0;
//     }
// }