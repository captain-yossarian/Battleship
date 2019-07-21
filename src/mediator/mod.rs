use crate::player;
use crate::structures;
use crate::utils;
use std::io;
use std::io::Read;
use utils::random_number;

use player::Player;
use structures::Point;

pub struct Mediator {
    pub human: Player,
    pub ai: Player,
}

fn get_number() -> u8 {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    let mut number = 0;
    match trimmed.parse::<u8>() {
        Ok(i) => number = i,
        Err(..) => (),
    };
    number
}
fn get_point() -> Point {
    let mut input = String::new();
    let mut row = get_number();
    let mut column = get_number();

    let point = Point { row, column };
    println!("User point {:?}", point);
    point
}
impl Mediator {
    pub fn new() -> Mediator {
        Mediator {
            human: Player::new(random_number),
            ai: Player::new(random_number),
        }
    }
    pub fn human_move(&mut self) {
        let point = get_point();
        let result = self.ai.enemy_move(point);
        self.human.player_move(result);
    }
    pub fn ai_move(&mut self, point: Point) {
        let result = self.human.enemy_move(point);
        self.ai.player_move(result);
    }
}
