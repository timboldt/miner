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
use bevy::prelude::*;

pub fn move_player(mut player: ResMut<Player>, mut map: ResMut<Map>, elev: Res<Elevator>) {
    let depth = elev.depth();
    let player_in_elevator =
        player.x == ELEVATOR_SHAFT_X && (player.y - SKY_HEIGHT) as u32 == depth;

    if player_in_elevator {
        player.target_y = player.y;
    }

    player.x = player.target_x;
    player.y = player.target_y;

    if map.tile(player.x, player.y) == TileType::Dirt {
        map.set_tile(player.x, player.y, TileType::Empty);
    }
}
