use bevy::{app::Events, prelude::*, render::{camera::PerspectiveProjection, pipeline::PipelineDescriptor, render_graph::RenderGraph}, window::WindowResized};
use bevy_rapier3d::na::ComplexField;
use crate::{camera::PlayerCamera, shaders::ShaderCache, window::WindowSize};

use bevy::core::Byteable;
use crate::utils::reflection::Reflectable;
use bevy::render::renderer::{RenderResource, RenderResources};
use bevy::reflect::*;

pub struct PathTraceScreen;

crate::resource!{
    #[uuid = "4b816f2b-f19f-4ec5-b4e4-9bb094905679"]
    struct PathTracer {
        width: f32,
        height: f32,
        time: f32,
        samples: i32,
        pathlenght: i32
    }
}

pub fn path_trace(
    time: Res<Time>,
    mut commands: Commands,
    asset_server: ResMut<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    shader_cache: ResMut<ShaderCache>,
	pipelines: ResMut<Assets<PipelineDescriptor>>,
	render_graph: ResMut<RenderGraph>,
	shaders: ResMut<Assets<Shader>>,
    window_size: Res<WindowSize>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // mut meshes: ResMut<Assets<Quad>>,
) {
    let plane = shape::Plane {
        size: 100.0
    };
    
    let mut entity = commands.spawn();

    let pos = Vec3::new(1.0, 10.0, 0.0);

    let mut transform = Transform::from_translation(pos);
    transform.rotate(Quat::from_rotation_z(90.0));

    entity
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(plane)),
            material:  materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            // global_transform: GlobalTransform::from_translation(pos),
            transform,
            ..Default::default()
        })
        .insert(PathTraceScreen);
    
    crate::shaders::add_shader::<PathTracer>(&mut entity, asset_server, shader_cache, pipelines, render_graph, shaders);

    entity.insert(PathTracer { width: window_size.width, height: window_size.height, time: time.seconds_since_startup() as f32, samples: 10, pathlenght: 10 });
}

pub fn update_pt(
    time: Res<Time>,
    // mut query: Query<WindowSize, Changed<WindowSize>>
    // window_size: Res<WindowSize>,
    mut query: Query<&mut PathTracer>,
    resize_event: Res<Events<WindowResized>>,
) {
    if let Ok(mut pt) = query.single_mut() {
        let mut reader = resize_event.get_reader();
        for e in reader.iter(&resize_event) {
            pt.width = e.width;
            pt.height = e.height;
        }

        pt.time = time.seconds_since_startup() as f32;
        let diff = 1.0/95.0 * (1.0/time.delta_seconds()).floor();
        if diff > 1.10 {
            if pt.pathlenght < 10 {
                pt.pathlenght += 1;
            } else {
                pt.samples += 1;
            }
        } else if diff < 0.90 {
            if pt.samples > 10 {
                pt.samples -= 1;
            } else {
                pt.pathlenght -= 1;
            }
        }
        // pt.samples = (1.0/time.delta_seconds()).floor() as i32;
    }
    // query.single_mut().
    // if let Ok(mut pt) = set.q0().single() {
        
    // }
}
//     let mut camera_rotation = None;

//     if let Ok(ct) = set.q0().single() {
//         camera_rotation = Some(ct.);
//     }
//     if let Some(ct) = camera_rotation {
//         if let Ok(mut screen_transform) = set.q1_mut().single_mut() {
//             screen_transform.rotation = ct;
//         }
//     }
// } 