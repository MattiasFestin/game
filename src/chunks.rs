use crate::{NUMBER_OF_MATERIALS, constants::{CHUNKS_LOADED, CHUNK_SIZE}, noise};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
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
	loaded: Local<LoadedChunks>
) {
	if let Ok(t) = camera_query.single() {
		let x = t.translation.x.floor() as u64;
		let y = t.translation.y.floor() as u64;
		// let z = t.translation.z.floor() as u64;
		
		let seed = state.seed;
		if !loaded.is_loaded(x, y) {
			let task = thread_pool.spawn(async move {
				unsafe {
					while NUMBER_OF_MATERIALS == 0 {
						println!("waiting...");
						future::yield_now().await;
					}
				}
		
				let hightmap = simdnoise::NoiseBuilder::fbm_2d(crate::constants::CHUNK_SIZE, crate::constants::CHUNK_SIZE)
					.with_seed(seed as i32)
					.generate_scaled(0.0, unsafe {NUMBER_OF_MATERIALS as f32 });
		
				let cs = crate::constants::CHUNK_SIZE as f32;
		
				let mut chunk = VoxelChunk::default();
				for x in 0..CHUNK_SIZE {
					for z in 0..CHUNK_SIZE {
						let y_index = x + z * CHUNK_SIZE;
						let max_y = (hightmap[y_index] * cs).floor() as usize;
		
						for y in 0..max_y {
							let index = (x + y * CHUNK_SIZE + z * CHUNK_SIZE * CHUNK_SIZE) as u64;
							chunk.voxels.push(Voxel {
								id: noise::noise_1d(index, seed),
								position: Vec3::new(x as f32, y as f32, z as f32),
								pbr_id: unsafe { noise::noise_1d(index, seed) % NUMBER_OF_MATERIALS },
							});
						}
					}
				}
		
				return chunk;
			});
		
			commands.spawn().insert(task);
		}
	}
}