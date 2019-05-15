mod field;
mod game;
mod utils;

use field::{GameField, Move, Point, ShipDirection};
use rand::{thread_rng, Rng};
use utils::random_number;

use std::sync::{mpsc, Arc, Mutex};
use std::thread::spawn;

fn main() {
  //use https://github.com/PistonDevelopers/piston

  let mut field = GameField::new(random_number);
  let size = 4;
  field.create_ship(
    size,
    &ShipDirection::Vertical,
    Some(Point { row: 5, column: 6 }),
  );
  
    let point = Point { row: 5, column: 6 };

    let result = field.player_move(point);
    match result {
      Move::Kill(p) => println!("Kill {:?}", p),
      Move::Miss(p) => println!("Miss {:?}", p),
    }
  
  // field.generate_random_field(random_number);
  field.show();
}

#[cfg(test)]
mod test;
