use bevy::prelude::*;


use bevy::{math::Vec3};
use bevy_frustum_culling::FrustumCulling;

use crate::constants::CHUNK_SIZE;

pub fn setup_camera(mut commands: Commands) {
    let offset = if CHUNK_SIZE % 2 == 0 {
        (CHUNK_SIZE / 2) as f32 - 0.5
    } else {
        (CHUNK_SIZE / 2) as f32
    };
    
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(offset, offset, CHUNK_SIZE as f32 * 3.0f32))
            .looking_at(Vec3::new(offset, offset, 0.0), Vec3::Y),
        ..Default::default()
    })
    .insert(PlayerCamera)
    .insert(FrustumCulling);
}

pub struct PlayerCamera;


// impl Default for CameraRotator {
//     fn default() -> Self {
//         Self { 
//             rot_x: Quat::from_rotation_x(0.0f32),
//             rot_y: Quat::from_rotation_y(0.0f32),
//             rot_z: Quat::from_rotation_z(0.0f32),
//         }
//     }
// }

// /// Rotate the meshes to demonstrate how the bounding volumes update
// pub fn camera_rotation_system(time: Res<Time>, mut query: Query<&mut Transform, With<Camera>>) {
//     for mut transform in query.iter_mut() {
        
//         // let scale = Vec3::ONE * ((time.seconds_since_startup() as f32).sin() * 0.3 + 1.0) * 0.3;
//         let rot_x = Quat::from_rotation_x((time.delta_seconds() as f32 / 5.0).sin() / 50.0);
//         let rot_y = Quat::from_rotation_y((time.delta_seconds() as f32 / 3.0).sin() / 50.0);
//         let rot_z = Quat::from_rotation_z((time.delta_seconds() as f32 / 4.0).sin() / 50.0);
//         let r = rot_x * rot_y * rot_z; //transform.rotation.mul_quat(rot_x * rot_y * rot_z);
//         // transform.scale = scale;
//         transform.rotate(r);
//     }
// }