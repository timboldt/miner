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
use bevy::{
    prelude::*,
    render::camera::{ActiveCameras, Camera},
};

pub fn player_input(
    mut player: ResMut<Player>,
    mut map: ResMut<Map>,
    mut elev: ResMut<Elevator>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let depth = elev.depth();
    let player_in_elevator = player.x == ELEVATOR_SHAFT_X && (player.y - SKY_HEIGHT) == depth;

    if keyboard_input.just_pressed(KeyCode::Left) {
        if player.x > 1 {
            player.target_x = player.x - 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        if player.x < MAP_WIDTH - 3 {
            player.target_x = player.x + 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        if player_in_elevator {
            elev.set_target_depth(depth - 1);
        } else if player.y > SKY_HEIGHT {
            player.target_y = player.y - 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        if player_in_elevator {
            elev.set_target_depth(depth + 1);
        } else if player.y < MAP_HEIGHT - 2 {
            player.target_y = player.y + 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::L) {
        if player.x < ELEVATOR_SHAFT_X && map.tile(player.x, player.y) == TileType::Empty {
            if player.use_energy(5) {
                map.set_tile(player.x, player.y, TileType::Ladder);
            }
        }
    } else if keyboard_input.just_pressed(KeyCode::R) {
        // Rescue!
        if player.x < ELEVATOR_SHAFT_X && player.y > GRASS_LEVEL {
            player.pay_money(250);
            player.x = ELEVATOR_SHAFT_X;
            player.y = SKY_HEIGHT + depth;
            player.target_x = player.x;
            player.target_y = player.y;
            elev.set_target_depth(0);
        }
    }

    player.rock_hammer =
        keyboard_input.pressed(KeyCode::LShift) || keyboard_input.pressed(KeyCode::RShift);
}

pub fn elevator_input(
    mut elev: ResMut<Elevator>,
    player: Res<Player>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        elev.set_target_depth(player.y - SKY_HEIGHT);
    } else if keyboard_input.just_pressed(KeyCode::H) {
        elev.set_target_depth(-SKY_HEIGHT);
    } else if keyboard_input.just_pressed(KeyCode::B) {
        elev.set_target_depth(MAP_HEIGHT - SKY_HEIGHT);
    }
}

pub fn camera_input(
    active_cameras: Res<ActiveCameras>,
    mut camera_transform_query: Query<(&mut Transform,), With<Camera>>,
    player: Res<Player>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const ZOOM_SPEED: f32 = 10.0;

    if let Some(active_camera_entity) = active_cameras.get("camera_2d").and_then(|ac| ac.entity) {
        if let Ok((mut tf,)) = camera_transform_query.get_mut(active_camera_entity) {
            if keyboard_input.pressed(KeyCode::X) {
                tf.scale -= Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::Z) {
                tf.scale += Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
            }

            tf.scale = tf.scale.max(Vec3::ONE);
            tf.translation.x = (64 * player.x) as f32;
            tf.translation.y = (-64 * player.y) as f32;
        }
    }
}
