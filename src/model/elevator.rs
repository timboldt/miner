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

pub struct Elevator {
    // The depth is the current location. Zero is ground level.
    depth: i32,
    max_depth: i32,
    // The target depth is the called location.
    target_depth: i32,
}

impl Elevator {
    pub fn new(max_depth: i32) -> Self {
        assert!(max_depth >= 0);
        Elevator {
            depth: 0,
            max_depth,
            target_depth: 0,
        }
    }

    pub fn depth(&self) -> i32 {
        self.depth
    }

    pub fn set_target_depth(&mut self, target: i32) {
        self.target_depth = core::cmp::max(0, core::cmp::min(target, self.max_depth));
    }

    pub fn move_towards_target(&mut self) -> i32 {
        if self.target_depth < self.depth {
            self.depth -= 1;
        } else if self.target_depth > self.depth {
            self.depth += 1;
        }
        self.depth
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let elev = Elevator::new(100);
        assert_eq!(0, elev.depth());
    }

    #[test]
    fn will_move() {
        let mut elev = Elevator::new(10);

        elev.set_target_depth(2);
        assert_eq!(0, elev.depth());
        assert_eq!(1, elev.move_towards_target());
        assert_eq!(2, elev.move_towards_target());
        assert_eq!(2, elev.move_towards_target());
        assert_eq!(2, elev.depth());

        elev.set_target_depth(0);
        assert_eq!(2, elev.depth());
        assert_eq!(1, elev.move_towards_target());
        assert_eq!(0, elev.move_towards_target());
        assert_eq!(0, elev.move_towards_target());
        assert_eq!(0, elev.depth());

        elev.set_target_depth(100);
        assert_eq!(0, elev.depth());
        // It moves towards the target depth.
        for i in 1..=10 {
            assert_eq!(i, elev.move_towards_target());
        }
        // But it doesn't move beyond the max depth.
        assert_eq!(10, elev.move_towards_target());
        assert_eq!(10, elev.move_towards_target());
        assert_eq!(10, elev.depth());
    }

    #[test]
    fn null_range() {
        let mut elev = Elevator::new(0);
        elev.set_target_depth(2);
        assert_eq!(0, elev.depth());
        assert_eq!(0, elev.move_towards_target());
        assert_eq!(0, elev.depth());
    }
}
