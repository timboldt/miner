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

mod mine;
use mine::{Mine, Tile};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Point {
    x: u32,
    y: u32,
}

pub struct Game {
    mine: Mine,
    player_pos: Point,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mine: Mine::new(),
            player_pos: Point { x: 40, y: 1 },
        }
    }

    pub fn player_tile(&self) -> Tile {
        self.get_tile(self.player_pos)
    }

    pub fn move_player(&mut self, command: Direction) {
        let below = Point {
            x: self.player_pos.x,
            y: self.player_pos.y + 1,
        };
        if self.player_tile() == Tile::Empty && self.get_tile(below) == Tile::Empty {
            // Fall.
            self.set_player_pos(below);
            return;
        }

        if command == Direction::Up && self.player_tile() != Tile::Ladder {
            // Can't climb without a ladder.
            return;
        }
        let target_pos = match command {
            Direction::Left => Point {
                x: self.player_pos.x - 1,
                y: self.player_pos.y,
            },
            Direction::Right => Point {
                x: self.player_pos.x + 1,
                y: self.player_pos.y,
            },
            Direction::Up => Point {
                x: self.player_pos.x,
                y: self.player_pos.y - 1,
            },
            Direction::Down => Point {
                x: self.player_pos.x,
                y: self.player_pos.y + 1,
            },
        };
        if self.get_tile(target_pos) == Tile::Empty {
            self.set_player_pos(target_pos);
        }
    }

    pub fn get_tile(&self, pt: Point) -> Tile {
        self.mine.get_tile(pt.x, pt.y)
    }

    fn set_player_pos(&mut self, pt: Point) {
        self.player_pos = pt;
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut out = String::new();
        for y in 0..self.mine.height() {
            for x in 0..self.mine.width() {
                if self.player_pos.x == x && self.player_pos.y == y {
                    out += "*";
                } else {
                    out += match self.get_tile(Point { x, y }) {
                        Tile::Empty => "\u{2591}",
                        Tile::KnownDirt | Tile::UnexploredDirt => "\u{2592}",
                        Tile::Grass => "\u{2593}",
                        _ => "?",
                    };
                }
            }
            out += "\n";
        }
        write!(f, "{}", out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walk_left_right() {
        let mut game = Game::new();
        game.mine.set_tile(19, 20, Tile::Empty);
        game.mine.set_tile(20, 20, Tile::Empty);
        game.set_player_pos(Point { x: 20, y: 20 });
        assert_eq!(Point { x: 20, y: 20 }, game.player_pos);
        game.move_player(Direction::Left);
        assert_eq!(Point { x: 19, y: 20 }, game.player_pos);
        game.move_player(Direction::Right);
        assert_eq!(Point { x: 20, y: 20 }, game.player_pos);
    }

    #[test]
    fn climb_ladder() {
        let mut game = Game::new();
        game.mine.set_tile(20, 19, Tile::Empty);
        game.mine.set_tile(20, 20, Tile::Empty);
        game.set_player_pos(Point { x: 20, y: 20 });
        assert_eq!(Point { x: 20, y: 20 }, game.player_pos);

        // Can't climb without a ladder.
        game.move_player(Direction::Up);
        assert_eq!(Point { x: 20, y: 20 }, game.player_pos);

        // Climbing works with a ladder.
        game.mine.set_tile(20, 20, Tile::Ladder);
        game.move_player(Direction::Up);
        assert_eq!(Point { x: 20, y: 19 }, game.player_pos);
    }
}
