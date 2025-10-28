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

const BLUE: Color = Color { r: 20, g: 40, b: 80, a: 255 };
const YELLOW: Color = Color { r: 170, g: 150, b: 80, a: 255 };
const GREEN: Color = Color { r: 60, g: 110, b: 70, a: 255 };

const BLUE_THRESHOLD: f64 = -0.5;
const YELLOW_THRESHOLD: f64 = -0.4;
const GREEN_THRESHOLD: f64 = -0.38;

pub fn render_map(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
) {
    let perlin = Perlin::new(rand::thread_rng().gen());
    let pixel_width = WIDTH * TILE_SIZE;
    let pixel_height = HEIGHT * TILE_SIZE;
    let mut noise_map = Vec::new();
    
    // Generate perlin noise for each pixel
    for y in 0..pixel_height {
        let mut row = Vec::new();
        for x in 0..pixel_width {
            let noise = perlin.get([x as f64 * 0.0008, y as f64 * 0.0008]);
            row.push(noise);
        }
        noise_map.push(row);
    }
    
    // Create single large texture from noise map
    let mut image_data = Vec::new();
    for y in 0..pixel_height {
        for x in 0..pixel_width {
            let noise_value = noise_map[y as usize][x as usize];
            let color = match noise_value {
                n if n < BLUE_THRESHOLD => [BLUE.r, BLUE.g, BLUE.b, BLUE.a],
                n if n < YELLOW_THRESHOLD => {
                    // Gradient from dark blue to yellow (clamped to not exceed YELLOW)
                    let t = (n - BLUE_THRESHOLD) / (YELLOW_THRESHOLD - BLUE_THRESHOLD);
                    let r = (BLUE.r as f64 * (1.0 - t) + YELLOW.r as f64 * t) as u8;
                    let g = (BLUE.g as f64 * (1.0 - t) + YELLOW.g as f64 * t) as u8;
                    let b = (BLUE.b as f64 * (1.0 - t) + YELLOW.b as f64 * t) as u8;
                    [r.min(YELLOW.r), g.min(YELLOW.g), b.min(YELLOW.b), 255]
                },
                n if n < GREEN_THRESHOLD => {
                    // Gradient from yellow to green (clamped to not go darker than GREEN)
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
    
    let image = Image::new(
        bevy::render::render_resource::Extent3d { 
            width: pixel_width, 
            height: pixel_height, 
            depth_or_array_layers: 1 
        },
        bevy::render::render_resource::TextureDimension::D2,
        image_data,
        bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb
    );
    
    let texture_handle = images.add(image);
    
    commands.spawn(SpriteBundle {
        texture: texture_handle,
        transform: Transform::from_translation(Vec3::new(
            (pixel_width as f32) / 2.0,
            (pixel_height as f32) / 2.0,
            0.0,
        )),
        ..default()
    });
}