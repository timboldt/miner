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

#[wasm_bindgen]
pub struct WebGame {
    game: Game,
}

#[wasm_bindgen]
impl WebGame {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<WebGame, JsValue> {
        Ok(WebGame { game: Game::new() })
    }

    #[wasm_bindgen]
    pub fn get_text_repr(&self) -> JsValue {
        JsValue::from_str(&format!("{}", self.game))
    }

    #[wasm_bindgen]
    pub fn move_player(&mut self, dir: &str) {
        self.game.move_player(Direction::Right);
    }
}
