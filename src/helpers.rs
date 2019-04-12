use std::collections::HashMap;
extern crate rand;
use rand::{thread_rng, Rng};

#[derive(Debug)]
pub enum ShipDirection {
  Horizontal,
  Vertical,
}

#[derive(Debug)]
enum ShipShape {
  Horizontal,
  Vertical,
}


#[derive(Debug)]
pub struct Ship {
  size: u8,
  direction: ShipDirection,
  point: Point,
}
impl Ship{
  pub fn end_point(&self)->u8{
   // row|column + size
  }
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
      let quadrant2 = LEN - size;
      let mut random = thread_rng();
      let point;

      match direction {
        ShipDirection::Horizontal => {
          point = Point {
            row: random.gen_range(0, LEN),
            column: random.gen_range(0, quadrant2),
          };
        }
        ShipDirection::Vertical => {
          point = Point {
            row: random.gen_range(0, quadrant2),
            column: random.gen_range(0, LEN),
          };
        }
      }
      Some(Ship {
        size,
        direction,
        point,
      })
    } else {
      None
    }
  }

  pub fn set_ship(&mut self, ship: Ship) {
    let quadrant2 = LEN - ship.size;
    let mut random = thread_rng();


    /*
    if ship.direction == ShipDirection::Up {
      let min_row_index = ship.size - 1;
          println!("MIn row {:?}", min_row_index);

    }
    */
  }
}

