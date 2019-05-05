
mod field;
mod game;
mod utils;

use field::{GameField, Point, ShipDirection};
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

  let mut field = GameField::new();
  let foo = Foo {start:10,end:20};
  for i in foo {
    println!("ELEM {}",i);
  }

  field.generate_random_field(random_number);
 field.show();
}

#[cfg(test)]
mod test;