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

pub const MAP_WIDTH: i32 = 40;
pub const MAP_HEIGHT: i32 = 60;
pub const PLAYER_START_X: i32 = MAP_WIDTH - 5;
pub const PLAYER_START_Y: i32 = SKY_HEIGHT;
pub const MAX_ELEVATOR_DEPTH: i32 = MAP_HEIGHT - SKY_HEIGHT - 2;
pub const ELEVATOR_SHAFT_X: i32 = MAP_WIDTH - 3;
pub const SKY_HEIGHT: i32 = 3;
pub const GRASS_LEVEL: i32 = SKY_HEIGHT + 1;

pub const PLAYER_LAYER: i32 = 1;
pub const ELEVATOR_LAYER: i32 = 2;

pub const TREASURE_BASE_VALUE: i32 = 200;

pub enum SpriteIndex {
    Empty = 0,
    Dirt,

    Stone0 = 2,

    Grass = 6,
    Water,
    Border,
    Sky,

    Person = 10,
    Ladder,

    Treasure0 = 17,

    BankTopLeft = 23,
    BankTopRight,
    BankBottomLeft = 33,
    BankBottomRight,

    Elevator = 30,
    ElevatorHook = 20,
    ElevatorTowerTop = 21,
    ElevatorTowerBottom = 31,
    ElevatorCable = 32,
}
