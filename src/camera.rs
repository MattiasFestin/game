use bevy::prelude::*;

use bevy::{math::Vec3};
use bevy_frustum_culling::FrustumCulling;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn()
    .insert_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(PlayerCamera)
    .insert(FrustumCulling);
}

pub struct PlayerCamera;