

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::utils::HashSet;
use bevy::{input::{Axis, Input}};

#[derive(Default)]
pub struct GamepadLobby {
    gamepads: HashSet<Gamepad>,
}

pub fn gamepad_connection_system(
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
    mut camera_query: Query<&mut crate::camera::PlayerCamera, With<crate::camera::PlayerCamera>>
) {
    for gamepad in lobby.gamepads.iter().cloned() {
        match camera_query.single_mut() {
            Ok(mut pc) => {
                let pos_speed = pc.position_speed;
                let rot_speed = pc.rotation_speed;

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
                        xp = left_stick_x * pos_speed;
                    }
                }

                let mut yp = 0f32;
                if let Some(left_stick_y) = axes.get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY)) {
                    if left_stick_y.abs() > 0.1 {
                        yp = left_stick_y * pos_speed;
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

                        zp = (right_trigger - left_trigger) * pos_speed;
                    }
                }

                pc.rotation *= xr * yr;
                let rotation = pc.rotation;
                pc.position += rotation * Vec3::new(xp, zp, -yp);
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
    keyboard_input: Res<Input<KeyCode>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<&mut crate::camera::PlayerCamera, With<crate::camera::PlayerCamera>>
) {
    let window = windows.get_primary_mut().unwrap();

    if btn.just_pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }

    if window.cursor_locked() {

        if keyboard_input.just_pressed(KeyCode::Escape) {
            window.set_cursor_lock_mode(false);
            window.set_cursor_visibility(true);
        }

        // match camera_query.single_mut() {
        //     Ok(mut pc) => {
        //         let pos_speed = pc.position_speed;
        //         let rot_speed = pc.rotation_speed;

        //         let mut xr = Quat::IDENTITY;
        //         let mut yr = Quat::IDENTITY;

        //         for event in mouse_motion_events.iter() {
        //             xr = Quat::from_rotation_x( -event.delta.y * 0.2 * rot_speed);
        //             yr = Quat::from_rotation_y( -event.delta.x * 0.2 * rot_speed);
        //         }
            

        //         let mut xp = 0f32;
        //         let mut yp = 0f32;
        //         let mut zp = 0.0f32;
                
        //         if keyboard_input.pressed(KeyCode::W) {
        //             yp = 0.50 * pos_speed;
        //         } else if keyboard_input.pressed(KeyCode::S) {
        //             yp = -0.50 * pos_speed;
        //         }

        //         if keyboard_input.pressed(KeyCode::D) {
        //             xp = 0.50 * pos_speed;
        //         } else if keyboard_input.pressed(KeyCode::A) {
        //             xp = -0.50 * pos_speed;
        //         }

        //         if keyboard_input.pressed(KeyCode::Space) {
        //             zp = 0.50 * pos_speed;
        //         } else if keyboard_input.pressed(KeyCode::LControl) {
        //             zp = -0.50 * pos_speed;
        //         }
        
        //         pc.rotation *= xr * yr;
        //         let rotation = pc.rotation;
        //         pc.position += rotation * Vec3::new(xp, zp, -yp);
        //     }
        //     Err(e) => {
        //         println!("{:?}", e);                
        //     }
        // }
    }
}