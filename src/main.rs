use bevy::prelude::*;

mod biomes;
mod camera;
mod map_components;
mod map_renderer;
mod tank;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (map_renderer::render_map, camera::setup_camera))
        .add_systems(
            Update,
            (
                map_renderer::render_nature.run_if(run_once()),
                tank::spawn_tank
                    .run_if(run_once())
                    .after(map_renderer::render_nature),
            ),
        )
        .add_systems(
            Update,
            (
                camera::move_camera_to_tank.after(tank::spawn_tank),
                camera::camera_controls.after(camera::move_camera_to_tank),
            ),
        )
        .run();
}
