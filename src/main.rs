mod helpers;
use helpers::{GameField, ShipDirection};

fn main() {
  let mut field = GameField::new();
  field.show();

  let ship = field.create_ship(4, ShipDirection::Up);
  field.set_ship(ship.unwrap());
  //println!("Rev {:?}", ship);
}
