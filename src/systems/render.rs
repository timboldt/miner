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
    const ELEVATOR_SHAFT_X: i32 = MAP_WIDTH - 3;

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