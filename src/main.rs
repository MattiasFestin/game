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

use crate::chunks::LoadedChunks;
use crate::chunks::Voxel;
use crate::chunks::VoxelChunk;
use bevy_rng::Rng;
use chunks::load_chunk;

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

        .add_startup_system(setup_env.system())
        .add_startup_system(add_assets.system())
        
		.add_system(load_chunk.system())
		
        // .add_system(rotation_system.system())
        .add_system(handle_tasks.system())

        //Start game
        .run();
}

struct BoxMeshHandle(Handle<Mesh>);

static mut MATERIAL_MAPPINGS: Option<HashMap<u64, StandardMaterial>> = None;
static mut NUMBER_OF_MATERIALS: u64 = 0;


fn add_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let box_mesh_handle = meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 1.0 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));


    let cwd = std::env::current_dir().unwrap().display().to_string();
    let contents = fs::read_to_string(format!("{0}/assets/materials/base.toml", cwd))
        .expect("Something went wrong reading the file");

    let dict: HashMap<String, PbrConfig> = toml::from_str(&contents).unwrap();
    let mut map = HashMap::new();
    for (_k, v) in dict {
        map.insert(v.id, v.pbr());
    }

    let count = map.keys().count() as u64;
    unsafe {
        MATERIAL_MAPPINGS = Some(map);
        NUMBER_OF_MATERIALS = count;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
struct PbrConfig {
    pub uuid: Uuid,
    pub id: u64,
    
    pub unlit: bool,

    pub color: [u8; 3],
    pub emissive: [u8; 3],

    pub metalic: u8,
    pub roughness: u8,
    pub reflectance: u8,
    // pub clearcoat: u8,
    // pub clearcoatroughness: u8,
    // pub ansiotropy: u8,
}

impl TypeUuidDynamic for PbrConfig {
    fn type_uuid(&self) -> Uuid {
        self.uuid
    }

    fn type_name(&self) -> &'static str {
        "PbrConfig"
    }
}

impl Default for PbrConfig {
    fn default() -> Self {
        Self {
            id: 0,
            uuid: Uuid::new_v4(),
            
            unlit: false,
            color: [0u8; 3],
            emissive: [0u8; 3],
            metalic: 0u8,
            reflectance: 0u8,
            roughness: 0u8,
        }
    }
}

impl PbrConfig {
    pub fn color(&self) -> Color {
        let h = (self.color[0] & 31) as f32;
        let s = (self.color[1] & 31) as f32;
        let l = (self.color[2] & 31) as f32;
        return Color::hsl(h, s, l);
    }

    pub fn emissive(&self) -> Color {
        let h = (self.emissive[0] & 31) as f32;
        let s = (self.emissive[1] & 31) as f32;
        let l = (self.emissive[2] & 31) as f32;
        return Color::hsl(h, s, l);
    }

    pub fn metalic(&self) -> f32 {
        return (self.metalic & 31) as f32 / 32.0f32;
    }

    pub fn roughness(&self) -> f32 {
        return (self.roughness & 31) as f32 / 32.0f32;
    }

    pub fn reflectance(&self) -> f32 {
        return (self.reflectance & 31) as f32 / 32.0f32;
    }

    // pub fn clearcoat(&self) -> f32 {
    //     return (self.clearcoat & 31) as f32 / 32.0f32;
    // }

    // pub fn clearcoatroughness(&self) -> f32 {
    //     return (self.clearcoatroughness & 31) as f32 / 32.0f32;
    // }

    // pub fn ansiotropy(&self) -> f32 {
    //     return (self.ansiotropy & 31) as f32 / 32.0f32;
    // }

    pub fn pbr(&self) -> StandardMaterial {
        StandardMaterial {
            base_color: self.color(),
            emissive: self.emissive(),
            double_sided: false,
            metallic: self.metalic(),
            reflectance: self.reflectance(),
            roughness: self.roughness(),
            unlit: self.unlit,

            ..Default::default()
        }
    }
}

fn handle_tasks<'a>(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<VoxelChunk>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    box_mesh_handle: Res<BoxMeshHandle>,
    _rng: Local<Rng>
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel_chunk) = future::block_on(future::poll_once(&mut *task)) {

            for voxel in voxel_chunk.voxels {

                let mut mat: Option<&StandardMaterial> = None;
                unsafe {
                    if let Some(mm) = &MATERIAL_MAPPINGS {
                        mat = mm.get(&voxel.pbr_id);
                    }
                }

                if let Some(m) = mat {
                    commands
                        .spawn()
                        .insert_bundle(PbrBundle {
                            visible: Visible {
                                is_visible: true,
                                is_transparent: false,
                            },
                            mesh: box_mesh_handle.0.clone(),
                            material: materials.add(StandardMaterial {
                                double_sided: m.double_sided,
                                base_color: m.base_color,
                                metallic: m.metallic,
                                reflectance: m.reflectance,
                                roughness: m.roughness,
                                emissive: m.emissive,
                                unlit: m.unlit,
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(voxel.position),
                            ..Default::default()
                        })
                        .insert(bevy_frustum_culling::aabb::Aabb::default())
                        .insert_bundle(bevy_rapier3d::physics::RigidBodyBundle {
                            position: voxel.position.into(),
                            velocity: bevy_rapier3d::prelude::RigidBodyVelocity { 
                                linvel: Vec3::ZERO.into(),
                                angvel: Vec3::ZERO.into()
                            },
                            forces: bevy_rapier3d::prelude::RigidBodyForces { gravity_scale: 0.0, ..Default::default() },
                            activation: bevy_rapier3d::prelude::RigidBodyActivation::active(),
                            ccd: bevy_rapier3d::prelude::RigidBodyCcd { ccd_enabled: true, ..Default::default() },
                            ..Default::default()
                        })
                        .insert_bundle(bevy_rapier3d::physics::ColliderBundle {
                            shape: bevy_rapier3d::prelude::ColliderShape::ball(1.0),
                            collider_type: bevy_rapier3d::prelude::ColliderType::Sensor,
                            position: (Vec3::new(2.0, 0.0, 3.0), Quat::from_rotation_x(0.4)).into(),
                            material: bevy_rapier3d::prelude::ColliderMaterial { friction: 0.7, restitution: 0.3, ..Default::default() },
                            mass_properties: bevy_rapier3d::prelude::ColliderMassProps::Density(2.0),
                            ..Default::default()
                        })
                        .insert(Transform::default())
                        .insert(bevy_rapier3d::physics::RigidBodyPositionSync::Discrete)
                        ;
                }
            }

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<Task<VoxelChunk>>();
        }
    }
}

fn setup_env(mut commands: Commands) {
    // lights
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.8, 0.8, 0.8)),
        ..Default::default()
    });
}