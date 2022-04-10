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

use crate::constants::{MAP_HEIGHT, MAP_WIDTH};
use crate::model::map::{Map, TileType};
use crate::model::player::Player;
use crate::{constants::SKY_HEIGHT, model::elevator::Elevator};
use bevy::{
    prelude::*,
    render::camera::{ActiveCameras, Camera},
};

pub fn player_input(
    mut player: ResMut<Player>,
    mut map: ResMut<Map>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Left) {
        if player.x > 1 {
            player.target_x = player.x - 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Right) {
        if player.x < MAP_WIDTH - 3 {
            player.target_x = player.x + 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Up) {
        if player.y > SKY_HEIGHT {
            player.target_y = player.y - 1;
        }
    } else if keyboard_input.just_pressed(KeyCode::Down) {
        if player.y < MAP_HEIGHT - 2 {
            player.target_y = player.y + 1;
        }
    }
    if map.tile(player.x, player.y) == TileType::Dirt {
        map.set_tile(player.x, player.y, TileType::Empty);
    }
}

pub fn elevator_input(
    mut elev: ResMut<Elevator>,
    player: Res<Player>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let depth = elev.depth();
    if keyboard_input.just_pressed(KeyCode::Period) {
        elev.set_target_depth(depth + 1);
    } else if keyboard_input.just_pressed(KeyCode::Comma) && depth > 0 {
        elev.set_target_depth(depth - 1);
    } else if keyboard_input.just_pressed(KeyCode::Space) {
        elev.set_target_depth((player.y - SKY_HEIGHT) as u32);
    }
}

pub fn camera_input(
    active_cameras: Res<ActiveCameras>,
    mut camera_transform_query: Query<(&mut Transform,), With<Camera>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    const MOVE_SPEED: f32 = 1000.0;
    const ZOOM_SPEED: f32 = 10.0;

    if let Some(active_camera_entity) = active_cameras.get("camera_2d").and_then(|ac| ac.entity) {
        if let Ok((mut tf,)) = camera_transform_query.get_mut(active_camera_entity) {
            if keyboard_input.pressed(KeyCode::X) {
                tf.scale -= Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::Z) {
                tf.scale += Vec3::splat(ZOOM_SPEED) * time.delta_seconds();
            }

            if keyboard_input.just_pressed(KeyCode::A) {
                tf.translation.x += MOVE_SPEED * time.delta_seconds();
            } else if keyboard_input.just_pressed(KeyCode::D) {
                tf.translation.x -= MOVE_SPEED * time.delta_seconds();
            }

            if keyboard_input.pressed(KeyCode::S) {
                tf.translation.y -= MOVE_SPEED * time.delta_seconds();
            } else if keyboard_input.pressed(KeyCode::W) {
                tf.translation.y += MOVE_SPEED * time.delta_seconds();
            }
        }
    }
}
