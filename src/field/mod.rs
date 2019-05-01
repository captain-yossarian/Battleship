
use rand::{thread_rng, Rng};
use std::collections::HashMap;
use std::time::Instant;


#[derive(Debug)]
pub enum ShipDirection {
  Horizontal,
  Vertical,
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

type Field = [[Status; LEN as usize]; LEN as usize];
pub struct GameField {
  pub field: Field,
  pub ships: HashMap<u8, u8>,
  pub num:i32,
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
      field: [[Status::Empty; 12]; 12],
      ships,
      num:99
    }

  }


  pub fn reduce_ships(&mut self, size: &u8) {
    let val = self.ships.get_mut(&size).unwrap();
    *val -= 1;
  }

  pub fn create_ship(
    &mut self,
    size: u8,
    direction: &'static ShipDirection,
    point: Option<Point>,
  ) -> Option<Ship> {

    if self.check_permission(&size) == true {
      let mut draw = None;
      let mut start_point: Point;
      while draw.is_none() {
        match point {
          Some(value) => start_point = value,
          None => start_point = self.generate_random_point(&size, &direction),
        }
        draw = self.draw_ship(size, direction, start_point);
      }
      draw
    } else {
      None
    }
  }
  pub fn check_permission(&mut self, size: &u8) -> bool {
    self.ships.get(&size).unwrap() > &0
  }
  pub fn draw_ship_bounds(
    &mut self,
    direction: &ShipDirection,
    size: u8,
    point: Point,
  ) -> Option<()> {
    let bounds_path = self.generate_ship_bounds(direction, &size);

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
    let start = Instant::now();
    let drawed_ship = self.draw_ship_core(direction, start_point, size);
    let drawed_bounds = self.draw_ship_bounds(direction, size, start_point);

    match (drawed_ship, drawed_bounds) {
      (Some(()), Some(())) => {
        self.reduce_ships(&size);
        let end = Instant::now();
        let diff = end.duration_since(start);
        println!("Time diff {:?}", diff);
        Some(Ship {
          size,
          direction,
          start_point,
        })
      }
      _ => None,
    }

  }
  pub fn get_cell_value(&self, point: &Point) -> Status {
    let Point { row, column } = point;
    self.field[*row as usize][*column as usize]
  }
  pub fn scan_for(
    &self,
    path: &Vec<(Direction, u8)>,
    mut point: Point,
    statuses: Vec<Status>,
  ) -> bool {
    let mut allow = false;

    for step in path {
      let (direction, steps) = step;
      for _ in 0..*steps {
        let value = self.get_cell_value(point.go_to(&direction));
        allow = statuses.contains(&value);
        if allow == false {
          return allow;
        }
      }
    }
    allow
  }


  pub fn generate_random_point(&self, size: &u8, direction: &ShipDirection) -> Point {
    let mut random = thread_rng();
    let will_change = random.gen_range(1, 12 - size);
    let fixed = random.gen_range(1, 11);
    match direction {
      ShipDirection::Horizontal => Point {
        row: fixed,
        column: will_change,
      },
      ShipDirection::Vertical => Point {
        row: will_change,
        column: fixed,
      },
    }
  }
  pub fn draw_ship_core(
    &mut self,
    direction: &ShipDirection,
    mut start_point: Point,
    size: u8,
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

    if self.scan_for(&path, start_point, allowed_status) == true {
      path.iter().for_each(|(direction, steps)| {
        (0..*steps).collect::<Vec<u8>>().iter().for_each(|_| {
          self.draw_cell(start_point.go_to(direction), &draw_status);
        });
      });
      Some(())
    } else {
      None
    }
  }


  pub fn draw_cell(&mut self, point: &Point, status: &Status) {
    let Point { row, column } = point;
    self.field[*row as usize][*column as usize] = *status;
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

  pub fn generate_ship_bounds(&self, direction: &ShipDirection, size: &u8) -> Vec<(Direction, u8)> {
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

