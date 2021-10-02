use bevy::prelude::*;

use bevy::{math::Vec3};
use bevy_frustum_culling::FrustumCulling;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn()
    .insert_bundle(PerspectiveCameraBundle {
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    })
    .insert(PlayerCamera::default())
    .insert(FrustumCulling);
}

pub struct PlayerCamera {
    pub position: Vec3,
    pub rotation: Quat,
    pub speed: f32
}
impl Default for PlayerCamera {
    fn default() -> Self {
        Self { position: Default::default(), rotation: Default::default(), speed: 0.2 }
    }
}

pub fn update_camera(
    mut query: Query<(&mut Transform, &PlayerCamera), Changed<PlayerCamera>>
) {
    for (mut t, pc) in query.iter_mut() {
        t.rotation = crate::easing::asymptotic_averaging_rot(t.rotation, pc.rotation, pc.speed);
        t.translation = crate::easing::asymptotic_averaging_3d(t.translation, pc.position, pc.speed);
    }
}