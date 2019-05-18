use std::collections::HashMap;

#[derive(Debug)]
pub enum ShipDirection {
    Horizontal,
    Vertical,
}

#[derive(Debug)]
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

#[derive(Debug, Copy, Clone)]
pub struct Ship {
    pub size: u8,
    pub direction: &'static ShipDirection,
    pub start_point: Point,
}

impl Ship {
    pub fn get_all() -> HashMap<u8, u8> {
        let mut ships = HashMap::new();
        let keys: [u8; 4] = [1, 2, 3, 4];
        let mut values = keys.iter().rev();

        for &key in keys.iter() {
            ships.insert(key, *values.next().unwrap());
        }
        ships
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Status {
    Empty,
    Ship,
    Bound,
    Kill,
}

pub type RandomNumber = fn(u8, u8) -> u8;

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

impl Point {
    pub fn go_to(&mut self, direction: &Direction) -> &mut Self {
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
pub fn convert_to_u8(elem: &[Status; 12]) -> Vec<u8> {
    elem.to_vec()
        .into_iter()
        .map(status_u8)
        .collect::<Vec<u8>>()
}
pub type Field = [[Status; LEN as usize]; LEN as usize];
