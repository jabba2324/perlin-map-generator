use bevy::prelude::*;

mod map_renderer;
mod map_components;
mod biomes;
mod camera;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (map_renderer::render_map, camera::setup_camera))
        .add_systems(Update, (map_renderer::render_nature.run_if(run_once()), camera::camera_controls))
        .run();
}