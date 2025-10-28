use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::tools;

const TILE_SIZE: u32 = 32;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

#[derive(Resource, Serialize, Deserialize, Debug)]
pub struct MapData {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<TileData>,
    pub nature: Vec<NatureData>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NatureData {
    pub x: u32,
    pub y: u32,
    pub sprite: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TileData {
    pub x: u32,
    pub y: u32,
    pub sprite: String,
}

pub fn render_map(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let perlin = noise::Perlin::new(rand::thread_rng().gen());
    
    let map_data = MapData {
        width: WIDTH,
        height: HEIGHT,
        tiles: vec![],
        nature: vec![],
    };

    // Generate texture variants
    let mut grass_handles = Vec::new();
    let mut water_handles = Vec::new();
    let mut desert_handles = Vec::new();
    let mut snow_handles = Vec::new();
    
    for _ in 0..3 {
        let grass_img = tools::terrain_generator::generate_grass_texture(TILE_SIZE as u32, TILE_SIZE as u32);
        grass_handles.push(images.add(grass_img));
        
        let water_img = tools::terrain_generator::generate_water_texture(TILE_SIZE as u32, TILE_SIZE as u32);
        water_handles.push(images.add(water_img));
        
        let desert_img = tools::terrain_generator::generate_desert_texture(TILE_SIZE as u32, TILE_SIZE as u32);
        desert_handles.push(images.add(desert_img));
        
        let snow_img = tools::terrain_generator::generate_snow_texture(TILE_SIZE as u32, TILE_SIZE as u32);
        snow_handles.push(images.add(snow_img));
    }
        
    // Generate all textures and store in perlin_map
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let noise = perlin.get([x as f64 * 0.1 / 2.5, y as f64 * 0.1 / 2.5]);
            
            let mut rng = rand::thread_rng();
            let handle = match noise {
                n if n < -0.5 => water_handles[rng.gen_range(0..water_handles.len())].clone(),
                n if n < -0.35 => images.add(tools::terrain_generator::generate_blended_texture(TILE_SIZE, TILE_SIZE, &perlin, x, y, WIDTH, HEIGHT)),
                n if n < 0.8 => grass_handles[rng.gen_range(0..grass_handles.len())].clone(),
                _ => grass_handles[rng.gen_range(0..grass_handles.len())].clone(),
            };
                                    
            commands.spawn(SpriteBundle {
                texture: handle,
                transform: Transform::from_translation(Vec3::new(
                    (x * TILE_SIZE) as f32,
                    (y * TILE_SIZE) as f32,
                    0.0,
                )),
                ..default()
            });
        }
    }

    commands.insert_resource(map_data);
}