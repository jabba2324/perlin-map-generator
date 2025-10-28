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

const BLUE: Color = Color { r: 120, g: 150, b: 220, a: 255 };
const YELLOW: Color = Color { r: 160, g: 130, b: 90, a: 255 };
const GREEN: Color = Color { r: 40, g: 80, b: 50, a: 255 };

const BLUE_THRESHOLD: f64 = -0.5;
const YELLOW_THRESHOLD: f64 = -0.4;
const GREEN_THRESHOLD: f64 = -0.38;

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
            
            if r >= BLUE.r && g >= BLUE.g && b >= BLUE.b {
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
                n if n < BLUE_THRESHOLD => [BLUE.r, BLUE.g, BLUE.b, BLUE.a],
                n if n < YELLOW_THRESHOLD => {
                    let t = (n - BLUE_THRESHOLD) / (YELLOW_THRESHOLD - BLUE_THRESHOLD);
                    let r = (BLUE.r as f64 * (1.0 - t) + YELLOW.r as f64 * t) as u8;
                    let g = (BLUE.g as f64 * (1.0 - t) + YELLOW.g as f64 * t) as u8;
                    let b = (BLUE.b as f64 * (1.0 - t) + YELLOW.b as f64 * t) as u8;
                    [r, g, b, 255]
                },
                n if n < GREEN_THRESHOLD => {
                    let t = (n - YELLOW_THRESHOLD) / (GREEN_THRESHOLD - YELLOW_THRESHOLD);
                    let r = (YELLOW.r as f64 * (1.0 - t) + GREEN.r as f64 * t) as u8;
                    let g = (YELLOW.g as f64 * (1.0 - t) + GREEN.g as f64 * t) as u8;
                    let b = (YELLOW.b as f64 * (1.0 - t) + GREEN.b as f64 * t) as u8;
                    [r.max(GREEN.r), g.max(GREEN.g), b.max(GREEN.b), 255]
                },
                _ => [GREEN.r, GREEN.g, GREEN.b, GREEN.a],
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