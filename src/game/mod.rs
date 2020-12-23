//  Copyright 2019 Google LLC
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

pub mod mine;
use mine::{Mine, Tile};

pub struct Game {
    mine: Mine,
    player_x: u32,
    player_y: u32,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mine: Mine::new(),
            player_x: 40,
            player_y: 1,
        }
    }
}
