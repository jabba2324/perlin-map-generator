use bevy::prelude::*;

mod tools {
    pub mod terrain_generator;
}

mod map_v2;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (map_v2::render_map, camera::setup_camera).chain())
        .add_systems(Update, camera::camera_controls)
        .run();
}