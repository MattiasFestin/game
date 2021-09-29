

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
    mut camera_query: Query<&mut Transform, With<crate::camera::PlayerCamera>>
) {
    for gamepad in lobby.gamepads.iter().cloned() {
        let mut xr = Quat::IDENTITY;
        let mut yr = Quat::IDENTITY;

        let right_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::RightStickX))
            .unwrap();
        if right_stick_x.abs() > 0.1 {
            xr = Quat::from_rotation_y( - right_stick_x * (time.delta_seconds() as f32));
            info!("{:?} RightStickX value is {}", gamepad, right_stick_x);
        }

        let right_stick_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::RightStickY))
            .unwrap();
        if right_stick_y.abs() > 0.1 {
            yr = Quat::from_rotation_x(right_stick_y * (time.delta_seconds() as f32));
            info!("{:?} RightStickY value is {}", gamepad, right_stick_y);
        }

        let mut xp = 0f32;
        let left_stick_x = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickX))
            .unwrap();
        if left_stick_x.abs() > 0.1 {
            xp = left_stick_x;
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_x);
        }

        let mut yp = 0f32;
        let left_stick_y = axes
            .get(GamepadAxis(gamepad, GamepadAxisType::LeftStickY))
            .unwrap();
        if left_stick_y.abs() > 0.1 {
            yp = left_stick_y;
            info!("{:?} LeftStickX value is {}", gamepad, left_stick_y);
        }

        let mut right_trigger = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::RightTrigger2))
            .unwrap();
        if right_trigger.abs() < 0.1 {
            right_trigger = 0.0f32;
        } else {
            info!("{:?} RightTrigger2 value is {}", gamepad, right_trigger);
        }
        let mut left_trigger = button_axes
            .get(GamepadButton(gamepad, GamepadButtonType::LeftTrigger2))
            .unwrap();
        if left_trigger.abs() < 0.1 {
            left_trigger = 0.0f32;
        } else {
            info!("{:?} LeftTrigger2 value is {}", gamepad, left_trigger);
        }

        let zp = right_trigger - left_trigger;

        match camera_query.single_mut() {
            Ok(mut transform) => {
                transform.rotate(xr * yr);
                let translation = transform.rotation * Vec3::new(xp, zp, -yp) * 0.3;
                transform.translation += translation;
            }
            Err(e) => {
                println!("{:?}", e);                
            }
        }
    }
}
