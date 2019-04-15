use std::collections::HashMap;

use rand::{thread_rng, Rng};
use std::ops::Range;

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
}

#[derive(Debug)]
pub struct Ship {
  size: u8,
  direction: ShipDirection,
  start_point: Point,
  end_point: Point,
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
 pub row: u8,
 pub column: u8,
}
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.row == other.row && self.column == other.column
    }
}

impl Point {
  pub fn go_to(&mut self, row: u8, column: u8) {
    self.row = row;
    self.column = column;
  }
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

pub const LEN: u8 = 12;

pub struct Coordinates {
  pub will_change: u8,
  pub fixed: u8,
  pub range: Range<u8>,
}

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


  pub fn reduce_ships(&mut self, size: &u8) {
    let val = self.ships.get_mut(&size).unwrap();
    *val -= 1;
  }

  pub fn draw_ship(&mut self, size: u8, direction: ShipDirection) -> Ship {
    let coordinates = self.random_coordinates(&size);
    let (start_point, end_point) = self.draw_ship_core(&direction, coordinates);

    let bounds = self.generate_ship_bounds(&direction, &size);
    self.draw_ship_bounds(start_point.clone(), bounds);
    self.reduce_ships(&size);

    Ship {
      size,
      direction,
      start_point,
      end_point,
    }
  }


  pub fn check_permission(&mut self, size: &u8) -> bool {
    self.ships.get(&size).unwrap() > &0
  }

  pub fn create_ship(&mut self, size: u8, direction: ShipDirection) -> Option<Ship> {
    if self.check_permission(&size) == true {
      Some(self.draw_ship(size, direction))
    } else {
      None
    }
  }
  pub fn random_coordinates(&self, size: &u8) -> Coordinates {
    let mut random = thread_rng();
    let will_change = random.gen_range(1, 12 - size);
    let fixed = random.gen_range(1, 11);
    let range = will_change..will_change + size;
    Coordinates {
      will_change,
      fixed,
      range,
    }
  }
  pub fn draw_ship_core(
    &mut self,
    direction: &ShipDirection,
    coordinates: Coordinates,
  ) -> (Point, Point) {
    let Coordinates {
      will_change,
      fixed,
      range,
    } = coordinates;
    let start_point;
    let mut end_point;

    match direction {
      ShipDirection::Horizontal => {
        start_point = Point {
          row: fixed,
          column: will_change,
        };
        end_point = start_point.clone();
        for _ in range {
          self.draw_cell(&end_point, Status::Ship);
          end_point.right();
        }
      }
      ShipDirection::Vertical => {
        start_point = Point {
          row: will_change,
          column: fixed,
        };
        end_point = start_point.clone();
        for _ in range {
          self.draw_cell(&end_point, Status::Ship);
          end_point.down();
        }
      }
    }
    (start_point, end_point)
  }


  fn draw_ship_bounds(&mut self, mut point: Point, path: Vec<DrawStep>) {
    for step in path {
      let DrawStep { direction, cells } = step;
      for _ in 0..cells {
        match direction {
          Direction::Up => point.up(),
          Direction::Left => point.left(),
          Direction::Right => point.right(),
          Direction::Down => point.down(),
        }
        self.draw_cell(&point, Status::Bound);
      }
    }
  }


  pub fn draw_cell(&mut self, point: &Point, fixed: Status) {
    let Point { row, column } = point;
    let value = self.field[*row as usize][*column as usize];


    if value == Status::Empty as u8 {
      self.field[*row as usize][*column as usize] = fixed as u8;
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

  fn generate_ship_bounds(&self, direction: &ShipDirection, size: &u8) -> Vec<DrawStep> {
    let long_shot = size + 1;
    match direction {
      ShipDirection::Horizontal => vec![
        DrawStep {
          direction: Direction::Left,
          cells: 1,
        },
        DrawStep {
          direction: Direction::Up,
          cells: 1,
        },
        DrawStep {
          direction: Direction::Right,
          cells: long_shot,
        },
        DrawStep {
          direction: Direction::Down,
          cells: 2,
        },
        DrawStep {
          direction: Direction::Left,
          cells: long_shot,
        },
      ],
      ShipDirection::Vertical => vec![
        DrawStep {
          direction: Direction::Up,
          cells: 1,
        },
        DrawStep {
          direction: Direction::Right,
          cells: 1,
        },
        DrawStep {
          direction: Direction::Down,
          cells: long_shot,
        },
        DrawStep {
          direction: Direction::Left,
          cells: 2,
        },
        DrawStep {
          direction: Direction::Up,
          cells: long_shot,
        },
      ],
    }
  }
}

