use bevy::prelude::*;

mod biomes;
mod camera;
mod controls;
mod map_components;
mod map_renderer;
mod movement;
mod tank;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<controls::DragSelection>()
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
                controls::handle_unit_selection,
                controls::handle_drag_selection,
                movement::set_move_target,
                movement::move_units,
                controls::draw_selection_ui,
                controls::draw_drag_selection,
            ),
        )
        .run();
}
