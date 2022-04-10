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
use bevy::prelude::*;

pub fn move_elevator(mut elev: ResMut<Elevator>, mut player: ResMut<Player>) {
    let depth = elev.depth();
    let player_in_elevator = player.x == ELEVATOR_SHAFT_X && (player.y - SKY_HEIGHT) == depth;
    elev.move_towards_target();
    if player_in_elevator {
        player.y += elev.depth() as i32 - depth as i32;
    }
}
