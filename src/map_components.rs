use bevy::ecs::{component::Component, system::Resource};
use crate::biomes::*;

#[derive(Component)]
pub struct SeaTile;

#[derive(Component)]
pub struct LandTile;

#[derive(Component)]
pub struct ShoreTile;

#[derive(Resource)]
pub struct Map {
    pub biome: &'static Biome,
}