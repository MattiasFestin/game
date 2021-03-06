#![allow(dead_code)]

#![feature(const_type_id)]
#![feature(portable_simd)]
#![feature(test)]

extern crate bevy;
// #[macro_use] extern crate bevycheck;
#[macro_use] extern crate serde;
#[macro_use] extern crate derive_new;
extern crate bevy_mod_bounding;
extern crate bevy_frustum_culling;
// extern crate bevy_world_to_screenspace;
extern crate rayon;
extern crate num_cpus;
extern crate bevy_rng;
extern crate core_simd;
extern crate simdnoise;
extern crate rand;
extern crate lru;
extern crate dashmap;
extern crate bvh;
extern crate bevy_mod_raycast;
extern crate bevy_mod_picking;
extern crate glsl_include;


use std::collections::HashMap;

use chunks::create_voxels;
use chunks::load_chunk;
use pbr::load_materials;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;


use bevy_rng::*;

mod constants;
mod camera;
mod input;
mod physics;
mod noise;
mod state;
mod chunks;
mod pbr;
mod easing;
mod projections;
mod bsp;
mod procedual;
mod shaders;
mod utils;
mod path_tracer;
mod window;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(WindowDescriptor {
            title: "I am a window!".to_string(),
            width: 1920.,
            height: 1080.,
            vsync: true,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)

        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        // .add_plugin(RngPlugin::from(42)) //TODO: Seed
        // .add_plugin(bevy_rapier3d::render::RapierRenderPlugin)

        // .add_plugin(bevy_rapier3d::physics::RapierPhysicsPlugin::<bevy_rapier3d::physics::NoUserData>::default())
        // .add_system(physics::gravity.system())
        // .add_system(physics::impulse.system())

        //Camera
        .add_startup_system(camera::setup_camera.system())

        // .add_startup_system(load_materials.system())
        // .add_startup_system(setup_env.system())

        //Input register
        .init_resource::<input::GamepadLobby>()
        // .add_system_to_stage(CoreStage::PreUpdate, input::gamepad_connection_system.system())
        // .add_system(input::gamepad_system.system().label("gamepad"))
        .add_system(input::mouse_keyboard_system.system())

        .add_system(camera::update_camera.system())

        .insert_resource(window::WindowSize::default())
        .add_system(window::resize_notificator.system())
        .add_startup_system(window::setup_window.system().label("setup_window"))

        .add_startup_system(crate::path_tracer::path_trace.system().after("setup_window"))
        .add_system(path_tracer::update_pt.system())
        // .add_system(crate::path_tracer::update_pt.system())
        
		// .add_system(load_chunk.system())
        // .add_system(create_voxels.system())

        // .add_system(chunks::voxel_debug.system())

        .insert_resource(shaders::ShaderCache::default())
        
        // .add_startup_system(shaders::setup_shader.system())

        // .add_system(physics::black_body.system())
        // .add_startup_system(procedual::solar_system::render_solar_system.system())

        //Start game
        .run();
}


// fn setup_env(
//     mut commands: Commands,
// ) {
//     // lights
//     commands.spawn_bundle(LightBundle {
//         transform: Transform::from_translation(Vec3::new(0.8, 0.8, 0.8)),
//         ..Default::default()
//     });
// }