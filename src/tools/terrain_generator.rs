use bevy::prelude::*;
use image::{ImageBuffer, Rgb};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub struct Tile {
    pub noise_value: f64,
}

pub fn generate_grass_texture(width: u32, height: u32) -> Image {
    let mut rng = rand::thread_rng();
    let img_buffer = ImageBuffer::from_fn(width, height, |_, _| {
        let base_green = 100 + rng.gen_range(0..30);
        let noise = rng.gen_range(-15..15);
        let green = (base_green + noise).clamp(85, 130) as u8;
        Rgb([60, green, 70])
    });
    Image::from_dynamic(image::DynamicImage::ImageRgb8(img_buffer), true)
}

pub fn generate_water_texture(width: u32, height: u32) -> Image {
    let mut rng = rand::thread_rng();
    let img_buffer = ImageBuffer::from_fn(width, height, |_, _| {
        let base_blue = 90 + rng.gen_range(0..25);
        let noise = rng.gen_range(-12..12);
        let blue = (base_blue + noise).clamp(75, 115) as u8;
        Rgb([40, 60, blue])
    });
    Image::from_dynamic(image::DynamicImage::ImageRgb8(img_buffer), true)
}

pub fn generate_desert_texture(width: u32, height: u32) -> Image {
    let mut rng = rand::thread_rng();
    let img_buffer = ImageBuffer::from_fn(width, height, |_, _| {
        let base_yellow = 160 + rng.gen_range(0..30);
        let noise = rng.gen_range(-15..15);
        let yellow = (base_yellow + noise).clamp(145, 190) as u8;
        Rgb([yellow, yellow - 20, 80])
    });
    Image::from_dynamic(image::DynamicImage::ImageRgb8(img_buffer), true)
}

pub fn generate_snow_texture(width: u32, height: u32) -> Image {
    let mut rng = rand::thread_rng();
    let img_buffer = ImageBuffer::from_fn(width, height, |_, _| {
        let base_white = 210 + rng.gen_range(0..25);
        let noise = rng.gen_range(-8..8);
        let white = (base_white + noise).clamp(200, 235) as u8;
        Rgb([white, white, white - 3])
    });
    Image::from_dynamic(image::DynamicImage::ImageRgb8(img_buffer), true)
}


pub fn generate_blended_texture(tile_width: u32, tile_height: u32, perlin: &Perlin, x: u32, y: u32, map_width: u32, map_height: u32) -> Image {
    let center_noise = perlin.get([x as f64 * 0.1 / 2.5, y as f64 * 0.1 / 2.5]);
    let mut neighbors = Vec::new();
    for dy in -1..=1 {
        for dx in -1..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = (x as i32 + dx).clamp(0, map_width as i32 - 1) as usize;
            let ny = (y as i32 + dy).clamp(0, map_height as i32 - 1) as usize;
            neighbors.push(perlin.get([nx as f64 * 0.1 / 2.5, ny as f64 * 0.1 / 2.5]));
        }
    }
    
    let mut rng = rand::thread_rng();
    let avg_noise = (center_noise + neighbors.iter().sum::<f64>()) / (neighbors.len() as f64 + 1.0);
    
    let img_buffer = ImageBuffer::from_fn(tile_width, tile_height, |_, _| {
        let blend_factor = (avg_noise + 1.0) / 2.0; // Normalize to 0-1
        
        match avg_noise {
            n if n < -0.3 => {
                // Water-Desert blend
                let water_color = [40, 60, 100];
                let desert_color = [170, 150, 80];
                let blend = blend_factor.clamp(0.0, 1.0);
                let r = (water_color[0] as f64 * (1.0 - blend) + desert_color[0] as f64 * blend) as u8;
                let g = (water_color[1] as f64 * (1.0 - blend) + desert_color[1] as f64 * blend) as u8;
                let b = (water_color[2] as f64 * (1.0 - blend) + desert_color[2] as f64 * blend) as u8;
                Rgb([r, g, b])
            },
            n if n < 0.5 => {
                // Desert-Grass blend
                let desert_color = [170, 150, 80];
                let grass_color = [60, 110, 70];
                let blend = (blend_factor - 0.35).clamp(0.0, 1.0) / 0.65;
                let r = (desert_color[0] as f64 * (1.0 - blend) + grass_color[0] as f64 * blend) as u8;
                let g = (desert_color[1] as f64 * (1.0 - blend) + grass_color[1] as f64 * blend) as u8;
                let b = (desert_color[2] as f64 * (1.0 - blend) + grass_color[2] as f64 * blend) as u8;
                Rgb([r, g, b])
            },
            _ => {
                // Default grass
                let base_green = 100 + rng.gen_range(0..30);
                let noise = rng.gen_range(-15..15);
                let green = (base_green + noise).clamp(85, 130) as u8;
                Rgb([60, green, 70])
            }
        }
    });
    Image::from_dynamic(image::DynamicImage::ImageRgb8(img_buffer), true)
}