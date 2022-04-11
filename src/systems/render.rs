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

#![warn(clippy::all)]

use crate::constants::*;
use crate::model::elevator::Elevator;
use crate::model::map::{Map, TileType};
use crate::model::player::Player;
use bevy::{math::ivec3, prelude::*};
use bevy_simple_tilemap::prelude::*;

pub fn setup(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Load tilesheet texture and make a texture atlas from it
    let texture_handle = asset_server.load("64x64_tileset.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 10, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let cam = OrthographicCameraBundle::new_2d();
    commands.spawn_bundle(cam);

    let tilemap = TileMap::default();
    let tilemap_bundle = TileMapBundle {
        tilemap,
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            scale: Vec3::splat(1.0),
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
        ..Default::default()
    };
    commands.spawn_bundle(tilemap_bundle);

    let map = Map::new(MAP_WIDTH as usize, MAP_HEIGHT as usize);
    commands.insert_resource(map);
}

pub fn show_player(player: Res<Player>, mut query: Query<&mut TileMap>) {
    for mut tm in query.iter_mut() {
        tm.clear_layer(PLAYER_LAYER);

        tm.set_tile(
            ivec3(player.x, -player.y, PLAYER_LAYER),
            Some(Tile {
                sprite_index: SpriteIndex::Person as u32,
                ..Default::default()
            }),
        );
    }
}

pub fn show_elevator(elev: Res<Elevator>, mut query: Query<&mut TileMap>) {
    for mut tm in query.iter_mut() {
        tm.clear_layer(ELEVATOR_LAYER);

        // Elevator body.
        tm.set_tile(
            ivec3(
                ELEVATOR_SHAFT_X,
                1 - GRASS_LEVEL - elev.depth() as i32,
                ELEVATOR_LAYER,
            ),
            Some(Tile {
                sprite_index: SpriteIndex::Elevator as u32,
                ..Default::default()
            }),
        );
        for i in 0..elev.depth() {
            tm.set_tile(
                ivec3(ELEVATOR_SHAFT_X, 1 - GRASS_LEVEL - i as i32, ELEVATOR_LAYER),
                Some(Tile {
                    sprite_index: SpriteIndex::ElevatorCable as u32,
                    ..Default::default()
                }),
            );
        }
        tm.set_tile(
            ivec3(ELEVATOR_SHAFT_X, 2 - GRASS_LEVEL, ELEVATOR_LAYER),
            Some(Tile {
                sprite_index: SpriteIndex::ElevatorHook as u32,
                ..Default::default()
            }),
        );
        tm.set_tile(
            ivec3(ELEVATOR_SHAFT_X + 1, 2 - GRASS_LEVEL, ELEVATOR_LAYER),
            Some(Tile {
                sprite_index: SpriteIndex::ElevatorTowerTop as u32,
                ..Default::default()
            }),
        );
        tm.set_tile(
            ivec3(ELEVATOR_SHAFT_X + 1, 1 - GRASS_LEVEL, ELEVATOR_LAYER),
            Some(Tile {
                sprite_index: SpriteIndex::ElevatorTowerBottom as u32,
                ..Default::default()
            }),
        );
    }
}

pub fn update_tilemap(player: Res<Player>, map: Res<Map>, mut query: Query<&mut TileMap>) {
    for mut tm in query.iter_mut() {
        if tm.chunks.is_empty() {
            for x in 0..MAP_WIDTH {
                for y in 0..MAP_HEIGHT {
                    set_tile(&mut tm, x, -y, map.tile(x, y));
                }
            }
        } else {
            for x in player.x - 10..=player.x + 10 {
                for y in player.y - 10..=player.y + 10 {
                    set_tile(&mut tm, x, -y, map.tile(x, y));
                }
            }
        }

        // For now, just draw the bank over top.
        // TODO: Make the bank a real entity.
        tm.set_tile(
            ivec3(MAP_WIDTH - 10, 1 - SKY_HEIGHT, 0),
            Some(Tile {
                sprite_index: SpriteIndex::BankTopLeft as u32,
                ..Default::default()
            }),
        );
        tm.set_tile(
            ivec3(MAP_WIDTH - 9, 1 - SKY_HEIGHT, 0),
            Some(Tile {
                sprite_index: SpriteIndex::BankTopRight as u32,
                ..Default::default()
            }),
        );
        tm.set_tile(
            ivec3(MAP_WIDTH - 10, -SKY_HEIGHT, 0),
            Some(Tile {
                sprite_index: SpriteIndex::BankBottomLeft as u32,
                ..Default::default()
            }),
        );
        tm.set_tile(
            ivec3(MAP_WIDTH - 9, -SKY_HEIGHT, 0),
            Some(Tile {
                sprite_index: SpriteIndex::BankBottomRight as u32,
                ..Default::default()
            }),
        );
    }
}

fn set_tile(tm: &mut TileMap, x: i32, y: i32, t: TileType) {
    if t == TileType::Void {
        return;
    }
    let si = match t {
        TileType::Empty => SpriteIndex::Empty as u32,
        TileType::Sky => SpriteIndex::Sky as u32,
        TileType::Grass => SpriteIndex::Grass as u32,
        TileType::Dirt => SpriteIndex::Dirt as u32,
        TileType::Ladder => SpriteIndex::Ladder as u32,
        TileType::Rock { hardness } => {
            (SpriteIndex::Stone0 as u8 + core::cmp::min(hardness, 3)) as u32
        }
        TileType::Treasure { value } => {
            (SpriteIndex::Treasure0 as u8 + core::cmp::min(value, 2)) as u32
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
