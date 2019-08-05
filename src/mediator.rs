use crate::player;
use crate::structures;
use crate::utils;
use player::Player;
use std::io;
use structures::{Move, Point, ShipDirection};
use utils::random_number;

pub struct Mediator {
    pub human: Player,
    pub ai: Player,
}
fn read_line() -> String {
    ///read_line allocates a new String every time, while okay for user input,
    /// this could probably be better if we took a &mut String instead.
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");
    input_text
}

fn parse_number(direction: &str) -> u8 {
    println!("Enter {}", direction);
    let line = read_line();
    let trimmed = line.trim();
    let mut number = 0;
    if let Ok(i) = trimmed.parse::<u8>() {
        number = i
    };
    number
}
fn get_point() -> Point {
    let row = parse_number("row");
    let column = parse_number("column");
    println!("User moved, row: {:?}, column: {:?}", row, column);
    Point { row, column }
}
impl Mediator {
    pub fn new() -> Mediator {
        let mut human = Player::new(random_number);
        let mut ai = Player::new(random_number);
        ai.init();
        human.init();
        Mediator { human, ai }
    }

    pub fn human_move(&mut self) {
        let mut missed = false;
        while !missed {
            let point = get_point();
            let result = self.ai.enemy_attack(point);
            self.human.player_move(&result);
            if let Move::Miss(_) = result {
                missed = true
            }
        }
    }
    pub fn ai_move(&mut self) {
        let mut missed = false;
        while !missed {
            let random_point = self
                .ai
                .enemy_field
                .generate_random_point(&ShipDirection::Horizontal, 1);
            let result = self.human.enemy_attack(random_point);
            self.ai.player_move(&result);
            println!("AI moved {:?}, result: {:?}", random_point, result);
            if let Move::Miss(_) = result {
                missed = true
            }
        }
    }
}
