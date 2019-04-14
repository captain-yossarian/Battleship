mod helpers;
use helpers::{GameField, ShipDirection};

fn main() {
  let mut field = GameField::new();

  let ship = field.create_ship(2, ShipDirection::Horizontal);
  field.show();

  //println!("Rev {:?}", ship);
}

#[cfg(test)]
mod test;