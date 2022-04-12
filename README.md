# Miner

## Disclaimer

This is not an officially supported Google product.

## Introduction

This project is a port of an Apple ][ game I wrote in the 1980's. The [original](assets/original_miner.txt) was written in UCSD Pascal.

The original Miner game was the first time I had ever done "structured programming" (as they use to call it), and I had no formal training yet, so it isn't pretty (although it is much more readable than my AppleSoft BASIC code from the same era). It was inspired by a Commodore PET ASCII-based game of the same name.

## Usage Instructions

Collect precious metals and gems to get money. Exchange money for more energy at the bank. (The bank should probably be a saloon, but the 6yo play-tester wanted a bank.)

Player controls:

* Arrow keys - move player and dig dirt.
* L - build a ladder.
* Shift - In combination with arrow keys, allows you to remove rock.
* R - request a rescue (for a price).

Elevator controls:

* Space bar - summon the elevator to your level.
* H - send the elevator home (to the top).
* B - send the elevator to the bottom.

Zoom controls:

* Z - zoom out.
* X - zoom in (be careful, the screen will disappear if you zoom in too much).

![Screenshot](assets/screenshot1.png?raw=true "Screen Shot")

## Credits

I originally started with Herbert Wolverson's Rogue Toolkit [Bracket-Lib](https://github.com/amethyst/bracket-lib), which makes 2D game development really simple. I later refactored it to use [Bevy](https://bevyengine.org/) instead.

I also based a lot of the code on what I learned in his book [Hands-on Rust](https://pragprog.com/titles/hwrust/hands-on-rust/), which is a fantastic (and fun!) learning resource.
