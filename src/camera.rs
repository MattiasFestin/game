use bevy::prelude::*;

use bevy::{math::Vec3};
use bevy_frustum_culling::FrustumCulling;

pub fn setup_camera(mut commands: Commands) {

    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::ZERO)
            .looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(PlayerCamera)
    .insert(FrustumCulling);
}

pub struct PlayerCamera;