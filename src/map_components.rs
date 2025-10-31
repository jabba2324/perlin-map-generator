use bevy::ecs::{component::Component, system::Resource};
use crate::biomes::*;

#[derive(Component)]
pub struct TilePosition {
    pub x: u32,
    pub y: u32,
}

#[derive(Component)]
pub struct SeaTile;

#[derive(Component)]
pub struct LandTile;

#[derive(Component)]
pub struct ShoreTile;

#[derive(Component)]
pub struct Nature;

#[derive(Resource)]
pub struct Map {
    pub biome: &'static Biome,
}