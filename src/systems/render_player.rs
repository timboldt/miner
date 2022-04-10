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
use crate::model::player::Player;
use bevy::{math::ivec3, prelude::*};
use bevy_simple_tilemap::prelude::*;

pub fn show_player(player: Res<Player>, mut query: Query<&mut TileMap>) {
    for mut tm in query.iter_mut() {
        tm.clear_layer(PLAYER_LAYER);

        tm.set_tile(
            ivec3(player.x, -player.y, 1),
            Some(Tile {
                sprite_index: SpriteIndex::Person as u32,
                ..Default::default()
            }),
        );
    }
}
