use bevy::{app::Events, prelude::*, window::WindowResized};

#[derive(Default, Debug, Clone, Copy)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32
}

pub fn get_primary_window_size(windows: &Res<Windows>) -> WindowSize {
    let window = windows.get_primary().unwrap();
    let window = Vec2::new(window.width() as f32, window.height() as f32);
    
    return WindowSize {
        width: window.x,
        height: window.y,
    };
}

pub fn setup_window(
    windows: Res<Windows>,
    mut window_size: ResMut<WindowSize>,
) {
    let s = get_primary_window_size(&windows);
    window_size.width = s.width;
    window_size.height = s.height;
}

pub fn resize_notificator(
    mut window_size: ResMut<WindowSize>,
    resize_event: Res<Events<WindowResized>>) {
    let mut reader = resize_event.get_reader();
    for e in reader.iter(&resize_event) {
        window_size.width = e.width;
        window_size.height = e.height;
        println!("width = {} height = {}", e.width, e.height);
    }
}