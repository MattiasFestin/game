use std::{collections::HashMap, sync::{Arc, Mutex}};

use crate::{constants::{CHUNKS_LOADED, CHUNK_SIZE}, noise, pbr::{BoxMeshHandle, MaterialsMapping}};
use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use bevy_rng::Rng;
use futures_lite::future;
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
	loaded: Local<LoadedChunks>,
	// materials_mappings: Local<MaterialsMapping>
) {
	if let Ok(t) = camera_query.single() {
		let x = t.translation.x.floor() as u64;
		let y = t.translation.y.floor() as u64;

		// let arc_materials_mappings = Arc::new(Mutex::new(materials_mappings));
		let seed = state.seed;
		if !loaded.is_loaded(x, y) {
			println!("Not loaded ({0}, {1})", x, y);
			let task = thread_pool.spawn(async move {
				let mut chunk = VoxelChunk::default();
				// if let Ok(mm_lock) = arc_materials_mappings.lock() {
					// let mm = *mm_lock;
					// while materials_mappings.keys().count() == 0 {
					// 	println!("waiting...");
					// 	future::yield_now().await;
					// }
			
					let hightmap = simdnoise::NoiseBuilder::fbm_2d(crate::constants::CHUNK_SIZE, crate::constants::CHUNK_SIZE)
						.with_seed(seed as i32)
						.generate_scaled(0.0, 1.0 );
			
					let cs = crate::constants::CHUNK_SIZE as f32;
			
					
					for x in 0..CHUNK_SIZE {
						for z in 0..CHUNK_SIZE {
							let y_index = x + z * CHUNK_SIZE;
							let max_y = (hightmap[y_index] * cs).floor() as usize;
			
							for y in 0..max_y {
								println!("Creating ({0}, {1}, {2})", x, y, z);
								let index = (x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE) as u64;
								chunk.voxels.push(Voxel {
									id: noise::noise_1d(index, seed),
									position: Vec3::new(x as f32, y as f32, z as f32),
									pbr_id: 0, //unsafe { noise::noise_1d(index, seed) % NUMBER_OF_MATERIALS }
								});
							}
						}
					}
				// }

				return chunk;
			});
		
			commands.spawn().insert(task);
		} else {
			println!("Is loaded ({0}, {1})", x, y);
		}
	}
}

pub fn create_voxels<'a>(
    mut commands: Commands,
    _meshes: ResMut<Assets<Mesh>>,
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<VoxelChunk>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    box_mesh_handle: Res<BoxMeshHandle>,
    _rng: Local<Rng>,
	materials_mappings: Local<HashMap<u64, StandardMaterial>>
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel_chunk) = future::block_on(future::poll_once(&mut *task)) {

            for voxel in voxel_chunk.voxels {
                println!("voxel");

                if let Some(m) = materials_mappings.get(&voxel.pbr_id) {
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