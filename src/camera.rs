use bevy::prelude::*;
use bevy::app::AppExit;

const TILE_SIZE: f32 = 32.0;
const WIDTH: f32 = 100.0;
const HEIGHT: f32 = 100.0;
const BORDER: f32 = 10.0;

pub fn setup_camera(
    mut commands: Commands,
    windows: Query<&Window>,
) {
    let window = windows.single();
    let window_center_width = window.width() / 2.0;
    let window_center_height = window.height() / 2.0;
    
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            window_center_width - TILE_SIZE,
            window_center_height - TILE_SIZE,
            0.0,
        )),
        ..default()
    });
}

pub fn camera_controls(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Query<&Window>,
    mut exit: EventWriter<AppExit>,
) {
    let mut camera_transform = camera_query.single_mut();
    let speed = 500.0;
    let window = windows.single();
    let window_center_width = window.width() / 2.0;
    let window_center_height = window.height() / 2.0;
    let edge_threshold = 50.0;
    let map_bound_x: f32 = WIDTH * TILE_SIZE;
    let map_bound_y: f32 = HEIGHT * TILE_SIZE;

    let max_x = map_bound_x - window_center_width + TILE_SIZE + BORDER;
    let min_x = window_center_width - TILE_SIZE - BORDER;
    let max_y = map_bound_y - window_center_height + TILE_SIZE + BORDER;
    let min_y = window_center_height - TILE_SIZE - BORDER;
    
    // Exit game
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
    
    // Keyboard controls
    if keyboard_input.pressed(KeyCode::W) {
        camera_transform.translation.y += speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::S) {
        camera_transform.translation.y -= speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::A) {
        camera_transform.translation.x -= speed * time.delta_seconds();
    }
    if keyboard_input.pressed(KeyCode::D) {
        camera_transform.translation.x += speed * time.delta_seconds();
    }
    
    // Mouse edge scrolling
    if let Some(cursor_pos) = window.cursor_position() {
        if cursor_pos.x < edge_threshold {
            camera_transform.translation.x -= speed * time.delta_seconds();
        }
        if cursor_pos.x > window.width() - edge_threshold {
            camera_transform.translation.x += speed * time.delta_seconds();
        }
        if cursor_pos.y < edge_threshold {
            camera_transform.translation.y += speed * time.delta_seconds();
        }
        if cursor_pos.y > window.height() - edge_threshold {
            camera_transform.translation.y -= speed * time.delta_seconds();
        }
    }
    
    camera_transform.translation.x = camera_transform.translation.x.clamp(min_x, max_x);
    camera_transform.translation.y = camera_transform.translation.y.clamp(min_y, max_y);
}