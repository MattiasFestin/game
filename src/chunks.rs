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

    chunk_query: Query<&VoxelChunk, With<VoxelChunk>>
	// mut loaded: ResMut<Option<LoadedChunks>>,
	// materials_mappings: Res<Option<MaterialsMapping>>
) {
	if let Ok(t) = camera_query.single() {
        let ft = (CHUNK_SIZE as f32) * (t.translation / (CHUNK_SIZE as f32)).floor();
        if !chunk_query.iter().any(|x| (ft.x - x.position.x).abs() < 1.0 || (ft.y - x.position.y).abs() < 1.0) {
            println!("Generating chunk {:?}, {:?}", ft, t.translation);
            let seed = state.seed;
            
            let len = 2u64;//materials_mappings.as_ref().unwrap().map.len() as u64;

            commands.spawn().insert(thread_pool.spawn(async move {
                return generate_chunk(seed, ft, CHUNK_SIZE as u64, len);
            }));
        }
	}
}

fn generate_chunk(seed: u64, pos: Vec3, size: u64, number_of_materials: u64) -> VoxelChunk {
    let mut chunk = VoxelChunk::default();
    chunk.position = pos;

    let x = pos.x as u64;
    let y = pos.y as u64;
    
    let hightmap = simdnoise::NoiseBuilder::fbm_2d(size as usize, size as usize)
        .with_seed(noise::noise_2d(x, y, seed) as i32)
        .generate_scaled(0.0, 1.0);

    let cs = size as f32;

    
    for x in 0..size {
        for z in 0..size {
            let y_index = (x + z * size) as usize;
            let max_y = (hightmap[y_index] * cs).floor() as u64;

            for y in 0..max_y {
                let index = (x + y * size + z * size * size) as u64;
                let pbr_id = index % number_of_materials;
                chunk.voxels.push(Voxel {
                    id: noise::noise_1d(index, seed),
                    position: pos + Vec3::new(x as f32, y as f32, z as f32),
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
    mut voxel_chunk_tasks: Query<(Entity, &mut Task<VoxelChunk>)>,

    existing_voxel_chunks: Query<&VoxelChunk, With<VoxelChunk>>,
    box_mesh_handle: Res<BoxMeshHandle>,
	material_mapping: Res<MaterialsMapping>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel_chunk) = future::block_on(future::poll_once(&mut *task)) {
            if !existing_voxel_chunks.iter().any(|x| x.position.abs_diff_eq(voxel_chunk.position, 0.9)) {
                let voxels = voxel_chunk.voxels.clone();
                // let pos = voxel_chunk.position.clone();
                for voxel in voxels {
                    if let Some(m) = material_mapping.map.get(&voxel.pbr_id) {
                        commands
                            .spawn()
                            .insert_bundle(PbrBundle {
                                visible: Visible {
                                    is_visible: true,
                                    is_transparent: false,
                                },
                                mesh: box_mesh_handle.0.clone(),
                                material: m.value().clone(),
                                // materials.add(StandardMaterial {
                                //     base_color: ,
                                //     ..Default::default()
                                // }),
                                transform: Transform::from_translation(voxel.position),
                                ..Default::default()
                            })
                            // .spawn()
                            // .insert(voxel_chunk)
                            // .with_children(|parent| {
                            //     for voxel in voxels {
                            //         if let Some(m) = material_mapping.map.get(&voxel.pbr_id) {
                            //             parent
                            //                 .spawn()
                            //                 .insert_bundle(PbrBundle {
                            //                     visible: Visible {
                            //                         is_visible: true,
                            //                         is_transparent: false,
                            //                     },
                            //                     mesh: box_mesh_handle.0.clone(),
                            //                     material:  materials.add(StandardMaterial {
                            //                         base_color: Color::BLUE,
                            //                         ..Default::default()
                            //                     }),
                            //                     transform: Transform::from_translation(voxel.position),
                            //                     ..Default::default()
                            //                 }); 
                            //         }
                            //     }
                            
                            // })
                            .insert(bevy_frustum_culling::aabb::Aabb::default())
                            // .insert_bundle(bevy_rapier3d::physics::RigidBodyBundle {
                            //     position: pos.into(),
                            //     velocity: bevy_rapier3d::prelude::RigidBodyVelocity { 
                            //         linvel: Vec3::ZERO.into(),
                            //         angvel: Vec3::ZERO.into()
                            //     },
                            //     forces: bevy_rapier3d::prelude::RigidBodyForces { gravity_scale: 1.0, ..Default::default() },
                            //     activation: bevy_rapier3d::prelude::RigidBodyActivation::cannot_sleep(),
                            //     ccd: bevy_rapier3d::prelude::RigidBodyCcd { ccd_enabled: true, ..Default::default() },
                            //     ..Default::default()
                            // })
                            // .insert_bundle(bevy_rapier3d::physics::ColliderBundle {
                            //     shape: bevy_rapier3d::prelude::ColliderShape::cuboid(1.0, 1.0, 1.0),
                            //     collider_type: bevy_rapier3d::prelude::ColliderType::Sensor,
                            //     position: (pos, Quat::from_rotation_x(0.0)).into(),
                            //     material: bevy_rapier3d::prelude::ColliderMaterial { friction: 0.7, restitution: 0.3, ..Default::default() },
                            //     mass_properties: bevy_rapier3d::prelude::ColliderMassProps::Density(2.0),
                            //     ..Default::default()
                            // })
                            // .insert(bevy_rapier3d::physics::RigidBodyPositionSync::Discrete)
                            ;
                        }
                    }
                }
            }

            commands.entity(entity).remove::<Task<VoxelChunk>>();
    }
}

mod tests {
    extern crate test;
    use bevy::math::Vec3;
    use test::Bencher;


    #[test]
    fn squirrel3_tests() {
        let chunk = super::generate_chunk(55, Vec3::new(0.0, 0.0, 0.0), 0, 10);
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