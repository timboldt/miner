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

//use std::f64;
// use rand::prelude::*;

// mod game;
// use crate::game::*;

// fn main() {
//     let mut game = Game::new();
//     println!("{}", game);
// }
use bracket_lib::prelude::*;
use std::{thread, time};


struct State {
    y: i32,
    going_down: bool,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        let col1 = RGB::named(CYAN);
        let col2 = RGB::named(YELLOW);
        let percent: f32 = self.y as f32 / 50.0;
        let fg = col1.lerp(col2, percent);

        ctx.cls();
        ctx.printer(
            40,
            49,
            "#[blue]Hello #[pink]Bracket#[] world.",
            TextAlign::Center,
            Some(RGBA::from_u8(200, 200, 200, 255)),
        );

        ctx.print_color(
            1,
            self.y,
            fg,
            RGB::named(BLACK),
            "♫ ♪ Hello Bracket World ☺",
        );

        if self.going_down {
            self.y += 1;
            if self.y > 48 {
                self.going_down = false;
            }
        } else {
            self.y -= 1;
            if self.y < 2 {
                self.going_down = true;
            }
        }

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
        
        thread::sleep(time::Duration::from_millis(16));
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Hello Bracket World")
        .build()?;

    let gs: State = State {
        y: 1,
        going_down: true,
    };

    register_palette_color("blue", RGB::named(BLUE));
    register_palette_color("pink", RGB::named(MAGENTA));

    main_loop(context, gs)
}