use crate::structures::{Field, Point, Status};
use rand::{thread_rng, Rng};

pub const ALL_SHIPS: u8 = 20;


pub fn random_number(bottom: u8, up: u8) -> u8 {
    let mut random = thread_rng();
    random.gen_range(bottom, up)
}

pub fn generate_all_empty_points(field: &Field) -> Vec<Point> {
    let mut vec: Vec<Point> = Vec::new();
    field[1..11]
        .iter()
        .enumerate()
        .for_each(|(row_index, row)| {
            row[1..11]
                .iter()
                .enumerate()
                .for_each(|(column_index, value)| {
                    if let Status::Empty = value {
                        vec.push(Point {
                            row: (row_index as u8) + 1,
                            column: (column_index as u8) + 1,
                        });
                    }
                })
        });
    vec
}

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
