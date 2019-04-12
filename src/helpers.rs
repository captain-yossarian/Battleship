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

  fn vector(&self, ship: &Ship, point: Point) -> bool {
    let Ship { direction, size } = ship;
    let Point { row, column } = point;
    let vector = vec![0; *size as usize];
    let mut i: u8 = 0;
    /**
     * @todo add forEach https://docs.rs/foreach/0.3.0/foreach/
     */

    for (index, elem) in vector.iter().enumerate() {
      let custom_row = row - i;

      // println!("Index, elem, {:?}", self.field[*size as usize]);
      println!(
        "Index, elem, {:?} {}",&custom_row,
        self.field[custom_row as usize][column as usize]
      );
      i += 1;

      //   println!("Range {} {:?}",index,elem);
    }


    true

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

    // let mut rng = rand::random::<usize>();
    //let roll = rng.gen_range(1, 7);

    let quadrant2 = LEN - ship.size;
    let quadrant4 = ship.size - 1;
    let mut random = thread_rng();

    match ship.direction {
      ShipDirection::Up => {
        let min_row_index = quadrant4;
        let point = Point {
          row: random.gen_range(min_row_index, LEN),
          column: random.gen_range(0, LEN),
        };
        let is_empty_direction = self.vector(&ship, point);
        println!("Allow {}, Direction UP", is_empty_direction);
      }
      ShipDirection::Right => {
        let max_column_index = quadrant2;
        let random_column = random.gen_range(0, max_column_index);
      }
      ShipDirection::Down => {
        let max_row_index = quadrant2;
        let random_row = random.gen_range(0, max_row_index);
      }
      ShipDirection::Left => {
        let min_column_index = quadrant4;
        let random_column = random.gen_range(min_column_index, LEN);
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

