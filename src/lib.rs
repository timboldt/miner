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

//use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod game;
use crate::game::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen(start)]
pub fn start() {
    let window = web_sys::window().expect("should have a window in this context");
    let performance = window
        .performance()
        .expect("performance should be available");
    console_log!("start time (in ms) is {}", performance.now());

    let mut game = Game::new();
    game.move_player(Direction::Right);

    let document = web_sys::window().unwrap().document().unwrap();
    let txtout :web_sys::HtmlElement= document.get_element_by_id("txtout").unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .map_err(|_| ())
        .unwrap();
    txtout.set_inner_text(&format!("{}", game));

    // let canvas = document.get_element_by_id("canvas").unwrap();
    // let canvas: web_sys::HtmlCanvasElement = canvas
    //     .dyn_into::<web_sys::HtmlCanvasElement>()
    //     .map_err(|_| ())
    //     .unwrap();

    // let context = canvas
    //     .get_context("2d")
    //     .unwrap()
    //     .unwrap()
    //     .dyn_into::<web_sys::CanvasRenderingContext2d>()
    //     .unwrap();

    // context.begin_path();

    // // Draw the outer circle.
    // context
    //     .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // // Draw the mouth.
    // context.move_to(100.0, 100.0);
    // context
    //     .arc(
    //         75.0,
    //         75.0,
    //         35.0,
    //         f64::consts::PI * 0.25,
    //         f64::consts::PI * 0.75,
    //     )
    //     .unwrap();

    // // Draw the left eye.
    // context.move_to(65.0, 65.0);
    // context
    //     .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // // Draw the right eye.
    // context.move_to(95.0, 65.0);
    // context
    //     .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
    //     .unwrap();

    // context.stroke();

    //console::log_1(&format!("{:?}", duration).into());
    console_log!("end time (in ms) is {}", performance.now());
}
