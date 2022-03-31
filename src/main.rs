//  Copyright 2022 Google LLC
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      https://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.

#![warn(clippy::all, clippy::pedantic)]

use bevy::{
    math::ivec3,
    prelude::*,
    render::camera::{ActiveCameras, Camera},
};

use bevy_simple_tilemap::prelude::*;
use rand::Rng;

fn main() {
    App::new()
        // Disable MSAA, as it produces weird rendering artifacts
        .insert_resource(Msaa { samples: 1 })
        .add_plugins(DefaultPlugins)
        .add_plugin(SimpleTileMapPlugin)
        .add_startup_system(setup)
        .add_system(input_system)
        .run();
}

fn input_system(
    active_cameras: Res<ActiveCameras>,
    mut camera_transform_query: Query<(&mut Transform,), With<Camera>>,
    mut tilemap_visible_query: Query<&mut Visibility, With<TileMap>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const MOVE_SPEED: f32 = 1000.0;
    const ZOOM_SPEED: f32 = 10.0;

    if let Some(active_camera_entity) = active_cameras.get("camera_2d").and_then(|ac| ac.entity) {
        if let Ok((mut tf,)) = camera_transform_query.get_mut(active_camera_entity) {
            if keyboard_input.pressed(KeyCode::X) {
                tf.scale -= Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::Z) {
                tf.scale += Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
            }

            if keyboard_input.pressed(KeyCode::A) {
                tf.translation.x -= MOVE_SPEED * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::D) {
                tf.translation.x += MOVE_SPEED * time.delta_seconds();
            }

            if keyboard_input.pressed(KeyCode::S) {
                tf.translation.y -= MOVE_SPEED * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::W) {
                tf.translation.y += MOVE_SPEED * time.delta_seconds();
            }

            if keyboard_input.just_pressed(KeyCode::V) {
                // Toggle visibility
                let mut visible = tilemap_visible_query.iter_mut().next().unwrap();
                visible.is_visible = !visible.is_visible;
            }
        }
    }
}

fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load tilesheet texture and make a texture atlas from it
    let texture_handle = asset_server.load("64x64_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut tilemap = TileMap::default();
    populate_tiles(&mut tilemap);

    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform {
            scale: Vec3::splat(0.75),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(tilemap_bundle);
}

enum TileType {
    Empty = 0,
    Dirt,
    Sandstone,
    Limestone,
    Granite,
    Bedrock,
    Grass = 6,
    Water = 7,
    Border = 8,
    Sky = 9,

    Person = 10,
    Elevator = 30,
    ElevatorHook = 20,
    ElevatorTowerTop = 21,
    ElevatorTowerBottom = 31,
    //ElevatorCable = 32,
}

fn populate_tiles(tm: &mut TileMap) {
    const WIDTH: i32 = 50;
    const DEPTH: i32 = 30;
    const SKY_HEIGHT : i32 = 3;

    let mut rng = rand::thread_rng();

    for x in 0..WIDTH {
        set_tile(tm, x, 2, TileType::Sky);
        set_tile(tm, x, 1, TileType::Sky);
        set_tile(tm, x, 0, TileType::Sky);
        set_tile(tm, x, -1, TileType::Grass);
        for y in -DEPTH..-1 {
            let t = match rng.gen_range(0..100) {
                r if r < 10 => TileType::Sandstone,
                r if r < 20 => TileType::Limestone,
                r if r < 30 => TileType::Granite,
                r if r < 35 => TileType::Bedrock,
                r if r < 40 => TileType::Water,
                _ => TileType::Dirt, 
            };
            set_tile(tm, x, y, t);
        }
    }
    for y in -DEPTH..0 {
        set_tile(tm, WIDTH - 2, y, TileType::Empty);
    }
    for x in -1..WIDTH+1 {
        set_tile(tm, x, SKY_HEIGHT, TileType::Border);
        set_tile(tm, x, -DEPTH-1, TileType::Border);
    }
    for y in -DEPTH..SKY_HEIGHT {
        set_tile(tm, -1, y, TileType::Border);
        set_tile(tm, WIDTH, y, TileType::Border);
    }

    // Person in layer 1.
    tm.set_tile(
        ivec3(WIDTH - 2, 0, 1),
        Some(Tile {
            sprite_index: TileType::Person as u32,
            ..Default::default()
        }),
    );

    // Elevator in layer 2.
    tm.set_tile(
        ivec3(WIDTH - 2, 0, 2),
        Some(Tile {
            sprite_index: TileType::Elevator as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(WIDTH - 2, 1, 2),
        Some(Tile {
            sprite_index: TileType::ElevatorHook as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(WIDTH - 1, 1, 2),
        Some(Tile {
            sprite_index: TileType::ElevatorTowerTop as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(WIDTH - 1, 0, 2),
        Some(Tile {
            sprite_index: TileType::ElevatorTowerBottom as u32,
            ..Default::default()
        }),
    );
}

fn set_tile(tm: &mut TileMap, x: i32, y: i32, t: TileType) {
    tm.set_tile(
        ivec3(x, y, 0),
        Some(Tile {
            sprite_index: t as u32,
            ..Default::default()
        }),
    )
}
