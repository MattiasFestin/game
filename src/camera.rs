use bevy::prelude::*;

use bevy::render::camera::Camera;
use bevy::{math::Vec3};
use bevy_frustum_culling::FrustumCulling;

use crate::constants::GLOBAL_SCALE;

pub fn setup_camera(mut commands: Commands) {
    let t = Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))
        .looking_at(Vec3::new(10.0, 0.0, 0.0), Vec3::Y);
    commands
        .spawn()
        .insert_bundle(PerspectiveCameraBundle {
            ..Default::default()
        })
        .insert(PlayerCamera {
            position: t.translation,
            rotation: t.rotation,
            ..Default::default()
        })
        .insert(FrustumCulling);
}

#[derive(Debug)]
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
            position_easing: 2.0,
            
            rotation_speed: 0.1,
            position_speed: 1.0
        }
    }
}

pub fn update_camera(
    mut query: Query<(&mut Transform, &PlayerCamera), Changed<PlayerCamera>>,
    time: Res<Time>,
) {
    for (mut t, pc) in query.iter_mut() {
        t.rotation = crate::easing::asymptotic_averaging_rot(
            t.rotation,
            pc.rotation,
            pc.rotation_easing * (time.delta_seconds() as f32)
        );

        t.translation = crate::easing::asymptotic_averaging_3d(
            t.translation, 
            pc.position, 
            pc.position_easing * (time.delta_seconds() as f32)
        );
    }
}