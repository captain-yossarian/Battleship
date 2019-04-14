use std::collections::HashMap;

use rand::{thread_rng, Rng};
use std::ops::{Range, Sub};

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

enum Direction {
  Up,
  Right,
  Down,
  Left,
}

struct DrawStep {
  direction: Direction,
  cells: u8,
  increment: i8,
}

#[derive(Debug)]
pub struct Ship {
  size: u8,
  direction: ShipDirection,
  point: Point,

}

#[derive(Debug)]
pub enum Status {
  Empty,
  Ship,
  Bound,
  Kill,
}

#[derive(Debug, Clone)]
pub struct Point {
  row: u8,
  column: u8,
}

impl Point {
  pub fn up(&mut self) {
    self.row -= 1;
  }

  pub fn right(&mut self) {
    self.column += 1;
  }
  pub fn down(&mut self) {
    self.row += 1;
  }
  pub fn left(&mut self) {
    self.column -= 1;
  }
}

const LEN: u8 = 12;

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
      field: [[0; 12]; 12],
      ships,
    }
  }
  pub fn show(&self) {

    let columns = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("col         {:?}", columns);
    println!("            -------------------------------");
    for (index, elem) in self.field.iter().enumerate() {
      if index <= 9 {
        println!("row:{}    {:?}", index, elem);
      }
      if index > 9 {
        println!("row:{}   {:?}", index, elem);
      }
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
      let dynamic = random.gen_range(1, 12 - size);
      let stat = random.gen_range(1, 11);
      let range = dynamic..dynamic + size;
      let point;

      match direction {
        ShipDirection::Horizontal => {
          point = Point {
            row: stat,
            column: dynamic,
          };
          for index in range {
            self.draw(stat, index, Status::Ship);
          }
          let path = self.wrap_ship_horizontal(&size);
          self.draw_cell(point.clone(), path);
        }
        ShipDirection::Vertical => {
          point = Point {
            row: dynamic,
            column: stat,
          };
          for index in range {
            self.draw(index, stat, Status::Ship);
          }
          let path = self.wrap_ship_vertical(&size);
          self.draw_cell(point.clone(), path);
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

  fn wrap_ship_horizontal(&self, size: &u8) -> Vec<DrawStep> {
    let long_shot = size + 1;
    vec![
      DrawStep {
        direction: Direction::Left,
        cells: 1,
        increment: -1,
      },
      DrawStep {
        direction: Direction::Up,
        cells: 1,
        increment: -1,
      },
      DrawStep {
        direction: Direction::Right,
        cells: long_shot,
        increment: 1,
      },
      DrawStep {
        direction: Direction::Down,
        cells: 2,
        increment: 1,
      },
      DrawStep {
        direction: Direction::Left,
        cells: long_shot,
        increment: -1,
      },
    ]
  }

  fn wrap_ship_vertical(&self, size: &u8) -> Vec<DrawStep> {
    let long_shot = size + 1;


    vec![
      DrawStep {
        direction: Direction::Up,
        cells: 1,
        increment: -1,
      },
      DrawStep {
        direction: Direction::Right,
        cells: 1,
        increment: 1,
      },
      DrawStep {
        direction: Direction::Down,
        cells: long_shot,
        increment: 1,
      },
      DrawStep {
        direction: Direction::Left,
        cells: 2,
        increment: -1,
      },
      DrawStep {
        direction: Direction::Up,
        cells: long_shot,
        increment: -1,
      },
    ]
  }
  fn drawer(&mut self, mut callback: Box<FnMut() -> u8>) {
    let result = callback();
    println!("After {}", result);
  }
  fn draw_cell(&mut self, point: Point, path: Vec<DrawStep>) {
    let Point {
      mut row,
      mut column,
    } = point;

    for step in path {
      let DrawStep {
        direction,
        cells,
        increment,
      } = step;

      match direction {
        Direction::Up => {
          for _ in 0..cells {
            row -= 1;
            self.draw(row, column, Status::Bound);
          }
        }
        Direction::Left => {
          for _ in 0..cells {
            column -= 1;
            self.draw(row, column, Status::Bound);
          }
        }
        Direction::Right => {
          for _ in 0..cells {
            column += 1;
            self.draw(row, column, Status::Bound);
          }
        }
        Direction::Down => {
          for _ in 0..cells {
            row += 1;
            self.draw(row, column, Status::Bound);
          }
        }

      }
    }

  }

  pub fn draw(&mut self,point:Point, status: Status) {
    let Point {
      row, column
    } = point;
    self.field[row as usize][column as usize] = status as u8;
  }
}

