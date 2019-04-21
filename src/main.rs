extern crate azul;
// extern crate azul;
use azul::prelude::*;
mod battleship;
use battleship::{GameField, Point, ShipDirection};

struct MyDataModel {}

impl Layout for MyDataModel {
  fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
    Dom::div()
  }
}

fn main() {
  /*
        let mut app = App::new(MyDataModel { }, AppConfig::default()).unwrap();
      let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
      app.run(window).unwrap();
  */

  let mut field = GameField::new();
  let size = 4;
 field.create_ship(
    4,
    &ShipDirection::Vertical,
    Some(Point {row:5,column:6}),
  );
 /*
  field.create_ship(
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