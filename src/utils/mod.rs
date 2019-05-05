use crate::field::{Field, Point, Status};
use rand::{thread_rng, Rng};

pub fn random_number(bottom: u8, up: u8) -> u8 {
  let mut random = thread_rng();
  random.gen_range(bottom, up)
}

pub fn generate_all_empty_points(field: Field) -> Vec<Point> {
  let mut vec: Vec<Point> = Vec::new();
  field[1..11]
    .iter()
    .enumerate()
    .for_each(|(row_index, row)| {
      row[1..11]
        .iter()
        .enumerate()
        .for_each(|(column_index, value)| match value {
          Status::Empty => {
            let point = Point {
              row: (row_index as u8) + 1,
              column: (column_index as u8) + 1,
            };
            vec.push(point);
          }
          _ => (),
        })
    });
  vec
}

