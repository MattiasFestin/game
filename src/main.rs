#![feature(portable_simd)]

#[macro_use] extern crate bevy;
#[macro_use] extern crate bevycheck;
#[macro_use] extern crate serde;
extern crate bevy_mod_bounding;
extern crate bevy_frustum_culling;
extern crate rayon;
extern crate num_cpus;
extern crate bevy_rng;
extern crate core_simd;

use bevy::core::FixedTimestep;
use bevy::math::Vec3A;
use bevy_rng::Rng;
use rayon::prelude::*;

use std::collections::HashMap;
use std::fs;

use bevy::asset::HandleId;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::reflect::{TypeUuid, TypeUuidDynamic, Uuid};
use bevy::render::colorspace::HslRepresentation;
use bevy::tasks::{AsyncComputeTaskPool, ComputeTaskPool, Task, TaskPool};
use bevy::pbr::*;
use bevy_frustum_culling::Bounded;
use constants::{CHUNK_SIZE, CHUNK_SIZE_CUBE};
use futures_lite::future;

use bevy_rng::*;

mod constants;
mod camera;
mod input;
mod physics;
mod noise;

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
        .add_startup_system(spawn_tasks.system())

        // .add_system(rotation_system.system())
        .add_system(handle_tasks.system())

        // .add_system(print_ball_altitude.system())

        // .add_system(physics::position.system().label(physics::PhysicsSystem::Position).after(physics::PhysicsSystem::Velocity))
        // .add_system_set(SystemSet::new()
        //     .label(physics::Physics)
        //     .with_run_criteria(FixedTimestep::step(constants::PHYSICS_TICKS))
        //     .with_system(physics::velocity.system().label(physics::PhysicsSystem::Velocity).after(physics::PhysicsSystem::Force))
        //     .with_system(physics::force.system().label(physics::PhysicsSystem::Force))
        //     .with_system(physics::gravity.system().label(physics::PhysicsSystem::Gravity).before(physics::PhysicsSystem::Force))
        //     .with_system(physics::collition.system().after(physics::PhysicsSystem::Position))
        // )

        //Start game
        .run();
}

#[derive(Debug, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
struct Voxel {
    pub position: Vec3,
    pub id: u64,
    pub pbr_id: u64
}

impl Default for Voxel {
    fn default() -> Self {
        Self { position: Vec3::ZERO , pbr_id: 0u64, id: 0u64 }
    }
}
#[derive(Debug, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
struct VoxelChunk {
    pub position: Vec3,
    pub voxels: Vec<Voxel>
    // pub bounding_box: 
}

impl Default for VoxelChunk {
    fn default() -> Self {
        Self { 
            position: Vec3::ZERO,
            voxels: (1..CHUNK_SIZE_CUBE).into_iter().map(|index| {
                let rest = index;
                let x = rest % CHUNK_SIZE;

                let rest = (rest - x) / CHUNK_SIZE;
                let y = rest % CHUNK_SIZE;

                let rest = (rest - y) / CHUNK_SIZE;
                let z = rest % CHUNK_SIZE;


                let position = 2.0*Vec3::new(x as f32, y as f32, z as f32); //TODO: Chunk Offset
                Voxel {
                    id: index as u64,
                    position,
                    ..Default::default()
                }
            }).collect() 
        }
    }
}


struct BoxMeshHandle(Handle<Mesh>);

// #[derive(Debug, TypeUuid)]
// #[uuid = "f28c2ec3-0d0c-4ecd-8622-63e6d4262a60"]
// struct MaterialMapping {
//     map: HashMap<u64, PbrConfig>
// }

static mut material_mappings: Option<HashMap<u64, StandardMaterial>> = None;
static mut number_of_materials: u64 = 0;


fn add_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let box_mesh_handle = meshes.add(Mesh::from(bevy::prelude::shape::Icosphere { radius: 1.0, subdivisions: 32 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));


    let cwd = std::env::current_dir().unwrap().display().to_string();
    let contents = fs::read_to_string(format!("{0}/assets/materials/base.toml", cwd))
        .expect("Something went wrong reading the file");

    let dict: HashMap<String, PbrConfig> = toml::from_str(&contents).unwrap();
    let mut map = HashMap::new();
    for (k, v) in dict {
        map.insert(v.id, v.pbr());
    }

    let count = map.keys().count() as u64;
    unsafe {
        material_mappings = Some(map);
        number_of_materials = count;
    }
}

fn spawn_tasks(
        mut commands: Commands,
        thread_pool: Res<AsyncComputeTaskPool>
    ) {
    let task = thread_pool.spawn(async move {
        unsafe {
            while number_of_materials == 0 {
                println!("waiting...");
                future::yield_now().await;
            }
        }

        let mut chunk = VoxelChunk::default();
        let tmp: Vec<Voxel> = chunk.voxels.iter().map(|x| {
            Voxel {
                id: x.id,
                position: x.position,
                pbr_id: unsafe { noise::noise_1d(x.id, 54) % number_of_materials },
            }
        }).collect();
        chunk.voxels = tmp;
        return chunk;
    });

    commands.spawn().insert(task);
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
    mut meshes: ResMut<Assets<Mesh>>,
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<VoxelChunk>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    box_mesh_handle: Res<BoxMeshHandle>,
    mut rng: Local<Rng>
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel_chunk) = future::block_on(future::poll_once(&mut *task)) {

            for voxel in voxel_chunk.voxels {

                let mut mat: Option<&StandardMaterial> = None;
                unsafe {
                    if let Some(mm) = &material_mappings {
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
                                linvel: Vec3::ZERO.into(),//(-1.0 * voxel.position).into(), 
                                angvel: Vec3::ZERO.into()
                            },
                            forces: bevy_rapier3d::prelude::RigidBodyForces { gravity_scale: 1.0, ..Default::default() },
                            activation: bevy_rapier3d::prelude::RigidBodyActivation::cannot_sleep(),
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
                        // .insert(physics::Position {
                        //     position: voxel.position.into()
                        // })
                        // .insert(physics::Velocity {
                        //     velocity: Vec3A::ZERO,
                        // })
                        // .insert(physics::Force {
                        //     force: Vec3A::new(rng.gen::<f32>()* - 0.5, rng.gen::<f32>() - 0.5, rng.gen::<f32>() - 0.5),
                        //     is_dirty: true
                        // })
                        // .insert( physics::Mass {
                        //     mass: 125000000000.0,//rng.gen::<f32>()*10.0,
                        // })
                        // .insert(physics::Identity {
                        //     id: voxel.id
                        // })
                        // .insert(physics::InelasticCollision {
                        //     cor: 0.75
                        // })
                        // .insert(Bounded::<bevy_frustum_culling::aabb::Aabb>::default())
                        // .insert(bevy_frustum_culling::debug::DebugBounds)
                        ;
                }
            }

            // //Center heavy cube
            // commands
            //             .spawn()
            //             .insert_bundle(PbrBundle {
            //                 visible: Visible {
            //                     is_visible: true,
            //                     is_transparent: false,
            //                 },
            //                 mesh: box_mesh_handle.0.clone(),
            //                 material: materials.add(StandardMaterial {
            //                     double_sided: false,
            //                     base_color: Color::ALICE_BLUE,
            //                     ..Default::default()
            //                 }),
            //                 transform: Transform::from_translation(Vec3::ZERO),
            //                 ..Default::default()
            //             })
            //             .insert(bevy_frustum_culling::aabb::Aabb::default())
            //             .insert(physics::Position {
            //                 position: Vec3::new(CHUNK_SIZE as f32 / 2.0, CHUNK_SIZE as f32 / 2.0, CHUNK_SIZE as f32 / 2.0)
            //             })
            //             .insert(physics::Velocity {
            //                 velocity: Vec3::ZERO,
            //             })
            //             .insert(physics::Force {
            //                 force: Vec3::ZERO,
            //                 is_dirty: false
            //             })
            //             .insert( physics::Mass {
            //                 mass: 1498284460.0,
            //             })
            //             .insert(physics::Identity {
            //                 id: 99999999
            //             })
                        // .insert(Bounded::<bevy_frustum_culling::aabb::Aabb>::default())
                        // .insert(bevy_frustum_culling::debug::DebugBounds)
                        ;

                // commands
                //         .spawn()
                //         .insert_bundle(PbrBundle {
                //             visible: Visible {
                //                 is_visible: true,
                //                 is_transparent: false,
                //             },
                //             mesh: box_mesh_handle.0.clone(),
                //             material: materials.add(StandardMaterial {
                //                 double_sided: false,
                //                 base_color: Color::ALICE_BLUE,
                //                 ..Default::default()
                //             }),
                //             transform: Transform::from_translation(Vec3::ZERO),
                //             ..Default::default()
                //         })
                //         .insert(bevy_frustum_culling::aabb::Aabb::default())
                //         .insert(physics::Position {
                //             position: Vec3::new(-10.0, -10.0, -10.0)
                //         })
                //         .insert(physics::Velocity {
                //             velocity: Vec3::ZERO,
                //         })
                //         .insert(physics::Force {
                //             force: Vec3::ZERO,
                //             is_dirty: false
                //         })
                //         .insert( physics::Mass {
                //             mass: 14982844643.0,
                //         })
                //         .insert(physics::Identity {
                //             id: 99999999+1
                //         })
                //         // .insert(Bounded::<bevy_frustum_culling::aabb::Aabb>::default())
                //         // .insert(bevy_frustum_culling::debug::DebugBounds)
                //         ;

                // let collider = bevy_rapier3d::physics::ColliderBundle {
                //     shape: bevy_rapier3d::prelude::ColliderShape::cuboid(100.0, 0.1, 100.0),
                //     ..Default::default()
                // };
                // commands.spawn_bundle(collider);
            
                // /* Create the bouncing ball. */
                // let rigid_body = bevy_rapier3d::physics::RigidBodyBundle {
                //     position: Vec3::new(0.0, 10.0, 0.0).into(),
                //     ..Default::default()
                // };
                // let collider = bevy_rapier3d::physics::ColliderBundle {
                //     shape: bevy_rapier3d::prelude::ColliderShape::ball(0.5),
                //     material: bevy_rapier3d::prelude::ColliderMaterial {
                //         restitution: 0.7,
                //         ..Default::default()
                //     },
                //     ..Default::default()
                // };
                // commands.spawn_bundle(rigid_body)
                //     .insert_bundle(collider)
                //     .insert(bevy_rapier3d::physics::ColliderPositionSync::Discrete)
                //     .insert(bevy_rapier3d::render::ColliderDebugRender::with_id(1));

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

// struct Rotator;
// /// Rotate the meshes to demonstrate how the bounding volumes update
// fn rotation_system(
//         time: Res<Time>,
//         thread_pool: Res<ComputeTaskPool>,
//         mut query: Query<(&mut Transform, Option<&Visible>), With<Rotator>>
//     ) {
//     // let tp = TaskPool::new();
//     query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut transform, visible)| {
//         if visible.is_some() && visible.unwrap().is_visible {
//             // let scale = ::ONE * ((time.seconds_since_startup() as f32).sin() * 0.3 + 1.0) * 0.3;
//             let rot_x = Quat::from_rotation_x(time.delta_seconds() as f32 * 5.0);
//             let rot_y = Quat::from_rotation_y(time.delta_seconds() as f32 * 3.0);
//             let rot_z = Quat::from_rotation_z(time.delta_seconds() as f32 * 2.0);
//             // transform.scale = scale;
//             transform.rotate(rot_x * rot_y * rot_z);
//         }
//     });
// }

// fn print_ball_altitude(positions: Query<&bevy_rapier3d::prelude::RigidBodyPosition>) {
//     for rb_pos in positions.iter() {
//         println!("Ball altitude: {}", rb_pos.position.translation.vector.y);
//     }
// }