mod helpers;
use helpers::{GameField, ShipDirection};

fn main() {
  let mut field = GameField::new();
  field.create_ship(4, ShipDirection::Vertical);




  field.show();
}

#[cfg(test)]
mod test;