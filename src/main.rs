#![allow(dead_code)]

#![feature(portable_simd)]
#![feature(test)]

extern crate bevy;
// #[macro_use] extern crate bevycheck;
#[macro_use] extern crate serde;
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

use crate::chunks::LoadedChunks;
use crate::chunks::Voxel;
use crate::chunks::VoxelChunk;
use crate::pbr::PbrConfig;
use bevy_rng::Rng;
use chunks::create_voxels;
use chunks::load_chunk;
use chunks::setup_material_mappings;
use pbr::MaterialsMapping;
use pbr::load_materials;

use std::collections::HashMap;
use std::fs;


use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::reflect::{TypeUuidDynamic, Uuid};

use bevy::tasks::{AsyncComputeTaskPool, Task};


use constants::{CHUNK_SIZE, CHUNK_SIZE_CUBE};
use futures_lite::future;

use bevy_rng::*;

mod constants;
mod camera;
mod input;
mod physics;
mod noise;
mod state;
mod chunks;
mod pbr;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)

        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RngPlugin::from(42)) //TODO: Seed
        .add_plugin(bevy_rapier3d::render::RapierRenderPlugin)

        .add_plugin(bevy_rapier3d::physics::RapierPhysicsPlugin::<bevy_rapier3d::physics::NoUserData>::default())
        // .add_startup_system(setup_physics.system())

        //Camera
        .add_startup_system(camera::setup_camera.system())

        //Input register
        .init_resource::<input::GamepadLobby>()
        .add_system_to_stage(CoreStage::PreUpdate, input::connection_system.system())
        .add_system(input::gamepad_system.system().label("gamepad"))


        .init_resource::<LoadedChunks>()
        .init_resource::<MaterialsMapping>()
        .add_startup_system(setup_env.system())
        .add_startup_system(load_materials.system())
        
		.add_system(load_chunk.system())
		
        // .add_system(rotation_system.system())
        .add_system(create_voxels.system())

        //Start game
        .run();
}


fn setup_env(mut commands: Commands) {
    // lights
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.8, 0.8, 0.8)),
        ..Default::default()
    });
}