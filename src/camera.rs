use bevy::prelude::*;

use bevy::render::camera::Camera;
use bevy::{math::Vec3};
use bevy_frustum_culling::FrustumCulling;

pub fn setup_camera(mut commands: Commands) {
    let t = Transform::from_translation(Vec3::new(0.0, 0.0, 20.0)).looking_at(Vec3::ZERO, Vec3::Y);
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
            position_easing: 5.0,
            
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
        if t.rotation.angle_between(pc.rotation) < 10.0 {
            t.rotation = crate::easing::asymptotic_averaging_rot(
                t.rotation,
                pc.rotation,
                pc.rotation_easing * (time.delta_seconds() as f32)
            );
        } else {
            t.rotation = pc.rotation;
        }

        if t.translation.distance(pc.position) < 10.0 {
            t.translation = crate::easing::asymptotic_averaging_3d(
                t.translation, 
                pc.position, 
                pc.position_easing * (time.delta_seconds() as f32)
            );
        } else {
            t.translation = pc.position;
        }
    }
}