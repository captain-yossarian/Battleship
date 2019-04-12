mod helpers;
use helpers::{GameField, ShipDirection};

fn main() {
  let mut field = GameField::new();
  field.show();

  let ship = field.create_ship(3, ShipDirection::Horizontal);
  field.set_ship(ship.unwrap());
  //println!("Rev {:?}", ship);
}

#[cfg(test)]
mod test;