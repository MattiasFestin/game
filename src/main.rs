#[macro_use] extern crate bevy;
#[macro_use] extern crate bevycheck;
#[macro_use] extern crate serde;
extern crate bevy_mod_bounding;
extern crate bevy_frustum_culling;
extern crate rayon;
extern crate num_cpus;

use rayon::prelude::*;

use std::collections::HashMap;
use std::fs;
use std::ops::Add;
use std::str::FromStr;

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

mod constants;
mod camera;
mod input;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)

        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())


        .add_plugin(bevy_frustum_culling::BoundingVolumePlugin::<bevy_frustum_culling::aabb::Aabb>::default())
        .add_plugin(bevy_frustum_culling::FrustumCullingPlugin::<bevy_frustum_culling::aabb::Aabb>::default())

        //Camera
        .add_startup_system(camera::setup_camera.system())

        //Input register
        .init_resource::<input::GamepadLobby>()
        .add_system_to_stage(CoreStage::PreUpdate, input::connection_system.system())
        .add_system(input::gamepad_system.system())

        .add_startup_system(setup_env.system())
        .add_startup_system(add_assets.system())
        .add_startup_system(spawn_tasks.system())
        .add_system(rotation_system.system())
        .add_system(handle_tasks.system())

        //Start game
        .run();
}

#[derive(Debug, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
struct Voxel {
    pub position: Vec3,
    pub pbr_id: u64
}

impl Default for Voxel {
    fn default() -> Self {
        Self { position: Vec3::ZERO , pbr_id: 0u64 }
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


                let position = Vec3::new(x as f32, y as f32, z as f32); //TODO: Chunk Offset
                Voxel {
                    position,
                    pbr_id: ((x + y + z) % 2) as u64
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


fn add_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>
) {
    let box_mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));


    let cwd = std::env::current_dir().unwrap().display().to_string();
    let contents = fs::read_to_string(format!("{0}/assets/materials/base.toml", cwd))
        .expect("Something went wrong reading the file");

    let dict: HashMap<String, PbrConfig> = toml::from_str(&contents).unwrap();
    let mut map = HashMap::new();
    for (k, v) in dict {
        map.insert(v.id, v.pbr());
        // let material_id = HandleId::new(v.uuid, v.id);
        // let handle = Handle::<StandardMaterial>::weak(material_id);
    
        //TODO: Ref
        // materials.add(v.pbr());
    //     let pbr = v.pbr();
    //     if materials.get(handle.clone()).is_none() {
    //         materials.set_untracked(handle.clone(), pbr);
    //     }
    }

    unsafe {
        material_mappings = Some(map);
    }
}

fn spawn_tasks(
        mut commands: Commands,
        thread_pool: Res<AsyncComputeTaskPool>
    ) {
    //TODO: Spawn in new chunks
    // let chunk = VoxelChunk::default();

    // for voxel in chunk.voxels {
        let task = thread_pool.spawn(async move {
            // let mut v= voxel.clone();
            // // v.pbr_id = Some(HandleId::new(
            // //     Uuid::from_str("2642b340-7267-4b72-8af7-bb4f279508dd").unwrap(), 
            // //     2
            // // ));
            return VoxelChunk::default();
        });

        // Spawn new entity and add our new task as a component
        commands.spawn().insert(task);
    // }
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
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<VoxelChunk>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    box_mesh_handle: Res<BoxMeshHandle>,
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel_chunk) = future::block_on(future::poll_once(&mut *task)) {

            for voxel in voxel_chunk.voxels {

                let mut mat: Option<&StandardMaterial> = None;
                unsafe {
                    if let Some(mm) = &material_mappings {
                        // println!("pbr_id: {:?}", voxel.pbr_id);
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
                                base_color: m.base_color,
                                ..Default::default()
                            }),
                            transform: Transform::from_translation(voxel.position),
                            ..Default::default()
                        })
                        .insert(bevy_frustum_culling::aabb::Aabb::default())
                        // .insert(Bounded::<bevy_frustum_culling::aabb::Aabb>::default())
                        // .insert(bevy_frustum_culling::debug::DebugBounds)
                        .insert(Rotator);
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

struct Rotator;
/// Rotate the meshes to demonstrate how the bounding volumes update
fn rotation_system(
        time: Res<Time>,
        thread_pool: Res<ComputeTaskPool>,
        mut query: Query<(&mut Transform, Option<&Visible>), With<Rotator>>
    ) {
    // let tp = TaskPool::new();
    query.par_for_each_mut(&thread_pool, num_cpus::get(), |(mut transform, visible)| {
        if visible.is_some() && visible.unwrap().is_visible {
            // let scale = ::ONE * ((time.seconds_since_startup() as f32).sin() * 0.3 + 1.0) * 0.3;
            let rot_x = Quat::from_rotation_x(time.delta_seconds() as f32 * 5.0);
            let rot_y = Quat::from_rotation_y(time.delta_seconds() as f32 * 3.0);
            let rot_z = Quat::from_rotation_z(time.delta_seconds() as f32 * 2.0);
            // transform.scale = scale;
            transform.rotate(rot_x * rot_y * rot_z);
        }
    });
}