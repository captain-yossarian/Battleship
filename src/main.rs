
mod field;
mod game;
mod utils;

use field::{GameField, Move, Point, ShipDirection};
use rand::{thread_rng, Rng};
use utils::random_number;

struct Foo {
  start: u8,
  end: u8,
}
impl Iterator for Foo {
  type Item = u8;
  fn next(&mut self) -> Option<u8> {
    if self.start >= self.end {
      return None;
    }
    let result = Some(self.start);
    self.start += 2;
    result
  }
}

fn main() {
  //use https://github.com/PistonDevelopers/piston

  let mut field = GameField::new(random_number);
  let size = 4;
  field.create_ship(
    size,
    &ShipDirection::Vertical,
    Some(Point { row: 5, column: 6 })
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