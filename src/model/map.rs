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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileType {
    Border,
    Dirt,
    Empty,
    Grass,
    Ladder,
    Rock { hardness: u8 },
    Sky,
    Treasure { value: u8 },
    Void,
    Water,
}

pub struct Map {
    height: usize,
    width: usize,
    tiles: Vec<TileType>,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let mut map = Map {
            height,
            width,
            tiles: vec![TileType::Dirt; height * width],
        };
        map.init_default_tiles();
        map
    }

    pub fn tile(&self, x: i32, y: i32) -> TileType {
        if x < 0 || x as usize >= self.width {
            return TileType::Void;
        }
        if y < 0 || y as usize >= self.height {
            return TileType::Void;
        }
        self.tiles[self.tile_idx(x, y)]
    }

    pub fn set_tile(&mut self, x: i32, y: i32, t: TileType) {
        if x < 0 || x as usize >= self.width {
            panic!("Invalid tile location.");
        }
        if y < 0 || y as usize >= self.height {
            panic!("Invalid tile location.");
        }
        let idx = self.tile_idx(x, y);
        self.tiles[idx] = t;
    }

    fn tile_idx(&self, x: i32, y: i32) -> usize {
        x as usize + y as usize * self.width
    }

    fn init_default_tiles(&mut self) {
        let h = self.height as i32;
        let w = self.width as i32;

        // Borders.
        for x in 0..w {
            // Top border.
            self.set_tile(x, 0, TileType::Border);
            // Bottom border.
            self.set_tile(x, h - 1, TileType::Border);
        }
        for y in 0..h {
            // Left border.
            self.set_tile(0, y, TileType::Border);
            // Right border.
            self.set_tile(w - 1, y, TileType::Border);
        }

        // Sky.
        for x in 1..w - 1 {
            for y in 1..=SKY_HEIGHT {
                self.set_tile(x, y, TileType::Sky);
            }
        }

        // Grass.
        for x in 1..w - 1 {
            self.set_tile(x, GRASS_LEVEL, TileType::Grass);
        }
        self.set_tile(3, GRASS_LEVEL, TileType::Ladder);
        self.set_tile(3, GRASS_LEVEL + 1, TileType::Ladder);
        self.set_tile(3, GRASS_LEVEL + 2, TileType::Ladder);
        self.set_tile(MAP_WIDTH / 2, GRASS_LEVEL, TileType::Ladder);
        self.set_tile(MAP_WIDTH / 2, GRASS_LEVEL + 1, TileType::Ladder);
        self.set_tile(MAP_WIDTH / 2, GRASS_LEVEL + 2, TileType::Ladder);

        // Mine shaft.
        for y in GRASS_LEVEL..h - 1 {
            // width-1 is the border.
            // width-2 is a column of dirt.
            // width-3 is the actual elevator shaft.
            self.set_tile(w - 3, y, TileType::Empty);
        }

        // A little scenery along the elevator shaft.
        for i in 0..=3 {
            self.set_tile(w - 4, 10 * i + 10, TileType::Rock { hardness: i as u8 });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_works() {
        let m = Map::new(30, 50);
        assert_eq!(TileType::Void, m.tile(-1, -1));
        assert_eq!(TileType::Border, m.tile(0, 0));
        assert_eq!(TileType::Sky, m.tile(1, 1));
        assert_eq!(TileType::Grass, m.tile(1, GRASS_LEVEL));
        assert_eq!(TileType::Dirt, m.tile(1, GRASS_LEVEL + 1));
        assert_eq!(TileType::Border, m.tile(29, 19));
        assert_eq!(TileType::Void, m.tile(30, 20));
    }

    #[test]
    fn set_tile_works() {
        let mut m = Map::new(30, 50);
        assert_eq!(TileType::Dirt, m.tile(2, 10));
        m.set_tile(2, 10, TileType::Rock { hardness: 2 });
        assert_eq!(TileType::Rock { hardness: 2 }, m.tile(2, 10));
    }
}
