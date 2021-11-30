//  Copyright 2021 Google LLC
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
pub enum MineTile {
    Empty,          // Empty space.
    UnexploredDirt, // Looks like dirt until examined.
    KnownDirt,      // Just plain dirt - diggable.
    Grass,          // Impenetrable surface material.
    Ladder,
    Invalid, // Beyond the edge of the universe.
}

pub struct Mine {
    height: u32,
    width: u32,
    tiles: Vec<MineTile>,
}

impl Mine {
    pub fn new() -> Mine {
        let mut m = Mine {
            height: 50,
            width: 50,
            tiles: vec![MineTile::UnexploredDirt; 50 * 50],
        };
        for x in 0..m.width {
            // Two rows of sky.
            m.set_tile(x, 0, MineTile::Empty);
            m.set_tile(x, 1, MineTile::Empty);
            // One row of impenetrable "grass".
            m.set_tile(x, 2, MineTile::Grass);
        }
        for y in 0..m.height {
            // Elevator shaft.
            m.set_tile(m.width - 2, y, MineTile::Empty);
        }
        m
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn get_tile(&self, x: u32, y: u32) -> MineTile {
        if x >= self.width || y >= self.height {
            return MineTile::Invalid;
        }
        let idx = (y * self.width + x) as usize;
        self.tiles[idx]
    }

    pub fn set_tile(&mut self, x: u32, y: u32, tile: MineTile) {
        if x >= self.width || y >= self.height {
            panic!("invalid tile address");
        }
        let idx = (y * self.width + x) as usize;
        self.tiles[idx] = tile;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_tile() {
        let mine = Mine::new();
        assert_eq!(MineTile::UnexploredDirt, mine.get_tile(20, 20));
    }

    #[test]
    fn invalid_tile() {
        let mine = Mine::new();
        assert_eq!(MineTile::Invalid, mine.get_tile(200, 20));
        assert_eq!(MineTile::Invalid, mine.get_tile(20, 200));
    }
}
