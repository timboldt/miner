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

use bracket_lib::prelude::*;
use std::{thread, time};

mod mine;
use mine::{Mine, MineTile};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct GamePoint {
    x: u32,
    y: u32,
}

pub struct Game {
    mine: Mine,
    player_pos: GamePoint,
}

impl Game {
    pub fn new() -> Game {
        Game {
            mine: Mine::new(),
            player_pos: GamePoint { x: 40, y: 1 },
        }
    }

    pub fn player_tile(&self) -> MineTile {
        self.get_tile(self.player_pos)
    }

    pub fn move_player(&mut self, command: Direction) {
        let below = GamePoint {
            x: self.player_pos.x,
            y: self.player_pos.y + 1,
        };
        if self.player_tile() == MineTile::Empty && self.get_tile(below) == MineTile::Empty {
            // Fall.
            self.set_player_pos(below);
            return;
        }

        if command == Direction::Up && self.player_tile() != MineTile::Ladder {
            // Can't climb without a ladder.
            return;
        }
        let target_pos = match command {
            Direction::Left => GamePoint {
                x: self.player_pos.x - 1,
                y: self.player_pos.y,
            },
            Direction::Right => GamePoint {
                x: self.player_pos.x + 1,
                y: self.player_pos.y,
            },
            Direction::Up => GamePoint {
                x: self.player_pos.x,
                y: self.player_pos.y - 1,
            },
            Direction::Down => GamePoint {
                x: self.player_pos.x,
                y: self.player_pos.y + 1,
            },
        };
        if self.get_tile(target_pos) == MineTile::Empty {
            self.set_player_pos(target_pos);
        }
    }

    pub fn get_tile(&self, pt: GamePoint) -> MineTile {
        self.mine.get_tile(pt.x, pt.y)
    }

    fn set_player_pos(&mut self, pt: GamePoint) {
        self.player_pos = pt;
    }
}

impl GameState for Game {
    fn tick(&mut self, ctx: &mut BTerm) {
        let col1 = RGB::named(CYAN);
        let col2 = RGB::named(YELLOW);

        ctx.cls();
        ctx.printer(
            40,
            49,
            "#[blue]Hello #[pink]Bracket#[] world.",
            TextAlign::Center,
            Some(RGBA::from_u8(200, 200, 200, 255)),
        );

        ctx.draw_box(39, 0, 20, 3, RGB::named(WHITE), RGB::named(BLACK));
        ctx.printer(
            58,
            1,
            &format!("#[pink]FPS: #[]{}", ctx.fps),
            TextAlign::Right,
            None,
        );
        ctx.printer(
            58,
            2,
            &format!("#[pink]Frame Time: #[]{} ms", ctx.frame_time_ms),
            TextAlign::Right,
            None,
        );

        thread::sleep(time::Duration::from_millis(5));
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
                    out += match self.get_tile(GamePoint { x, y }) {
                        MineTile::Empty => "\u{2591}",
                        MineTile::KnownDirt | MineTile::UnexploredDirt => "\u{2592}",
                        MineTile::Grass => "\u{2593}",
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
        game.mine.set_tile(19, 20, MineTile::Empty);
        game.mine.set_tile(20, 20, MineTile::Empty);
        game.set_player_pos(GamePoint { x: 20, y: 20 });
        assert_eq!(GamePoint { x: 20, y: 20 }, game.player_pos);
        game.move_player(Direction::Left);
        assert_eq!(GamePoint { x: 19, y: 20 }, game.player_pos);
        game.move_player(Direction::Right);
        assert_eq!(GamePoint { x: 20, y: 20 }, game.player_pos);
    }

    #[test]
    fn climb_ladder() {
        let mut game = Game::new();
        game.mine.set_tile(20, 19, MineTile::Empty);
        game.mine.set_tile(20, 20, MineTile::Empty);
        game.set_player_pos(GamePoint { x: 20, y: 20 });
        assert_eq!(GamePoint { x: 20, y: 20 }, game.player_pos);

        // Can't climb without a ladder.
        game.move_player(Direction::Up);
        assert_eq!(GamePoint { x: 20, y: 20 }, game.player_pos);

        // Climbing works with a ladder.
        game.mine.set_tile(20, 20, MineTile::Ladder);
        game.move_player(Direction::Up);
        assert_eq!(GamePoint { x: 20, y: 19 }, game.player_pos);
    }
}
