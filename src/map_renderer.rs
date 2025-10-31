use crate::biomes::*;
use crate::map_components::*;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;

const TILE_SIZE: u32 = 32;
const WIDTH: u32 = 100;
const HEIGHT: u32 = 100;

const SEA_THRESHOLD: f64 = -0.5;
const SHORE_THRESHOLD: f64 = -0.4;
const LAND_THRESHOLD: f64 = -0.38;

fn get_random_biome(rng: &mut rand::rngs::ThreadRng) -> &'static Biome {
    match rng.gen_range(0..4) {
        0 => &ALPINE,
        1 => &DESERT,
        2 => &TUNDRA,
        _ => &ALIEN,
    }
}

pub fn generate_tile_map(rng: &mut rand::rngs::ThreadRng) -> Vec<Vec<f64>> {
    let perlin = Perlin::new(rng.gen());
    let mut tile_map = Vec::with_capacity(HEIGHT as usize);

    for y in 0..HEIGHT {
        let mut row = Vec::with_capacity(WIDTH as usize);
        for x in 0..WIDTH {
            let noise = perlin.get([x as f64 * 0.1, y as f64 * 0.1]);
            row.push(noise);
        }
        tile_map.push(row);
    }
    tile_map
}

pub fn colourize_noise(biome: &Biome, noise: f64) -> [u8; 4] {
    let color = match noise {
        n if n < SEA_THRESHOLD => [
            biome.sea_color.r,
            biome.sea_color.g,
            biome.sea_color.b,
            biome.sea_color.a,
        ],
        n if n < SHORE_THRESHOLD => {
            let t = (n - SEA_THRESHOLD) / (SHORE_THRESHOLD - SEA_THRESHOLD);
            let r = (biome.sea_color.r as f64 * (1.0 - t) + biome.shore_color.r as f64 * t) as u8;
            let g = (biome.sea_color.g as f64 * (1.0 - t) + biome.shore_color.g as f64 * t) as u8;
            let b = (biome.sea_color.b as f64 * (1.0 - t) + biome.shore_color.b as f64 * t) as u8;
            [r, g, b, 255]
        }
        n if n < LAND_THRESHOLD => {
            let t = (n - SHORE_THRESHOLD) / (LAND_THRESHOLD - SHORE_THRESHOLD);
            let r = (biome.shore_color.r as f64 * (1.0 - t) + biome.land_color.r as f64 * t) as u8;
            let g = (biome.shore_color.g as f64 * (1.0 - t) + biome.land_color.g as f64 * t) as u8;
            let b = (biome.shore_color.b as f64 * (1.0 - t) + biome.land_color.b as f64 * t) as u8;
            [r, g, b, 255]
        }
        _ => [
            biome.land_color.r,
            biome.land_color.g,
            biome.land_color.b,
            biome.land_color.a,
        ],
    };
    color
}

pub fn render_map(mut commands: Commands, mut images: ResMut<Assets<Image>>) {
    let mut rng = rand::thread_rng();
    let biome = get_random_biome(&mut rng);
    let perlin = Perlin::new(rng.gen());
    
    let mut land_entities = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    let mut sea_entities = Vec::with_capacity((WIDTH * HEIGHT) as usize);
    let mut shore_entities = Vec::with_capacity((WIDTH * HEIGHT) as usize);

    for tile_y in 0..HEIGHT {
        for tile_x in 0..WIDTH {
            let mut tile_data = Vec::with_capacity((TILE_SIZE * TILE_SIZE * 4) as usize);
            let mut sea_count = 0;
            let mut shore_count = 0;
            let mut land_count = 0;

            // Extract tile pixels from full image and count tile types
            for tile_pixel_y in 0..TILE_SIZE {
                for tile_pixel_x in 0..TILE_SIZE {
                    let pixel_x = tile_x * TILE_SIZE + tile_pixel_x;
                    let pixel_y = tile_y * TILE_SIZE + tile_pixel_y;

                    let noise =
                        perlin.get([pixel_x as f64 * 0.0008, pixel_y as f64 * 0.0008]);
                    let colourized_pixel = colourize_noise(biome, noise);

                    let [r, g, b, a] = [colourized_pixel[0], colourized_pixel[1], colourized_pixel[2], colourized_pixel[3]];

                    match (r, g, b) {
                        (sr, sg, sb)
                            if sr == biome.sea_color.r
                                && sg == biome.sea_color.g
                                && sb == biome.sea_color.b =>
                        {
                            sea_count += 1
                        }
                        (lr, lg, lb)
                            if lr == biome.land_color.r
                                && lg == biome.land_color.g
                                && lb == biome.land_color.b =>
                        {
                            land_count += 1
                        }
                        _ => shore_count += 1,
                    }

                    let variation = rng.gen_range(-8..=8);
                    let new_r = (r as i16 + variation).clamp(0, 255) as u8;
                    let new_g = (g as i16 + variation).clamp(0, 255) as u8;
                    let new_b = (b as i16 + variation).clamp(0, 255) as u8;

                    tile_data.extend_from_slice(&[new_r, new_g, new_b, a]);
                }
            }

            let total_pixels = (TILE_SIZE * TILE_SIZE) as u32;
            let threshold = total_pixels / 2;

            let tile_image = Image::new(
                bevy::render::render_resource::Extent3d {
                    width: TILE_SIZE,
                    height: TILE_SIZE,
                    depth_or_array_layers: 1,
                },
                bevy::render::render_resource::TextureDimension::D2,
                tile_data,
                bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb,
            );

            let tile_handle = images.add(tile_image);

            match (sea_count >= threshold, shore_count >= threshold, land_count >= threshold) {
                (true, false, false) => sea_entities.push((
                    SpriteBundle {
                        texture: tile_handle,
                        transform: Transform::from_translation(Vec3::new(
                            tile_x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
                            (HEIGHT - 1 - tile_y) as f32 * TILE_SIZE as f32
                                + TILE_SIZE as f32 / 2.0,
                            0.0,
                        )),
                        ..default()
                    },
                    SeaTile,
                    TilePosition { x: tile_x, y: tile_y },
                )),
                (false, true, false) => shore_entities.push((
                    SpriteBundle {
                        texture: tile_handle,
                        transform: Transform::from_translation(Vec3::new(
                            tile_x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
                            (HEIGHT - 1 - tile_y) as f32 * TILE_SIZE as f32
                                + TILE_SIZE as f32 / 2.0,
                            0.0,
                        )),
                        ..default()
                    },
                    ShoreTile,
                    TilePosition { x: tile_x, y: tile_y },
                )),
                (false, false, true) => land_entities.push((
                    SpriteBundle {
                        texture: tile_handle,
                        transform: Transform::from_translation(Vec3::new(
                            tile_x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
                            (HEIGHT - 1 - tile_y) as f32 * TILE_SIZE as f32
                                + TILE_SIZE as f32 / 2.0,
                            0.0,
                        )),
                        ..default()
                    },
                    LandTile,
                    TilePosition { x: tile_x, y: tile_y },
                )),
                _ => land_entities.push((
                    SpriteBundle {
                        texture: tile_handle,
                        transform: Transform::from_translation(Vec3::new(
                            tile_x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
                            (HEIGHT - 1 - tile_y) as f32 * TILE_SIZE as f32
                                + TILE_SIZE as f32 / 2.0,
                            0.0,
                        )),
                        ..default()
                    },
                    LandTile,
                    TilePosition { x: tile_x, y: tile_y },
                )),
            };
        }
    }
    commands.spawn_batch(sea_entities);
    commands.spawn_batch(shore_entities);
    commands.spawn_batch(land_entities);
    commands.insert_resource(Map { biome });
}

pub fn render_nature(
    mut commands: Commands,
    land_tiles: Query<(&Transform, &TilePosition), With<LandTile>>,
    asset_server: Res<AssetServer>,
    map: Res<Map>,
) {
    let mut rng = rand::thread_rng();
    let biome = map.biome;
    let nature_map = generate_tile_map(&mut rng);
    let mut entities: Vec<_> = Vec::with_capacity((WIDTH * HEIGHT) as usize);

    let rock1 = asset_server.load(&format!("{}{}", biome.asset_path, "rock1.png"));
    let rock2 = asset_server.load(&format!("{}{}", biome.asset_path, "rock2.png"));
    let rock3 = asset_server.load(&format!("{}{}", biome.asset_path, "rock3.png"));

    let tree1 = asset_server.load(&format!("{}{}", biome.asset_path, "tree1.png"));
    let tree2 = asset_server.load(&format!("{}{}", biome.asset_path, "tree2.png"));
    let tree3 = asset_server.load(&format!("{}{}", biome.asset_path, "tree3.png"));

    println!("Found {} land tiles", land_tiles.iter().count());

    for (transform, tile_pos) in land_tiles.iter() {
        let noise_value = nature_map[tile_pos.y as usize][tile_pos.x as usize];

        let nature_handle = match noise_value {
            n if n > 0.1 && n < 0.11 => Some(match rng.gen_range(1..=3) {
                1 => rock1.clone(),
                2 => rock2.clone(),
                _ => rock3.clone(),
            }),
            n if n > 0.8 => Some(match rng.gen_range(1..=3) {
                1 => tree1.clone(),
                2 => tree2.clone(),
                _ => tree3.clone(),
            }),
            _ => None,
        };

        if let Some(handle) = nature_handle {
            entities.push((SpriteBundle {
                texture: handle,
                transform: Transform::from_translation(Vec3::new(
                    transform.translation.x,
                    transform.translation.y,
                    1.0,
                )),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                    ..default()
                },
                ..default()
            }, Nature, TilePosition { x: tile_pos.x, y: tile_pos.y }));
        }
    }
    commands.spawn_batch(entities);
}
