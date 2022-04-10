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

use crate::constants::*;
use crate::model::elevator::Elevator;
use crate::model::map::{Map, TileType};
use crate::model::player::Player;
use bevy::{math::ivec3, prelude::*};
use bevy_simple_tilemap::prelude::*;

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
            for x in player.x - 1..=player.x + 1 {
                for y in player.y - 1..=player.y + 1 {
                    set_tile(&mut tm, x, -y, map.tile(x, y));
                }
            }
        }
    }
}

fn set_tile(tm: &mut TileMap, x: i32, y: i32, t: TileType) {
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
