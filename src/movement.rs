use bevy::prelude::*;
use crate::tank::{Unit, Selectable, Selected};
use crate::map_components::{SeaTile, Nature};

#[derive(Component)]
pub struct MoveTarget {
    pub target: Vec2,
}

const TILE_SIZE: u32 = 32;

pub fn set_move_target(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    selected_units: Query<Entity, (With<Selected>, With<Selectable>)>,
    selectable_query: Query<&Transform, (With<Selectable>, Without<Selected>)>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_query.single();
        
        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                // Check if click is on a selectable object
                let mut clicked_on_unit = false;
                for transform in selectable_query.iter() {
                    let unit_pos = transform.translation.truncate();
                    let distance = (world_pos - unit_pos).length();
                    
                    if distance < TILE_SIZE as f32 / 2.0 {
                        clicked_on_unit = true;
                        break;
                    }
                }
                
                // If didn't click on a unit, set move target for selected units
                if !clicked_on_unit {
                    for entity in selected_units.iter() {
                        commands.entity(entity).insert(MoveTarget { target: world_pos });
                    }
                }
            }
        }
    }
}

pub fn move_units(
    mut commands: Commands,
    time: Res<Time>,
    mut units: Query<(Entity, &mut Transform, &Unit, &MoveTarget)>,
    sea_tiles: Query<&Transform, (With<SeaTile>, Without<Unit>)>,
    nature_objects: Query<&Transform, (With<Nature>, Without<Unit>)>,
) {
    for (entity, mut transform, unit, move_target) in units.iter_mut() {
        let current_pos = transform.translation.truncate();
        let target_pos = move_target.target;
        let distance = (target_pos - current_pos).length();
        
        if distance > 1.0 {
            let direction = (target_pos - current_pos).normalize();
            let move_distance = unit.speed * time.delta_seconds();
            let new_pos = current_pos + direction * move_distance;
            
            // Check if new position would collide with sea or nature
            let mut can_move = true;
            
            // Check sea tiles
            for sea_transform in sea_tiles.iter() {
                let sea_pos = sea_transform.translation.truncate();
                if (new_pos - sea_pos).length() < TILE_SIZE as f32 {
                    can_move = false;
                    break;
                }
            }
            
            // Check nature objects
            if can_move {
                for nature_transform in nature_objects.iter() {
                    let nature_pos = nature_transform.translation.truncate();
                    if (new_pos - nature_pos).length() < TILE_SIZE as f32 {
                        can_move = false;
                        break;
                    }
                }
            }
            
            if can_move {
                transform.translation = new_pos.extend(transform.translation.z);
            } else {
                // Can't move directly, remove target (simple collision avoidance)
                commands.entity(entity).remove::<MoveTarget>();
            }
        } else {
            // Reached target, remove MoveTarget component
            commands.entity(entity).remove::<MoveTarget>();
        }
    }
}