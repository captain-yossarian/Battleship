
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::ops::Range;
use std::thread;


#[derive(Debug)]
pub enum ShipDirection {
  Horizontal,
  Vertical,
}

#[derive(Debug)]
enum Direction {
  Up(u8),
  Right(u8),
  Down(u8),
  Left(u8),
}

#[derive(Debug)]
struct DrawStep {
  direction: Direction,
  cells: u8,
}

#[derive(Debug)]
pub struct Ship {
  size: u8,
  direction: ShipDirection,
  start_point: Point,
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

pub fn forEach<F>(len: u8, mut callback: F)
where
  F: FnMut(),
{
  for _ in 0..len {
    callback()
  }
}

impl Point {
  fn go_to(&mut self, row: u8, column: u8) {
    self.row = row;
    self.column = column;
  }

  fn up(&mut self) {
    self.row -= 1;
  }

  fn right(&mut self) {
    self.column += 1;

  }
  fn down(&mut self) {
    self.row += 1;

  }
  fn left(&mut self) {
    self.column -= 1;

  }
}


pub const LEN: u8 = 12;

pub struct Coordinates {
  pub will_change: u8,
  pub fixed: u8,
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


  pub fn create_ship(&mut self, size: u8, direction: ShipDirection) -> Option<Ship> {
    if self.check_permission(&size) == true {
      Some(self.draw_ship(size, direction))
    } else {
      None
    }
  }
  pub fn check_permission(&mut self, size: &u8) -> bool {
    self.ships.get(&size).unwrap() > &0
  }

  pub fn draw_ship(&mut self, size: u8, direction: ShipDirection) -> Ship {
    let coordinates = self.random_coordinates(&size);
    let start_point = self.draw_ship_core(&direction, coordinates, size);
    println!("Start point {:?}", start_point);
    let bounds = self.generate_ship_bounds(&direction, &size);
    let clone_point = start_point.clone();
    self.draw(clone_point, bounds, Status::Bound);
    self.reduce_ships(&size);

    Ship {
      size,
      direction,
      start_point,
    }
  }

  pub fn random_coordinates(&self, size: &u8) -> Coordinates {
    let mut random = thread_rng();
    let will_change = random.gen_range(1, 12 - size);
    let fixed = random.gen_range(1, 11);
    Coordinates { will_change, fixed }
  }
  pub fn draw_ship_core(
    &mut self,
    direction: &ShipDirection,
    coordinates: Coordinates,
    size: u8,
  ) -> Point {
    let Coordinates { will_change, fixed } = coordinates;
    let start_point;
    match direction {
      ShipDirection::Horizontal => {
        start_point = Point {
          row: fixed,
          column: will_change,
        };
        let path = vec![Direction::Right(size)];
        let mut clone_point = start_point.clone();
        clone_point.left();
        self.draw(clone_point, path, Status::Ship);
      }
      ShipDirection::Vertical => {
        start_point = Point {
          row: will_change,
          column: fixed,
        };
        let path = vec![Direction::Down(size)];
        let mut clone_point = start_point.clone();
        clone_point.up();
        self.draw(clone_point, path, Status::Ship);
      }
    }
    start_point
  }


  fn draw(&mut self, mut point: Point, path: Vec<Direction>, status: Status) -> Option<bool> {
    //  let mut success: bool = false;
    let point = &mut point;
    for direction in path {
      match direction {
        Direction::Up(len) => forEach(len, || {
          point.up();
          self.draw_cell(&point, &status);
        }),
        Direction::Left(len) => forEach(len, || {
          point.left();
          self.draw_cell(&point, &status);
        }),
        Direction::Right(len) => forEach(len, || {
          point.right();
          self.draw_cell(&point, &status);
        }),
        Direction::Down(len) => forEach(len, || {
          point.down();
          self.draw_cell(&point, &status);
        }),
      }
      /*
            success = if self.draw_cell(&point, &status).is_some() {
              true
            } else {
              false
            }
      */
    }
    Some(true)
  }


  pub fn draw_cell(&mut self, point: &Point, status: &Status) -> Option<()> {
    let Point { row, column } = point;
    let value = &mut self.field[*row as usize][*column as usize];
    let success = *value == 0 || *value == 2;
    if success == true {
      match status {
        Status::Empty => *value = 0,
        Status::Ship => {
          println!("Draw ship SHIP status");
          *value = 1
        }
        Status::Bound => *value = 2,
        Status::Kill => *value = 3,
      }
      Some(())
    } else {
      None
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

  fn generate_ship_bounds(&self, direction: &ShipDirection, size: &u8) -> Vec<Direction> {
    let long_shot = size + 1;
    match direction {
      ShipDirection::Horizontal => vec![
        Direction::Left(1),
        Direction::Up(1),
        Direction::Right(long_shot),
        Direction::Down(2),
        Direction::Left(long_shot),
      ],
      ShipDirection::Vertical => vec![
        Direction::Up(1),
        Direction::Right(1),
        Direction::Down(long_shot),
        Direction::Left(2),
        Direction::Up(long_shot),
      ],
    }
  }
}

