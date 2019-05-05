
use crate::utils;
use rand::{thread_rng, Rng};

use std::collections::HashMap;
use std::thread;
use utils::generate_all_empty_points;

//use super::utils::{convert_tu_u8,status_u8};

#[derive(Debug)]
pub enum ShipDirection {
  Horizontal,
  Vertical,
}


pub enum Move {
  Miss(Point),
  Kill(Point),
}

#[derive(Debug)]
pub enum Direction {
  Up,
  Right,
  Down,
  Left,
}


#[derive(Debug)]
pub struct Ship {
  size: u8,
  direction: &'static ShipDirection,
  start_point: Point,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
  Empty,
  Ship,
  Bound,
  Kill,
}


type RandomNumber = fn(u8, u8) -> u8;
pub struct Draw {
  pub start_point: Point,
  pub path: Vec<(Direction, u8)>,
  pub draw_status: Status,
  pub allowed_status: Vec<Status>,
}

#[derive(Debug, Clone, Copy)]
pub struct Point {
  pub row: u8,
  pub column: u8,
}
impl PartialEq for Point {
  fn eq(&self, other: &Point) -> bool {
    self.row == other.row && self.column == other.column
  }
}

/**
 * @todo
 * convert argument [size,direction,point] to Ship structure
 */
impl Point {
  fn go_to(&mut self, direction: &Direction) -> &mut Self {
    match direction {
      Direction::Up => self.row -= 1,
      Direction::Left => self.column -= 1,
      Direction::Right => self.column += 1,
      Direction::Down => self.row += 1,
    }
    self
  }
}


pub const LEN: u8 = 12;

pub fn status_u8(status: Status) -> u8 {
  match status {
    Status::Empty => 0,
    Status::Ship => 1,
    Status::Bound => 2,
    Status::Kill => 3,
  }

}
pub fn convert_tu_u8(elem: &[Status; 12]) -> Vec<u8> {
  elem
    .to_vec()
    .into_iter()
    .map(status_u8)
    .collect::<Vec<u8>>()
}

pub type Field = [[Status; LEN as usize]; LEN as usize];

pub struct GameField {
  pub field: Field,
  pub ships: HashMap<u8, u8>,
}

impl GameField {

  pub fn new() -> GameField {
    let mut ships = HashMap::new();
    let keys: [u8; 4] = [1, 2, 3, 4];
    let mut values = keys.iter().rev();

    for &key in keys.iter() {
      ships.insert(key, *values.next().unwrap());
    }
    let field = [[Status::Empty; 12]; 12];

    GameField { field, ships }
  }

  pub fn generate_random_field(&mut self, callback: RandomNumber) {
    let ships = self.ships.clone();
    let queue = [4, 3, 2, 1];
    queue.iter().for_each(|key| {
      if let Some(val) = ships.get(key) {
        for _ in 1..=*val {
          self.create_ship(*key, &ShipDirection::Vertical, None, callback);
        }
      }
    });
  }
  pub fn generate_random_point(
    &mut self,
    direction: &ShipDirection,
    size: u8,
    callback: RandomNumber,
  ) -> Point {
    let empty_points = generate_all_empty_points(self.field);
    let allowed_points = empty_points
      .iter()
      .filter(|point| match direction {
        ShipDirection::Horizontal => point.column < 12 - size,
        ShipDirection::Vertical => point.row < 12 - size,
      })
      .cloned()
      .collect::<Vec<Point>>();
    let point_index = callback(0, allowed_points.len() as u8);
    allowed_points[point_index as usize]
  }


  pub fn reduce_ships(&mut self, size: u8) {
    let val = self.ships.get_mut(&size).unwrap();
    *val -= 1;
  }

  pub fn create_ship(
    &mut self,
    size: u8,
    direction: &'static ShipDirection,
    point: Option<Point>,
    callback: RandomNumber,
  ) -> Option<Ship> {
    if self.check_permission(size) {
      let mut draw = None;
      let mut start_point: Point;
      let mut random_points: Vec<Point> = vec![];

      while draw.is_none() {
        match point {
          Some(value) => start_point = value,
          None => {
            start_point = {
              let mut temp_point;
              loop {
                temp_point = self.generate_random_point(&direction, size, callback);
                if random_points
                  .iter()
                  .find(|point| **point == temp_point)
                  .is_none()
                {
                  random_points.push(temp_point);
                  break;
                }
              }
              temp_point
            }
          }
        }
        draw = self.draw_ship(size, direction, start_point);
      }
      draw
    } else {
      None
    }
  }


  pub fn check_permission(&mut self, size: u8) -> bool {
    self.ships[&size] > 0
  }
  pub fn draw_ship_bounds(
    &mut self,
    direction: &ShipDirection,
    size: u8,
    point: Point,
  ) -> Option<()> {
    let bounds_path = self.generate_ship_bounds(direction, size);

    self.draw_by_path(Draw {
      start_point: point,
      path: bounds_path,
      draw_status: Status::Bound,
      allowed_status: vec![Status::Bound, Status::Empty],
    })
  }

  pub fn draw_ship(
    &mut self,
    size: u8,
    direction: &'static ShipDirection,
    start_point: Point,
  ) -> Option<Ship> {
    self.draw_ship_core(direction, size, start_point)?;
    self.draw_ship_bounds(direction, size, start_point)?;
    self.reduce_ships(size);
    Some(Ship {
      size,
      direction,
      start_point,
    })
  }
  pub fn get_cell_value(&self, point: Point) -> Status {
    let Point { row, column } = point;
    self.field[row as usize][column as usize]
  }
  pub fn player_move(&mut self, point: Point) -> Move {
    match self.get_cell_value(point) {
      Status::Ship => {
        self.draw_cell(point, Status::Kill);
        Move::Kill(point)
      }
      _ => Move::Miss(point),
    }
  }
  pub fn scan_for(
    &self,
    path: &[(Direction, u8)],
    mut point: Point,
    statuses: Vec<Status>,
  ) -> bool {
    let mut allow = false;

    for step in path {
      let (direction, steps) = step;
      for _ in 0..*steps {
        let value = self.get_cell_value(*point.go_to(&direction));
        allow = statuses.contains(&value);
        if !allow {
          return allow;
        }
      }
    }
    allow
  }


  pub fn draw_ship_core(
    &mut self,
    direction: &ShipDirection,
    size: u8,
    mut start_point: Point,
  ) -> Option<()> {
    let path: Vec<(Direction, u8)>;
    match direction {
      ShipDirection::Horizontal => {
        path = vec![(Direction::Right, size)];
        start_point.go_to(&Direction::Left);
      }
      ShipDirection::Vertical => {
        path = vec![(Direction::Down, size)];
        start_point.go_to(&Direction::Up);
      }
    }

    self.draw_by_path(Draw {
      start_point,
      path,
      draw_status: Status::Ship,
      allowed_status: vec![Status::Empty],
    })
  }

  pub fn draw_by_path(&mut self, draw: Draw) -> Option<()> {
    let Draw {
      mut start_point,
      path,
      draw_status,
      allowed_status,
    } = draw;

    if self.scan_for(&path, start_point, allowed_status) {      
      path.iter().for_each(|(direction, steps)| {        
        (0..*steps).collect::<Vec<u8>>().iter().for_each(|_| {
          self.draw_cell(*start_point.go_to(direction), draw_status);
        });
      });
      Some(())
    } else {
      None
    }
  }


  pub fn draw_cell(&mut self, point: Point, status: Status) {
    let Point { row, column } = point;
    self.field[row as usize][column as usize] = status;
  }


  pub fn show(&self) {
    let columns = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("col         {:?}", columns);
    println!("            -------------------------------");
    for (index, elem) in self.field.iter().enumerate() {
      if index <= 9 {
        println!("row:{}    {:?}", index, convert_tu_u8(elem));
      }
      if index > 9 {
        println!("row:{}   {:?}", index, convert_tu_u8(elem));
      }
    }
  }

  pub fn generate_ship_bounds(&self, direction: &ShipDirection, size: u8) -> Vec<(Direction, u8)> {
    let long_shot = size + 1;
    match direction {
      ShipDirection::Horizontal => {
        let path = vec![
          (Direction::Left, 1),
          (Direction::Up, 1),
          (Direction::Right, long_shot),
          (Direction::Down, 2),
          (Direction::Left, long_shot),
        ];
        path
      }
      ShipDirection::Vertical => {
        let path = vec![
          (Direction::Up, 1),
          (Direction::Right, 1),
          (Direction::Down, long_shot),
          (Direction::Left, 2),
          (Direction::Up, long_shot),
        ];
        path
      }
    }
  }
}

