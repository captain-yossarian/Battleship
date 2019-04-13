use std::collections::HashMap;
use std::ops::Range;
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


#[derive(Debug)]
pub struct Point {
  row: u8,
  column: u8,
}

const LEN: u8 = 10;

type Field = [[u8; LEN as usize]; LEN as usize];
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
      let mut random = thread_rng();
      let quadrant2 = LEN - size;
      let dynamic = random.gen_range(0, quadrant2);
      let stat = random.gen_range(0, LEN);
      let range = dynamic..dynamic + size;
      let point;

      match direction {
        ShipDirection::Horizontal => {
          point = Point {
            row: stat,
            column: dynamic,
          };
          for index in range {
            self.draw(stat, index);
          }
          println!("Horizontal point {:?}", point);
        }
        ShipDirection::Vertical => {
          point = Point {
            row: dynamic,
            column: stat,
          };
          for index in range {
            self.draw(index, stat);
          }
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


  pub fn draw(&mut self, x: u8, y: u8) {
    if x != 0 {
      self.field[(x - 1) as usize][y as usize] = 2;
   
    }
   // self.field[x as usize][(y - 1) as usize] = 2;
    self.field[x as usize][y as usize] = 1;
  //  self.field[x as usize][(y + 1) as usize] = 2;

    if x != 9 {
      self.field[(x + 1) as usize][y as usize] = 2;
    }
  }

}

