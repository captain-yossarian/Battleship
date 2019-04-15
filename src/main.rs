mod helpers;
use helpers::{GameField, ShipDirection};

fn main() {
  let mut field = GameField::new();

  let ship = field.create_ship(4, ShipDirection::Vertical);
 // let ship2 = field.create_ship(4, ShipDirection::Vertical);
  field.show();

  //println!("Rev {:?}", ship);
}

#[cfg(test)]
mod test;