mod helpers;
use helpers::{GameField, ShipDirection};

fn main() {
  let mut field = GameField::new();
  field.show();

  let ship = field.create_ship(4, ShipDirection::Horizontal);
  println!("Rev {:?}", ship);
}
