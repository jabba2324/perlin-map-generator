use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

const TILE_SIZE: u32 = 32;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

// ALPINE
// const SEA_COLOR: Color = Color { r: 120, g: 150, b: 220, a: 255 };
// const SHORE_COLOR: Color = Color { r: 160, g: 130, b: 90, a: 255 };
// const LAND_COLOR: Color = Color { r: 40, g: 80, b: 50, a: 255 };

// DESERT
const SEA_COLOR: Color = Color { r: 120, g: 150, b: 220, a: 255 };
const SHORE_COLOR: Color = Color { r: 130, g: 100, b: 60, a: 255 };
const LAND_COLOR: Color = Color { r: 160, g: 130, b: 90, a: 255 }; 

// TUNDRA
// const SEA_COLOR: Color = Color { r: 120, g: 150, b: 220, a: 255 };
// const SHORE_COLOR: Color = Color { r: 140, g: 145, b: 150, a: 255 };
// const LAND_COLOR: Color = Color { r: 248, g: 248, b: 255, a: 255 };

// ALIEN
// const SEA_COLOR: Color = Color { r: 200, g: 50, b: 30, a: 255 };
// const SHORE_COLOR: Color = Color { r: 65, g: 70, b: 75, a: 255 };
// const LAND_COLOR: Color = Color { r: 25, g: 15, b: 35, a: 255 };

const SEA_THRESHOLD: f64 = -0.5;
const SHORE_THRESHOLD: f64 = -0.4;
const LAND_THRESHOLD: f64 = -0.38;

#[derive(Debug, Clone)]
pub enum TileType {
    Sea,
    Land,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub image_data: Vec<u8>,
    pub tile_type: TileType,
}

impl Tile {
    fn determine_type(image_data: &[u8]) -> TileType {
        let mut sea_count = 0;
        let mut land_count = 0;
        
        for pixel in image_data.chunks(4) {
            let [r, g, b, _] = [pixel[0], pixel[1], pixel[2], pixel[3]];
            
            if r >= SEA_COLOR.r && g >= SEA_COLOR.g && b >= SEA_COLOR.b {
                sea_count += 1;
            } else {
                land_count += 1;
            }
        }
        
        let total_pixels = (TILE_SIZE * TILE_SIZE) as u32;
        let threshold = total_pixels / 2;
        
        if sea_count >= threshold {
            TileType::Sea
        } else {
            TileType::Land
        }
    }
    
    pub fn new(image_data: Vec<u8>) -> Self {
        let tile_type = Self::determine_type(&image_data);
        Self { image_data, tile_type }
    }
}

pub fn generate_map_image() -> Image {
    let perlin = Perlin::new(rand::thread_rng().gen());
    let pixel_width = WIDTH * TILE_SIZE;
    let pixel_height = HEIGHT * TILE_SIZE;
    let mut image_data = Vec::new();
    
    for y in 0..pixel_height {
        for x in 0..pixel_width {
            let noise = perlin.get([x as f64 * 0.0008, y as f64 * 0.0008]);
            let color = match noise {
                n if n < SEA_THRESHOLD => [SEA_COLOR.r, SEA_COLOR.g, SEA_COLOR.b, SEA_COLOR.a],
                n if n < SHORE_THRESHOLD => {
                    let t = (n - SEA_THRESHOLD) / (SHORE_THRESHOLD - SEA_THRESHOLD);
                    let r = (SEA_COLOR.r as f64 * (1.0 - t) + SHORE_COLOR.r as f64 * t) as u8;
                    let g = (SEA_COLOR.g as f64 * (1.0 - t) + SHORE_COLOR.g as f64 * t) as u8;
                    let b = (SEA_COLOR.b as f64 * (1.0 - t) + SHORE_COLOR.b as f64 * t) as u8;
                    [r, g, b, 255]
                },
                n if n < LAND_THRESHOLD => {
                    let t = (n - SHORE_THRESHOLD) / (LAND_THRESHOLD - SHORE_THRESHOLD);
                    let r = (SHORE_COLOR.r as f64 * (1.0 - t) + LAND_COLOR.r as f64 * t) as u8;
                    let g = (SHORE_COLOR.g as f64 * (1.0 - t) + LAND_COLOR.g as f64 * t) as u8;
                    let b = (SHORE_COLOR.b as f64 * (1.0 - t) + LAND_COLOR.b as f64 * t) as u8;
                    [r, g, b, 255]
                },
                _ => [LAND_COLOR.r, LAND_COLOR.g, LAND_COLOR.b, LAND_COLOR.a],
            };
            image_data.extend_from_slice(&color);
        }
    }
    
    Image::new(
        bevy::render::render_resource::Extent3d { 
            width: pixel_width, 
            height: pixel_height, 
            depth_or_array_layers: 1 
        },
        bevy::render::render_resource::TextureDimension::D2,
        image_data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb
    )
}

pub fn render_map(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let full_image = generate_map_image();
    let image_data = &full_image.data;
    
    for tile_y in 0..HEIGHT {
        for tile_x in 0..WIDTH {
            let mut tile_data = Vec::new();
            
            // Extract tile pixels from full image
            for y in 0..TILE_SIZE {
                for x in 0..TILE_SIZE {
                    let pixel_x = tile_x * TILE_SIZE + x;
                    let pixel_y = tile_y * TILE_SIZE + y;
                    let pixel_index = ((pixel_y * WIDTH * TILE_SIZE + pixel_x) * 4) as usize;
                    tile_data.extend_from_slice(&image_data[pixel_index..pixel_index + 4]);
                }
            }
            
            let mut tile: Tile = Tile::new(tile_data);
            
            // Add randomness to create texture variation based on tile type
            let mut rng = rand::thread_rng();
            let mut varied_tile_data = Vec::new();
            
            for pixel in tile.image_data.chunks(4) {
                let [r, g, b, a] = [pixel[0], pixel[1], pixel[2], pixel[3]];
                let variation = rng.gen_range(-8..=8);
                let new_r = (r as i16 + variation).clamp(0, 255) as u8;
                let new_g = (g as i16 + variation).clamp(0, 255) as u8;
                let new_b = (b as i16 + variation).clamp(0, 255) as u8;
                varied_tile_data.extend_from_slice(&[new_r, new_g, new_b, a]);
            }
            
            tile.image_data = varied_tile_data;
            
            let tile_image = Image::new(
                bevy::render::render_resource::Extent3d { 
                    width: TILE_SIZE, 
                    height: TILE_SIZE, 
                    depth_or_array_layers: 1 
                },
                bevy::render::render_resource::TextureDimension::D2,
                tile.image_data.clone(),
                bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb
            );
            
            let tile_handle = images.add(tile_image);
            
            commands.spawn(SpriteBundle {
                texture: tile_handle,
                transform: Transform::from_translation(Vec3::new(
                    tile_x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
                    (HEIGHT - 1 - tile_y) as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
                    0.0,
                )),
                ..default()
            });
        }
    }
}