use bevy::prelude::*;

use bevy::{math::Vec3};
use bevy_frustum_culling::FrustumCulling;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn()
    .insert_bundle(PerspectiveCameraBundle {
        ..Default::default()
    })
    .insert(PlayerCamera)
    .insert(FrustumCulling);
}

pub struct PlayerCamera;