use bevy::prelude::*;
use crate::tank::{Unit, Selectable, Selected, HealthBar};

#[derive(Resource, Default)]
pub struct DragSelection {
    pub is_dragging: bool,
    pub start_pos: Vec2,
    pub current_pos: Vec2,
}

const TILE_SIZE: u32 = 32;

pub fn handle_unit_selection(
    mut commands: Commands,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    selectable_query: Query<(Entity, &Transform), With<Selectable>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    if mouse_input.just_pressed(MouseButton::Left) {
        let window = windows.single();
        let (camera, camera_transform) = camera_query.single();
        
        if let Some(cursor_pos) = window.cursor_position() {
            if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
                // Check if click is on a selectable object
                for (entity, transform) in selectable_query.iter() {
                    let tank_pos = transform.translation.truncate();
                    let distance = (world_pos - tank_pos).length();
                    
                    if distance < TILE_SIZE as f32 / 2.0 {
                        // Deselect all first, then select this one
                        for selected_entity in selected_query.iter() {
                            commands.entity(selected_entity).remove::<Selected>();
                        }
                        commands.entity(entity).insert(Selected);
                        break;
                    }
                }
            }
        }
    }
    
    // Right click to deselect all
    if mouse_input.just_pressed(MouseButton::Right) {
        for entity in selected_query.iter() {
            commands.entity(entity).remove::<Selected>();
        }
    }
}

pub fn handle_drag_selection(
    mut commands: Commands,
    mut drag_selection: ResMut<DragSelection>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Query<&Window>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    selectable_query: Query<(Entity, &Transform), With<Selectable>>,
    selected_query: Query<Entity, With<Selected>>,
) {
    let window = windows.single();
    let (camera, camera_transform) = camera_query.single();
    
    if let Some(cursor_pos) = window.cursor_position() {
        if let Some(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_pos) {
            if mouse_input.just_pressed(MouseButton::Left) {
                drag_selection.is_dragging = true;
                drag_selection.start_pos = world_pos;
                drag_selection.current_pos = world_pos;
            }
            
            if drag_selection.is_dragging {
                drag_selection.current_pos = world_pos;
            }
            
            if mouse_input.just_released(MouseButton::Left) {
                // Only do rectangle selection if mouse actually moved (dragged)
                let drag_distance = (drag_selection.start_pos - drag_selection.current_pos).length();
                if drag_selection.is_dragging && drag_distance > 5.0 {
                    // Select objects inside rectangle
                    let min_x = drag_selection.start_pos.x.min(drag_selection.current_pos.x);
                    let max_x = drag_selection.start_pos.x.max(drag_selection.current_pos.x);
                    let min_y = drag_selection.start_pos.y.min(drag_selection.current_pos.y);
                    let max_y = drag_selection.start_pos.y.max(drag_selection.current_pos.y);
                    
                    // Deselect all first
                    for entity in selected_query.iter() {
                        commands.entity(entity).remove::<Selected>();
                    }
                    
                    // Select objects in rectangle
                    for (entity, transform) in selectable_query.iter() {
                        let pos = transform.translation.truncate();
                        if pos.x >= min_x && pos.x <= max_x && pos.y >= min_y && pos.y <= max_y {
                            commands.entity(entity).insert(Selected);
                        }
                    }
                }
                drag_selection.is_dragging = false;
            }
        }
    }
}

pub fn draw_drag_selection(
    mut gizmos: Gizmos,
    drag_selection: Res<DragSelection>,
) {
    if drag_selection.is_dragging {
        let min_x = drag_selection.start_pos.x.min(drag_selection.current_pos.x);
        let max_x = drag_selection.start_pos.x.max(drag_selection.current_pos.x);
        let min_y = drag_selection.start_pos.y.min(drag_selection.current_pos.y);
        let max_y = drag_selection.start_pos.y.max(drag_selection.current_pos.y);
        
        let center = Vec2::new((min_x + max_x) / 2.0, (min_y + max_y) / 2.0);
        let size = Vec2::new(max_x - min_x, max_y - min_y);
        
        gizmos.rect_2d(center, 0.0, size, Color::WHITE);
    }
}

pub fn draw_selection_ui(
    mut commands: Commands,
    mut gizmos: Gizmos,
    selected_tanks: Query<(&Transform, &Unit), With<Selected>>,
    health_bars: Query<Entity, With<HealthBar>>,
) {
    // Remove existing health bars
    for entity in health_bars.iter() {
        commands.entity(entity).despawn();
    }
    
    for (transform, unit) in selected_tanks.iter() {
        let pos = transform.translation.truncate();
        let size = TILE_SIZE as f32;
        
        draw_selection_corners(&mut gizmos, pos, size);
        
        // Spawn health bar sprites
        let health_bar_width = size;
        let health_bar_height = 6.0;
        let health_bar_pos = pos + Vec2::new(0.0, size / 2.0 + 8.0);
        
        // Background (red)
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::RED,
                    custom_size: Some(Vec2::new(health_bar_width, health_bar_height)),
                    ..default()
                },
                transform: Transform::from_translation(health_bar_pos.extend(3.0)),
                ..default()
            },
            HealthBar,
        ));
        
        // Foreground (green)
        let health_percent = unit.health as f32 / 100.0;
        let health_width = health_bar_width * health_percent;
        let health_pos = health_bar_pos - Vec2::new((health_bar_width - health_width) / 2.0, 0.0);
        
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::GREEN,
                    custom_size: Some(Vec2::new(health_width, health_bar_height)),
                    ..default()
                },
                transform: Transform::from_translation(health_pos.extend(4.0)),
                ..default()
            },
            HealthBar,
        ));
    }
}



fn draw_selection_corners(gizmos: &mut Gizmos, pos: Vec2, size: f32) {
    let corner_size = 8.0;
    let half_size = (size + 4.0) / 2.0;
    
    // Top-left corner
    gizmos.line_2d(
        pos + Vec2::new(-half_size, half_size),
        pos + Vec2::new(-half_size + corner_size, half_size),
        Color::WHITE
    );
    gizmos.line_2d(
        pos + Vec2::new(-half_size, half_size),
        pos + Vec2::new(-half_size, half_size - corner_size),
        Color::WHITE
    );
    
    // Top-right corner
    gizmos.line_2d(
        pos + Vec2::new(half_size, half_size),
        pos + Vec2::new(half_size - corner_size, half_size),
        Color::WHITE
    );
    gizmos.line_2d(
        pos + Vec2::new(half_size, half_size),
        pos + Vec2::new(half_size, half_size - corner_size),
        Color::WHITE
    );
    
    // Bottom-left corner
    gizmos.line_2d(
        pos + Vec2::new(-half_size, -half_size),
        pos + Vec2::new(-half_size + corner_size, -half_size),
        Color::WHITE
    );
    gizmos.line_2d(
        pos + Vec2::new(-half_size, -half_size),
        pos + Vec2::new(-half_size, -half_size + corner_size),
        Color::WHITE
    );
    
    // Bottom-right corner
    gizmos.line_2d(
        pos + Vec2::new(half_size, -half_size),
        pos + Vec2::new(half_size - corner_size, -half_size),
        Color::WHITE
    );
    gizmos.line_2d(
        pos + Vec2::new(half_size, -half_size),
        pos + Vec2::new(half_size, -half_size + corner_size),
        Color::WHITE
    );
}