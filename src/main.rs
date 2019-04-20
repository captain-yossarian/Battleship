extern crate azul;
// extern crate azul;
use azul::prelude::*;
mod battleship;
use battleship::{GameField, ShipDirection};


struct MyDataModel { }

impl Layout for MyDataModel {
    fn layout(&self, _: LayoutInfo<Self>) -> Dom<Self> {
        Dom::div()
    }
}

fn main() {


      let mut app = App::new(MyDataModel { }, AppConfig::default()).unwrap();
    let window = app.create_window(WindowCreateOptions::default(), css::native()).unwrap();
    app.run(window).unwrap();



  let mut field = GameField::new();
  field.create_ship(4, &ShipDirection::Vertical);

  field.create_ship(3, &ShipDirection::Vertical);
  field.create_ship(3, &ShipDirection::Vertical);

  field.create_ship(2, &ShipDirection::Vertical);
  field.create_ship(2, &ShipDirection::Vertical);
  field.create_ship(2, &ShipDirection::Vertical);

  field.create_ship(1, &ShipDirection::Vertical);
  field.create_ship(1, &ShipDirection::Vertical);
  field.create_ship(1, &ShipDirection::Vertical);
  field.create_ship(1, &ShipDirection::Vertical);
  

  field.show();
}

#[cfg(test)]
mod test;