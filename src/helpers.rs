use std::collections::HashMap;

#[derive(Debug)]
pub struct Ship {
  size: i32,
  direction: ShipDirection,
}

#[derive(Debug)]
pub enum ShipDirection {
  Vertical,
  Horizontal,
}

const LEN: usize = 10;

type Field = [[i32; LEN]; LEN];
pub struct GameField {
  field: Field,
  ships: HashMap<i32, i32>,
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

  fn reduce_ships(&mut self, size: &i32) {
    let val = self.ships.get_mut(&size).unwrap();
    *val -= 1;
    println!("Val {}", val);
  }
  pub fn create_ship(&mut self, size: i32, direction: ShipDirection) -> Option<Ship> {
    let allow = self.ships.get(&size).unwrap() > &0;
    if allow == true {
      self.reduce_ships(&size);
      Some(Ship { size, direction })
    } else {
      None
    }
  }
}
