#[macro_use] extern crate bevy;
#[macro_use] extern crate bevycheck;

use std::ops::Add;

use bevy::prelude::*;
use bevy::tasks::{AsyncComputeTaskPool, Task};
use bevy::pbr::*;
use futures_lite::future;

fn main() {
    App::build()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_env.system())
        .add_startup_system(add_assets.system())
        .add_startup_system(spawn_tasks.system())
        .add_system(handle_tasks.system())
        .run();
}

#[derive(Debug, PartialEq, Clone, Copy, Reflect)]
#[reflect(Component, PartialEq)]
struct  Voxel {
    pub position: Vec3
}

impl Default for Voxel {
    fn default() -> Self {
        Self { position: Vec3::ZERO }
    }
}


const CHUNK_SIZE: usize = 10;
const CHUNK_SIZE_CUBE: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

#[derive(Debug, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
struct VoxelChunk {
    pub position: Vec3,
    pub voxels: Vec<Voxel>
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
                    position
                }
            }).collect() 
        }
    }
}


struct BoxMeshHandle(Handle<Mesh>);
struct BoxMaterialHandle(Handle<StandardMaterial>);

fn add_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 1.0 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));

    let box_material_handle = materials.add(Color::rgb(1.0, 0.2, 0.3).into());
    commands.insert_resource(BoxMaterialHandle(box_material_handle));
}

fn spawn_tasks(mut commands: Commands, thread_pool: Res<AsyncComputeTaskPool>) {
    //TODO: Spawn in new chunks
    let chunk = VoxelChunk::default();

    for voxel in chunk.voxels {
        let task = thread_pool.spawn(async move {
            voxel
        });

        // Spawn new entity and add our new task as a component
        commands.spawn().insert(task);
    }
}

fn handle_tasks(
    mut commands: Commands,
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<Voxel>)>,
    box_mesh_handle: Res<BoxMeshHandle>,
    box_material_handle: Res<BoxMaterialHandle>,
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel) = future::block_on(future::poll_once(&mut *task)) {
            commands.entity(entity).insert_bundle(PbrBundle {
                mesh: box_mesh_handle.0.clone(),
                material: box_material_handle.0.clone(),
                transform: Transform::from_translation(voxel.position),
                ..Default::default()
            });

            // Task is complete, so remove task component from entity
            commands.entity(entity).remove::<Task<Voxel>>();
        }
    }
}

fn setup_env(mut commands: Commands) {
    // Used to center camera on spawned cubes
    let offset = if CHUNK_SIZE % 2 == 0 {
        (CHUNK_SIZE / 2) as f32 - 0.5
    } else {
        (CHUNK_SIZE / 2) as f32
    };

    // println!("offset: {:?}", offset);

    // lights
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_translation(Vec3::new(0.8, 0.8, 0.8)),
        ..Default::default()
    });

    // camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(offset, offset, CHUNK_SIZE as f32 * 2.0f32))
            .looking_at(Vec3::new(offset, offset, 0.0), Vec3::Y),
        ..Default::default()
    });
}