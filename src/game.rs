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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Dirt,
    Invalid,
}

pub struct Mine {
    height: u32,
    width: u32,
    tiles: Vec<Tile>,
}

impl Mine {
    pub fn new() -> Mine {
        Mine {
            height: 50,
            width: 50,
            tiles: vec![Tile::Dirt; 50 * 50],
        }
    }

    pub fn get_tile(&self, row: u32, col: u32) -> Tile {
        if row >= self.height || col >= self.width {
            return Tile::Invalid;
        }
        let idx = (row * self.width + col) as usize;
        self.tiles[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_tile() {
        let mine = Mine::new();
        assert_eq!(Tile::Dirt, mine.get_tile(20, 20));
    }

    #[test]
    fn invalid_tile() {
        let mine = Mine::new();
        assert_eq!(Tile::Invalid, mine.get_tile(200, 20));
        assert_eq!(Tile::Invalid, mine.get_tile(20, 200));
    }

}