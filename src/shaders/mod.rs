use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::pipeline::PipelineDescriptor;
use bevy::render::renderer::RenderResources;
use bevy::render::render_graph::RenderGraph;
use convert_case::{Case, Casing};

use crate::utils::reflection::Reflectable;

pub mod loader;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "945ccb13-4b67-4464-aa75-e8abc436ec29"]
pub struct MyMaterial {
    pub color: Color,
}

impl Reflectable for MyMaterial {
    fn struct_name() -> &'static str {
        return &"my_material";
    }

    fn field_names() -> &'static [&'static str] {
        return &["color"];
    }
}

// reflectable!{
//     struct MyMaterial {
//         pub color: Color,
//     }
// }

pub fn setup_shader(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<MyMaterial>>,
	pipelines: ResMut<Assets<PipelineDescriptor>>,
	render_graph: ResMut<RenderGraph>,
	shaders: ResMut<Assets<Shader>>,
) {
    if let Some(render_pipeline) = loader::setup_material::<MyMaterial>(pipelines, render_graph, shaders) {
        // Create a new material
        let material = materials.add(MyMaterial {
            color: Color::rgb(0.0, 0.8, 0.0),
        });

        commands
            .spawn_bundle(MeshBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
                render_pipelines: RenderPipelines::from_pipelines(vec![render_pipeline]),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            })
            .insert(material);
    }
}