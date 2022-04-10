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
            map.set_tile(player.x, player.y, TileType::Ladder);
        }
    }
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

            tf.translation.x = (64 * player.x) as f32;
            tf.translation.y = (-64 * player.y) as f32;
            // if keyboard_input.just_pressed(KeyCode::A) {
            //     tf.translation.x += MOVE_SPEED * time.delta_seconds();
            // } else if keyboard_input.just_pressed(KeyCode::D) {
            //     tf.translation.x -= MOVE_SPEED * time.delta_seconds();
            // }

            // if keyboard_input.pressed(KeyCode::S) {
            //     tf.translation.y -= MOVE_SPEED * time.delta_seconds();
            // } else if keyboard_input.pressed(KeyCode::W) {
            //     tf.translation.y += MOVE_SPEED * time.delta_seconds();
            // }
        }
    }
}
