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

use bevy::{math::ivec3, prelude::*};

use bevy_simple_tilemap::prelude::*;

mod constants;
use constants::*;

mod model;
use model::elevator::Elevator;
use model::map::{Map, TileType};
use model::player::Player;

mod systems;

fn main() {
    App::new()
        // Disable MSAA, as it produces weird rendering artifacts
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(Elevator::new(MAX_ELEVATOR_DEPTH as u32))
        .insert_resource(Player::new(PLAYER_START_X, PLAYER_START_Y))
        .add_plugins(DefaultPlugins)
        .add_plugin(SimpleTileMapPlugin)
        .add_startup_system(setup)
        .add_system(systems::input::player_input)
        .add_system(systems::render_player::show_player)
        .run();
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

    let mut cam = OrthographicCameraBundle::new_2d();
    cam.transform.translation.x = 1800f32;
    cam.transform.translation.y = -200f32;
    commands.spawn_bundle(cam);
    commands.spawn_bundle(tilemap_bundle);
}

fn populate_tiles(tm: &mut TileMap) {
    //let mut rng = rand::thread_rng();

    let m: Map = Map::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    for x in 0..MAP_WIDTH {
        for y in 0..MAP_HEIGHT {
            set_tile(tm, x, -y, m.tile(x, y));
        }
    }

    // Person in layer 1.
    tm.set_tile(
        ivec3(MAP_WIDTH - 3, -5, 1),
        Some(Tile {
            sprite_index: SpriteIndex::Person as u32,
            ..Default::default()
        }),
    );

    // Elevator in layer 2.
    tm.set_tile(
        ivec3(MAP_WIDTH - 3, -5, 2),
        Some(Tile {
            sprite_index: SpriteIndex::Elevator as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(MAP_WIDTH - 3, -4, 2),
        Some(Tile {
            sprite_index: SpriteIndex::ElevatorCable as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(MAP_WIDTH - 3, -3, 2),
        Some(Tile {
            sprite_index: SpriteIndex::ElevatorCable as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(MAP_WIDTH - 3, -2, 2),
        Some(Tile {
            sprite_index: SpriteIndex::ElevatorHook as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(MAP_WIDTH - 2, -2, 2),
        Some(Tile {
            sprite_index: SpriteIndex::ElevatorTowerTop as u32,
            ..Default::default()
        }),
    );
    tm.set_tile(
        ivec3(MAP_WIDTH - 2, -3, 2),
        Some(Tile {
            sprite_index: SpriteIndex::ElevatorTowerBottom as u32,
            ..Default::default()
        }),
    );
}

fn set_tile(tm: &mut TileMap, x: i32, y: i32, t: TileType) {
    let si = match t {
        TileType::Empty => SpriteIndex::Empty as u32,
        TileType::Sky => SpriteIndex::Sky as u32,
        TileType::Grass => SpriteIndex::Grass as u32,
        TileType::Dirt => SpriteIndex::Dirt as u32,
        TileType::Rock { hardness } => {
            (SpriteIndex::Stone0 as u8 + core::cmp::min(hardness, 3)) as u32
        }
        TileType::Water => SpriteIndex::Water as u32,
        _ => SpriteIndex::Border as u32,
    };
    tm.set_tile(
        ivec3(x, y, 0),
        Some(Tile {
            sprite_index: si,
            ..Default::default()
        }),
    )
}

//     let mut rng = rand::thread_rng();
//         for y in -DEPTH..-1 {
//             if x < WIDTH - 2 {
//                 let t = match rng.gen_range(0..100) {
//                     r if r < 10 => TileType::Sandstone,
//                     r if r < 20 => TileType::Limestone,
//                     r if r < 30 => TileType::Granite,
//                     r if r < 35 => TileType::Bedrock,
//                     r if r < 40 => TileType::Water,
//                     _ => TileType::Dirt,
//                 };
//                 set_tile(tm, x, y, t);
//             } else {
//                 set_tile(tm, x, y, TileType::Dirt);
//             }
//         }
//     }
