use crate::map_components::*;
use bevy::prelude::*;
use rand::Rng;

const TILE_SIZE: u32 = 32;

#[derive(Component)]
pub struct Vehicle {
    pub health: i32,
    pub speed: i32
}

pub fn spawn_tank(
    mut commands: Commands,
    land_tiles: Query<(&Transform, &TilePosition), With<LandTile>>,
    nature: Query<&TilePosition, With<Nature>>,
    asset_server: Res<AssetServer>,
) {
    let nature_positions: Vec<(u32, u32)> = nature.iter().map(|pos| (pos.x, pos.y)).collect();

    let available_tiles: Vec<(&Transform, &TilePosition)> = land_tiles
        .iter()
        .filter(|(_, tile_pos)| !nature_positions.contains(&(tile_pos.x, tile_pos.y)))
        .collect();

    if !available_tiles.is_empty() {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..available_tiles.len());
        let (transform, _) = available_tiles[random_index];

        let tank_texture = asset_server.load("vehicles/tank.png");

        let tank_pos = Vec3::new(transform.translation.x, transform.translation.y, 2.0);
        
        commands.spawn((
            SpriteBundle {
                texture: tank_texture,
                transform: Transform::from_translation(tank_pos),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..default()
                },
                ..default()
            },
            Vehicle{health: 100, speed: 10}
        ));
    }
}
