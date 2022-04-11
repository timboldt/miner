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
use rand::prelude::*;

pub fn move_player(mut player: ResMut<Player>, mut map: ResMut<Map>, elev: Res<Elevator>) {
    let depth = elev.depth();
    let player_in_elevator = player.x == ELEVATOR_SHAFT_X && (player.y - SKY_HEIGHT) == depth;

    if player_in_elevator {
        // Let the elevator move us instead.
        player.target_y = player.y;
    } else if map.tile(player.x, player.y + 1) == TileType::Empty
        && map.tile(player.x, player.y) != TileType::Ladder
    {
        // Fall down.
        player.target_x = player.x;
        player.target_y = player.y + 1;
    }

    // Change the target tile, if needed.
    match map.tile(player.target_x, player.target_y) {
        TileType::Dirt => {
            if player.use_energy(1) {
                match thread_rng().gen_range(0..50) {
                    0..=9 => {
                        // Rock.
                        map.set_tile(
                            player.target_x,
                            player.target_y,
                            TileType::Rock {
                                hardness: ((thread_rng().gen_range(0..50) + player.target_y) / 25)
                                    as u8,
                            },
                        );
                    }
                    10..=12 => {
                        // Treasure.
                        map.set_tile(
                            player.target_x,
                            player.target_y,
                            TileType::Treasure {
                                value: ((thread_rng().gen_range(0..50) + player.target_y) / 25)
                                    as u8,
                            },
                        );
                    }
                    20 => {
                        // Flooding.
                        for x in (player.target_x - 5)..=(player.target_x + 5) {
                            for y in (player.target_y - 5)..=(player.target_y + 5) {
                                if thread_rng().gen_range(0..100) < 10
                                    && (map.tile(x, y) == TileType::Empty
                                        || map.tile(x, y) == TileType::Dirt)
                                    && !(x == player.x && y == player.y)
                                    && x < ELEVATOR_SHAFT_X
                                {
                                    map.set_tile(x, y, TileType::Water);
                                }
                            }
                        }
                    }
                    21 => {
                        // Cave-in.
                        for x in (player.target_x - 5)..=(player.target_x + 5) {
                            for y in (player.target_y - 5)..=(player.target_y + 5) {
                                if thread_rng().gen_range(0..100) < 50
                                    && (map.tile(x, y) == TileType::Empty
                                        || map.tile(x, y) == TileType::Ladder)
                                    && !(x == player.x && y == player.y)
                                    && x < ELEVATOR_SHAFT_X
                                {
                                    map.set_tile(x, y, TileType::Dirt);
                                }
                            }
                        }
                    }
                    _ => map.set_tile(player.target_x, player.target_y, TileType::Empty),
                }
            }
        }
        TileType::Rock { .. } => {
            // Don't allow chiselling rock yet.
        }
        TileType::Treasure { value } => {
            // Collect the treasure.
            player.receive_money((1 << value) * TREASURE_BASE_VALUE);
            map.set_tile(player.target_x, player.target_y, TileType::Empty);
            player.x = player.target_x;
            player.y = player.target_y;
        }
        _ => {}
    }

    // You can't climb up in thin air.
    if !player_in_elevator
        && map.tile(player.x, player.y) == TileType::Empty
        && player.target_y < player.y
    {
        player.target_y = player.y;
    }

    // Move towards target, if possible.
    match map.tile(player.target_x, player.target_y) {
        TileType::Empty | TileType::Ladder | TileType::Sky => {
            // Allow the move.
            player.x = player.target_x;
            player.y = player.target_y;
        }
        _ => {
            // If we haven't moved there yet, we aren't going to.
            player.target_x = player.x;
            player.target_y = player.y;
        }
    }

    // Cash in and recharge.
    if player.x == MAP_WIDTH - 9 && player.y == SKY_HEIGHT {
        player.refill_energy();
    }
}
