mod field;
mod game;
use field::{GameField, Point, ShipDirection};



fn main() {
  //use https://github.com/PistonDevelopers/piston

  let mut field = GameField::new();

  field.create_ship(
    4,
    &ShipDirection::Vertical,
    Some(Point { row: 5, column: 6 }),
  );

  /*
      4,
      &ShipDirection::Vertical,
      None,
    );
      field.create_ship(
      3,
      &ShipDirection::Vertical,
      None,
    );
    field.create_ship(
      3,
      &ShipDirection::Vertical,
      None,
    );

    field.create_ship(
      2,
      &ShipDirection::Vertical,
      None,
    );
    field.create_ship(
      2,
      &ShipDirection::Vertical,
      None,
    );
    field.create_ship(
      2,
      &ShipDirection::Vertical,
      None,
    );

      field.create_ship(
      1,
      &ShipDirection::Vertical,
      None,
    );
      1,
      &ShipDirection::Vertical,
      None,
    );
    field.create_ship(
      1,
      &ShipDirection::Vertical,
      None,
    );
     field.create_ship(
      1,
      &ShipDirection::Vertical,
      None,
    );
  */
  field.show();
}

#[cfg(test)]
mod test;