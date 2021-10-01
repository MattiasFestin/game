use std::{collections::HashMap, pin::Pin, sync::{Arc, Mutex}};

use crate::{constants::{CHUNKS_LOADED, CHUNK_SIZE}, noise, pbr::{BoxMeshHandle, MaterialsMapping}};
use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use bevy_rng::Rng;
use futures_lite::future::{self, yield_now};
use lru::LruCache;

#[derive(Debug, PartialEq, Clone, Reflect)]
#[reflect(Component, PartialEq)]
pub struct Voxel {
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
pub struct VoxelChunk {
    pub position: Vec3,
    pub voxels: Vec<Voxel>
    // pub bounding_box: 
}

impl Default for VoxelChunk {
    fn default() -> Self {
        Self { 
            position: Vec3::ZERO,
            voxels: Vec::new()
        }
    }
}

pub struct LoadedChunks {
	pub chunks: LruCache<(u64,u64), VoxelChunk>
}

impl Default for LoadedChunks {
    fn default() -> Self {
		Self {
			chunks: LruCache::new(CHUNKS_LOADED)
		}
	}
}

impl LoadedChunks {
	fn is_loaded(&self, x: u64, y: u64) -> bool {
		self.chunks.contains(&(x, y))
	}

	fn insert(&mut self, x: u64, y: u64, chunk: VoxelChunk) {
		self.chunks.put((x, y), chunk);
	}
}

pub fn load_chunk(
	mut commands: Commands,
	camera_query: Query<&Transform, With<crate::camera::PlayerCamera>>,
	thread_pool: Res<AsyncComputeTaskPool>,
	state: Local<crate::state::GameState>,

    chunkQuery: Query<&VoxelChunk, With<VoxelChunk>>
	// mut loaded: ResMut<Option<LoadedChunks>>,
	// materials_mappings: Res<Option<MaterialsMapping>>
) {
	if let Ok(t) = camera_query.single() {
        // if materials_mappings.is_some() {// && loaded.is_some() {
            let x = t.translation.x.floor() as u64;
            let y = t.translation.y.floor() as u64;

            let seed = state.seed;
            
            let len = 2u64;//materials_mappings.as_ref().unwrap().map.len() as u64;
            let is_loaded = false;//loaded.as_ref().unwrap().is_loaded(x, y);

            if !is_loaded {
                println!("Not loaded ({0}, {1})", x, y);
                
            
                commands.spawn().insert(thread_pool.spawn(async move {
                    return generate_chunk(seed, x, y, CHUNK_SIZE as u64, len);
                }));
            } else {
                println!("Is loaded ({0}, {1})", x, y);
            }
        // }
	}
}

fn generate_chunk(seed: u64, x: u64, y: u64, size: u64, number_of_materials: u64) -> VoxelChunk {
    let mut chunk = VoxelChunk::default();
    
    let hightmap = simdnoise::NoiseBuilder::fbm_2d(crate::constants::CHUNK_SIZE, crate::constants::CHUNK_SIZE)
        .with_seed(noise::noise_2d(x, y, seed) as i32)
        .generate_scaled(0.0, 1.0 );

    let cs = crate::constants::CHUNK_SIZE as f32;

    
    for x in 0..size {
        for z in 0..size {
            let y_index = (x + z * size) as usize;
            let max_y = (hightmap[y_index] * cs).floor() as u64;

            for y in 0..max_y {
                let index = (x + y * size + z * size * size) as u64;
                let pbr_id = index % number_of_materials;
                chunk.voxels.push(Voxel {
                    id: noise::noise_1d(index, seed),
                    position: Vec3::new(x as f32, y as f32, z as f32),
                    pbr_id: pbr_id,
                });
            }
        }
    }

    return chunk;
}

pub fn setup_material_mappings(
    mut commands: Commands
) {
    println!("setup_material_mappings");
    commands
        .insert_resource(MaterialsMapping::default());
    
    commands
        .insert_resource(LoadedChunks::default());
}

pub fn create_voxels<'a>(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<VoxelChunk>)>,
    box_mesh_handle: Res<BoxMeshHandle>,
	material_mapping: Res<MaterialsMapping>
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        println!("create_voxels");
        if let Some(voxel_chunk) = future::block_on(future::poll_once(&mut *task)) {
            for voxel in voxel_chunk.voxels {
                println!("Voxel: {:?}", voxel.id);
                if let Some(m) = material_mapping.map.get(&voxel.pbr_id) {
                    commands
                        .spawn()

                        //TODO: insert chunk as parent and voxels as childern...

                        .insert_bundle(PbrBundle {
                            visible: Visible {
                                is_visible: true,
                                is_transparent: false,
                            },
                            mesh: box_mesh_handle.0.clone(),
                            material: m.value().clone(),
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

mod tests {
    extern crate test;
    use test::Bencher;


    #[test]
    fn squirrel3_tests() {
        let chunk = super::generate_chunk(55, 0, 0, 10, 42);
        assert_eq!(387, chunk.voxels.len());
        assert_eq!(bevy::math::Vec3::new(0.0, 0.0, 0.0), chunk.voxels[0].position);
        assert_eq!(bevy::math::Vec3::new(0.0, 1.0, 0.0), chunk.voxels[1].position);
        // let seed = 55u64;
        // assert_eq!(12687927802791220436, crate::noise::squirrel3(1, seed));

        // let seed = 56u64;
        // assert_eq!(12687927848928216793, crate::noise::squirrel3(1, seed));

        // let seed = 0u64;
        // assert_eq!(3033592379929695938, crate::noise::squirrel3(0, seed));
    }
}