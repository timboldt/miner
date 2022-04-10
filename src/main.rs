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

use bevy::text::text2d_system;
use bevy::{core::FixedTimestep, math::ivec3, prelude::*};

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
        .insert_resource(Elevator::new(MAX_ELEVATOR_DEPTH))
        .insert_resource(Player::new(PLAYER_START_X, PLAYER_START_Y))
        .add_plugins(DefaultPlugins)
        .add_plugin(SimpleTileMapPlugin)
        .add_startup_system(systems::render::setup)
        .add_startup_system(systems::text::setup)
        .add_system(systems::input::camera_input)
        .add_system(systems::input::elevator_input)
        .add_system(systems::input::player_input)
        .add_system(systems::player::move_player)
        .add_system(systems::elevator::move_elevator.with_run_criteria(FixedTimestep::step(0.1)))
        .add_system(systems::render::update_tilemap)
        .add_system(systems::render::show_player)
        .add_system(systems::render::show_elevator)
        .add_system(systems::text::update_money)
        .add_system(systems::text::update_energy)
        .run();
}
