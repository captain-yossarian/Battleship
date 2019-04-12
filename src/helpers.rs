use std::collections::HashMap;
extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum ShipDirection {
  Up,
  Down,
  Left,
  Right,
}

enum ShipShape {
  Horizontal,
  Vertical
}

#[derive(Debug)]
pub struct ShipCoordinates {
  start: u8,
  end: u8,
  shape:ShipShape
}

#[derive(Debug)]
pub struct Ship {
  size: u8,
  direction: ShipDirection,
}

#[derive(Debug)]
pub struct Point {
  row: u8,
  column: u8,
}

const LEN: u8 = 10;

type Field = [[i32; LEN as usize]; LEN as usize];
pub struct GameField {
  pub field: Field,
  pub ships: HashMap<u8, u8>,
}

impl GameField {
  pub fn new() -> GameField {
    let mut ships = HashMap::new();
    let keys = [1, 2, 3, 4];
    let mut values = keys.iter().rev();

    for &key in keys.iter() {
      ships.insert(key, *values.next().unwrap());
    }

    GameField {
      field: [[0; 10]; 10],
      ships,
    }
  }
  pub fn show(&self) {
    let columns = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    println!("col   {:?}", columns);
    println!("------------------------------------");
    for (index, elem) in self.field.iter().enumerate() {
      println!("row:{} {:?}", index, elem);
    }
  }

  fn reduce_ships(&mut self, size: &u8) {
    let val = self.ships.get_mut(&size).unwrap();
    *val -= 1;
    println!("Val {}", val);
  }

  pub fn create_ship(&mut self, size: u8, direction: ShipDirection) -> Option<Ship> {
    let allow = self.ships.get(&size).unwrap() > &0;
    if allow == true {
      self.reduce_ships(&size);
      Some(Ship { size, direction })
    } else {
      None
    }
  }

  pub fn set_ship(&mut self, ship: Ship) {

    let quadrant2 = LEN - ship.size;
    let quadrant4 = ship.size - 1;
    let mut random = thread_rng();

    match ship.direction {
      ShipDirection::Up => {
        let row = random.gen_range(quadrant4, LEN);
        let point = Point {
          row,
          column: random.gen_range(0, LEN),
          core:row - (ship.size - 1)..row + 1,
        };
      }
         ShipDirection::Left => {
        let column = random.gen_range(quadrant4, LEN);
        let point = Point {
          column,
          row: random.gen_range(0, LEN),
          core:column - (ship.size - 1)..column + 1,
        };
      }
      ShipDirection::Right => {
        let column = random.gen_range(0, quadrant2);
        let point = Point {
          row: random.gen_range(0, LEN),
          column ,
          core: column..column + ship.size
        };
      }
      ShipDirection::Down => {
        let row = random.gen_range(0, quadrant2);
        let point = Point {
          row,
          column: random.gen_range(0, LEN),
          core:row..row + ship.size
        };
      }
   
      _ => println!("Error"),
    }

    /*
    if ship.direction == ShipDirection::Up {
      let min_row_index = ship.size - 1;
          println!("MIn row {:?}", min_row_index);

    }
    */
  }
}

