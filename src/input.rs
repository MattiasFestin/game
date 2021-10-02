

use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::{input::{Axis, Input}};

#[derive(Default)]
pub struct GamepadLobby {
    gamepads: HashSet<Gamepad>,
}

pub fn connection_system(
    mut lobby: ResMut<GamepadLobby>,
    mut gamepad_event: EventReader<GamepadEvent>,
) {
    for event in gamepad_event.iter() {
        match &event {
            GamepadEvent(gamepad, GamepadEventType::Connected) => {
                lobby.gamepads.insert(*gamepad);
                info!("{:?} Connected", gamepad);
            }
            GamepadEvent(gamepad, GamepadEventType::Disconnected) => {
                lobby.gamepads.remove(gamepad);
                info!("{:?} Disconnected", gamepad);
            }
            _ => (),
        }
    }
}

pub fn gamepad_system(
    lobby: Res<GamepadLobby>,
    _button_inputs: Res<Input<GamepadButton>>,
    button_axes: Res<Axis<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    time: Res<Time>,
    mut camera_query: Query<&mut crate::camera::PlayerCamera, With<crate::camera::PlayerCamera>>
) {
    for gamepad in lobby.gamepads.iter().cloned() {
        match camera_query.single_mut() {
            Ok(mut pc) => {
                let pos_speed = pc.position_speed * (time.delta_seconds() as f32);
                let rot_speed = pc.rotation_speed * (time.delta_seconds() as f32);

                let mut xr = Quat::IDENTITY;
                let mut yr = Quat::IDENTITY;

                if let Some(right_stick_x) = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickX)) {
                    if right_stick_x.abs() > 0.1 {
                        xr = Quat::from_rotation_y( - right_stick_x * rot_speed);
                    }
                }

                if let Some(right_stick_y) = axes.get(GamepadAxis(gamepad, GamepadAxisType::RightStickY)) {
                    if right_stick_y.abs() > 0.1 {
                        yr = Quat::from_rotation_x(right_stick_y * rot_speed);
                    }
                }

                let mut xp = 0f32;
                if let Some(left_stick_x) = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX)) {
                    if left_stick_x.abs() > 0.1 {
                        xp = left_stick_x * (time.delta_seconds() as f32);
                    }
                }

                let mut yp = 0f32;
                if let Some(left_stick_y) = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY)) {
                    if left_stick_y.abs() > 0.1 {
                        yp = left_stick_y * (time.delta_seconds() as f32);
                    }
                }

                let mut zp = 0.0f32;
                if let Some(mut right_trigger) = button_axes.get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2)) {
                    if right_trigger.abs() < 0.1 {
                        right_trigger = 0.0f32;
                    }
                    

                    if let Some(mut left_trigger) = button_axes.get(GamepadButton(gamepad, GamepadButtonType::LeftTrigger2)) {
                        if left_trigger.abs() < 0.1 {
                            left_trigger = 0.0f32;
                        }

                        zp = (right_trigger - left_trigger) * (time.delta_seconds() as f32);
                    }
                }
        
                let rotation = pc.rotation * xr * yr; 
                pc.rotation = rotation;
                // let translation = transform.rotation * Vec3::new(xp, zp, -yp) * 0.3;
                pc.position += rotation * Vec3::new(xp, zp, -yp) * pos_speed;
            }
            Err(e) => {
                println!("{:?}", e);                
            }
        }
    }
}

pub fn mouse_keyboard_system(
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if key.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
    }
}