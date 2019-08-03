use crate::player;
use crate::structures;
use crate::utils;
use std::io;
use std::io::Read;
use utils::random_number;

use player::Player;
use structures::{Move, Point};

pub struct Mediator {
    pub human: Player,
    pub ai: Player,
}
fn read_line() -> String {
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    input_text
}

fn get_number(direction: &str) -> u8 {
    println!("Enter {}", direction);
    let line = read_line();
    let trimmed = line.trim();
    let mut number = 0;
    match trimmed.parse::<u8>() {
        Ok(i) => number = i,
        Err(..) => (),
    };
    number
}
fn get_point() -> Point {
    let row = get_number("row");
    let column = get_number("column");
    println!("User point, row: {:?}, column: {:?}", row, column);
    Point { row, column }
}
impl Mediator {
    pub fn new() -> Mediator {
        Mediator {
            human: Player::new(random_number),
            ai: Player::new(random_number),
        }
    }
    pub fn human_move(&mut self) {
        let mut missed = false;
        while !missed {
            let point = get_point();
            let result = self.ai.enemy_attack(point);
            self.human.player_move(&result);
            match result {
                Move::Miss(_) => missed = true,
                _ => (),
            }
        }
    }
    pub fn ai_move(&mut self, point: Point) {        
        let result = self.human.enemy_attack(point);
        self.ai.player_move(&result);
    }
}
