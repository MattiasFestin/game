use bevy::math::Vec3;
use bvh::{Point3, aabb::{AABB, Bounded}};

use crate::constants::CHUNK_SIZE;


impl Bounded for crate::chunks::Voxel {
    fn aabb(&self) -> AABB {
        let min = self.position;
        let max = self.position + Vec3::ONE;
        AABB::with_bounds(Point3::new(min.x, min.y, min.z), Point3::new(max.x, max.y, max.z))
    }
}

impl Bounded for crate::chunks::VoxelChunk {
    fn aabb(&self) -> AABB {
        let min = self.position;
        let max = self.position + (CHUNK_SIZE as f32) * Vec3::ONE;
        AABB::with_bounds(Point3::new(min.x, min.y, min.z), Point3::new(max.x, max.y, max.z))
    }
}