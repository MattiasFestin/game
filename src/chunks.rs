use crate::{constants::{CHUNKS_LOADED, CHUNK_SIZE}, noise, pbr::{BoxMeshHandle, MaterialsMapping}};
use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};
use futures_lite::future::{self};
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
        let cs = (CHUNK_SIZE as f32) as f32;
        let ft = Vec3::new(cs * (t.translation.x / cs).floor(), cs * (t.translation.y / cs).floor(), 0.0);
        if !chunk_query.iter().any(|x| x.position == ft) {
            println!("Generating chunk {:?}, {:?}", ft, t.translation);
            let seed = state.seed;
            
            let len = 2u64;//materials_mappings.as_ref().unwrap().map.len() as u64;

            commands.spawn().insert(thread_pool.spawn(async move {
                return generate_chunk(seed, ft, CHUNK_SIZE as u64, len);
            }));
        }
	}
}

fn is_loaded(vc: &VoxelChunk, pos: Vec3, len: f32) -> bool {
    let dx = vc.position.x - pos.x;
    if dx.abs() >= len {
        return false;
    }

    let dy = vc.position.y - pos.y;
    if dy.abs() >= len {
        return false;
    }

    return true;
}

fn generate_chunk(seed: u64, pos: Vec3, size: u64, number_of_materials: u64) -> VoxelChunk {
    let mut chunk = VoxelChunk::default();
    chunk.position = pos;

    let cs = size as f32;

    let mut i = 0;
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                i += 1;

                if Vec3::new(cs/2.0, cs/2.0, cs/2.0).distance(Vec3::new(x as f32, y as f32, z as f32)) <=  cs/2.0 {
                    let pbr_id = noise::noise_3d(x, y, z, seed) % number_of_materials;

                    chunk.voxels.push(Voxel {
                        id: noise::noise_1d(i, seed),
                        position:  Vec3::new(x as f32, y as f32, z as f32),
                        pbr_id: pbr_id,
                    });
                }
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

    // chunk_query: Query<&VoxelChunk, With<VoxelChunk>>,
    box_mesh_handle: Res<BoxMeshHandle>,
	material_mapping: Res<MaterialsMapping>,
) {
    for (entity, mut task) in voxel_chunk_tasks.iter_mut() {
        if let Some(voxel_chunk) = future::block_on(future::poll_once(&mut *task)) {
                let voxels = voxel_chunk.voxels.clone();
                let vc_pos = voxel_chunk.position.clone();
                commands

                    .spawn()
                    .insert(voxel_chunk)
                    .insert(GlobalTransform::from_translation(vc_pos))
                    .with_children(|parent| {
                        for voxel in voxels {
                            if let Some(m) = material_mapping.map.get(&voxel.pbr_id) {
                                // println!("{:?}", voxel.position);
                                let pos = voxel.position.clone();
                                parent
                                    .spawn()
                                    .insert(voxel)
                                    .insert_bundle(PbrBundle {
                                        visible: Visible {
                                            is_visible: true,
                                            is_transparent: false,
                                        },
                                        mesh: box_mesh_handle.0.clone(),
                                        material:  m.value().clone(),
                                        global_transform: GlobalTransform::from_translation(vc_pos + pos),
                                        ..Default::default()
                                    })
                                    .insert(bevy_frustum_culling::aabb::Aabb::default());
                            }
                        }
                    
                    })
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

            commands.entity(entity).remove::<Task<VoxelChunk>>();
    }
}

pub fn voxel_debug(
    _voxels: Query<&Voxel, With<Voxel>>,
) {
    // println!("Voxel: {}", voxels.iter().count());
}

mod tests {
    extern crate test;
    #[allow(unused_imports)]
    use bevy::math::Vec3;


    //TODO:
    // #[test]
    // fn squirrel3_tests() {
    //     let chunk = super::generate_chunk(55, Vec3::new(0.0, 0.0, 0.0), 0, 10);
    //     assert_eq!(387, chunk.voxels.len());
    //     assert_eq!(Vec3::new(0.0, 0.0, 0.0), chunk.voxels[0].position);
    //     assert_eq!(Vec3::new(0.0, 1.0, 0.0), chunk.voxels[1].position);
    // }
}