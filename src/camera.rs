use bevy::prelude::*;

use bevy::render::camera::Camera;
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

    pub position_speed: f32,
    pub rotation_speed: f32,

    pub position_easing: f32,
    pub rotation_easing: f32,
}
impl Default for PlayerCamera {
    fn default() -> Self {
        Self { 
            position: Default::default(),
            rotation: Default::default(),
            
            rotation_easing: 10.0,
            position_easing: 20.0,
            
            rotation_speed: 4.0,
            position_speed: 400.0
        }
    }
}

pub fn update_camera(
    mut query: Query<(&mut Transform, &PlayerCamera), Changed<PlayerCamera>>,
    time: Res<Time>,
) {
    for (mut t, pc) in query.iter_mut() {
        t.rotation = crate::easing::asymptotic_averaging_rot(t.rotation, pc.rotation, pc.rotation_easing * (time.delta_seconds() as f32));
        t.translation = crate::easing::asymptotic_averaging_3d(t.translation, pc.position, pc.position_easing * (time.delta_seconds() as f32));
    }
}