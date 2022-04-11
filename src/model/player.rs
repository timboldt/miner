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

#![warn(clippy::all, clippy::pedantic)]

const INITIAL_MONEY: i32 = 1000;

const MAX_ENERGY: i32 = 100;
const INTIAL_ENERGY: i32 = MAX_ENERGY;
const ENERGY_COST: i32 = 10;

pub struct Player {
    pub x: i32,
    pub y: i32,
    pub target_x: i32,
    pub target_y: i32,
    pub rock_hammer: bool,
    money: i32,
    energy: i32,
}

impl Player {
    pub fn new(x: i32, y: i32) -> Self {
        Player {
            x,
            y,
            target_x: x,
            target_y: y,
            rock_hammer: false,
            money: INITIAL_MONEY,
            energy: INTIAL_ENERGY,
        }
    }

    pub fn money(&self) -> i32 {
        self.money
    }

    pub fn receive_money(&mut self, m: i32) {
        self.money += m;
    }

    pub fn pay_money(&mut self, m: i32) {
        self.money -= m;
    }

    pub fn energy(&self) -> i32 {
        self.energy
    }

    pub fn refill_energy(&mut self) {
        // TODO: This could be done smarter.
        while self.energy < MAX_ENERGY && self.money >= ENERGY_COST {
            self.energy += 1;
            self.money -= ENERGY_COST;
        }
    }

    pub fn use_energy(&mut self, e: i32) -> bool {
        let ok = self.energy >= e;
        if ok {
            self.energy -= e;
        }
        ok
    }

    pub fn is_dead(&self) -> bool {
        self.money < 0 || self.energy < 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_works() {
        let p = Player::new(3, 4);
        assert_eq!(3, p.x);
        assert_eq!(4, p.y);
        assert_eq!(1000, p.money());
        assert_eq!(100, p.energy());
        assert_eq!(false, p.is_dead());
    }

    #[test]
    fn money_works() {
        let mut p = Player::new(0, 0);

        let m1 = p.money();
        p.receive_money(42);
        let m2 = p.money();
        assert_eq!(m1 + 42, m2);
        assert_eq!(false, p.is_dead());

        p.pay_money(p.money());
        assert_eq!(0, p.money());
        assert_eq!(false, p.is_dead());

        p.pay_money(1);
        assert_eq!(-1, p.money());
        assert_eq!(true, p.is_dead());
    }

    #[test]
    fn energy_works() {
        let mut p = Player::new(0, 0);
        p.receive_money(100000);

        p.refill_energy();
        assert_eq!(MAX_ENERGY, p.energy());
        assert_eq!(false, p.is_dead());

        assert_eq!(true, p.use_energy(p.energy() / 2));
        assert_eq!(MAX_ENERGY / 2, p.energy());
        assert_eq!(false, p.is_dead());

        p.refill_energy();
        assert_eq!(MAX_ENERGY, p.energy());
        assert_eq!(false, p.is_dead());

        assert_eq!(true, p.use_energy(MAX_ENERGY / 2));
        assert_eq!(MAX_ENERGY / 2, p.energy());
        assert_eq!(false, p.is_dead());

        assert_eq!(true, p.use_energy(MAX_ENERGY / 2));
        assert_eq!(0, p.energy());
        assert_eq!(false, p.is_dead());

        assert_eq!(false, p.use_energy(1));
        assert_eq!(0, p.energy());
        assert_eq!(false, p.is_dead());
    }

    #[test]
    fn energy_costs_money() {
        let mut p = Player::new(0, 0);

        let m1 = p.money();
        assert_eq!(true, p.use_energy(p.energy()));
        assert_eq!(m1, p.money());
        p.refill_energy();
        assert_eq!(m1 - p.energy() * ENERGY_COST, p.money());

        p.pay_money(p.money());
        assert_eq!(true, p.use_energy(p.energy()));
        p.receive_money(2 * ENERGY_COST);
        p.refill_energy();
        assert_eq!(2, p.energy());
        assert_eq!(0, p.money());
        assert_eq!(false, p.is_dead());
    }
}
