use super::*;
use field::GameField;
// use std::time::{Duration, SystemTime};
use structures::{Direction, Draw, Point, Ship, ShipDirection, Status};
use utils::{generate_all_empty_points, random_number, status_u8};

#[cfg(test)]

const ALL_SHIPS: u8 = 20;
fn point_sum(field: GameField, status: Status) -> u8 {
    field.field.iter().flatten().fold(0, |acc, elem| {
        if *elem == status {
            acc + status_u8(*elem)
        } else {
            acc
        }
    })
}

#[test]
fn create_field() {
    let GameField { field, .. } = GameField::new(random_number);
    assert_eq!(field, [[Status::Empty; 12]; 12]);
}
#[test]
fn get_ships() {
    let GameField { ships, .. } = GameField::new(random_number);
    let arr = [1, 2, 3, 4];
    let length = arr.len();
    for (i, elem) in arr.iter().enumerate() {
        let index = length - (1 + i);
        let to_be = &arr[index];
        assert_eq!(ships[elem], *to_be);
    }
}

#[test]
fn reduce_ships() {
    let mut field = GameField::new(random_number);
    for index in 1..5 {
        field.reduce_ships(index);
    }
    assert_eq!(field.ships[&1], 3);
    assert_eq!(field.ships[&2], 2);
    assert_eq!(field.ships[&3], 1);
    assert_eq!(field.ships[&4], 0);
}
#[test]
fn check_permission_positive() {
    let mut field = GameField::new(random_number);
    let permission = field.check_permission(4);
    assert_eq!(permission, true);
}
#[test]
fn check_permission_negative() {
    let mut field = GameField::new(random_number);
    field.reduce_ships(4);
    let permission = field.check_permission(4);
    assert_eq!(permission, false);
}
#[test]
fn test_generate_all_empty_points() {
    {
        let mut field = GameField::new(random_number);

        let size = 4;
        field.create_ship(
            size,
            &ShipDirection::Vertical,
            Some(Point { row: 5, column: 6 }),
        );
        let empty_points = generate_all_empty_points(field.field);
        println!("EMpty points {}", empty_points.len());
        field.show();
        assert_eq!(empty_points.len(), 82);
    }
    {
        let mut field = GameField::new(random_number);
        let size = 1;
        field.create_ship(
            size,
            &ShipDirection::Vertical,
            Some(Point { row: 5, column: 6 }),
        );
        let empty_points = generate_all_empty_points(field.field);
        println!("EMpty points {}", empty_points.len());
        field.show();
        assert_eq!(empty_points.len(), 91);
    }
    {
        let mut field = GameField::new(random_number);
        field.create_ship(
            4,
            &ShipDirection::Vertical,
            Some(Point { row: 5, column: 6 }),
        );
        field.create_ship(
            3,
            &ShipDirection::Vertical,
            Some(Point { row: 2, column: 8 }),
        );
        let empty_points = generate_all_empty_points(field.field);
        println!("EMpty points {}", empty_points.len());
        field.show();
        assert_eq!(empty_points.len(), 69);
    }
    {
        let mut field = GameField::new(random_number);
        field.create_ship(
            4,
            &ShipDirection::Vertical,
            Some(Point { row: 2, column: 1 }),
        );
        field.create_ship(
            3,
            &ShipDirection::Vertical,
            Some(Point { row: 2, column: 3 }),
        );
        field.create_ship(
            3,
            &ShipDirection::Vertical,
            Some(Point { row: 2, column: 5 }),
        );
        let empty_points = generate_all_empty_points(field.field);
        println!("EMpty points {}", empty_points.len());
        field.show();
        assert_eq!(empty_points.len(), 68);
    }
}
#[test]
fn generate_random_field() {
    for _ in 0..2000 {
        let mut field = GameField::new(random_number);
        field.generate_random_field();
        let sum = point_sum(field, Status::Ship);
        assert_eq!(sum, ALL_SHIPS);
    }
}
#[test]
fn random_point() {
    let mut field = GameField::new(random_number);
    for size in 1..5 {
        for _ in 0..100 {
            let Point { row, column } = field.generate_random_point(&ShipDirection::Vertical, size);

            let expect_row = row >= 1 && row < (12 - size);
            let expect_column = column >= 1 && column <= 10;

            assert_eq!(expect_row, true);
            assert_eq!(expect_column, true);
        }
    }
}
#[test]
fn scan_for() {
    let mut field = GameField::new(random_number);
    let size = 4;
    let point = Point { row: 5, column: 6 };
    field.create_ship(size, &ShipDirection::Vertical, Some(point));
    {
        let path = vec![(Direction::Down, 1)];
        assert_eq!(
            field.scan_for(&path, point, vec![Status::Empty, Status::Bound]),
            false
        );
    }
    {
        let path = vec![(Direction::Down, 1)];
        let point = Point { row: 5, column: 8 };
        assert_eq!(field.scan_for(&path, point, vec![Status::Empty]), true);
    }
    {
        let path = vec![(Direction::Down, 1)];
        let point = Point { row: 8, column: 6 };
        assert_eq!(field.scan_for(&path, point, vec![Status::Bound]), true);
    }
    {
        let path = vec![(Direction::Down, 1)];
        let point = Point { row: 9, column: 6 };
        assert_eq!(field.scan_for(&path, point, vec![Status::Empty]), true);
    }
}
#[test]
fn draw_cell() {
    let mut field = GameField::new(random_number);
    let points: Vec<(Point, Status)> = vec![
        (Point { row: 5, column: 5 }, Status::Ship),
        (Point { row: 8, column: 9 }, Status::Bound),
        (Point { row: 2, column: 4 }, Status::Empty),
        (Point { row: 4, column: 1 }, Status::Kill),
    ];
    points.iter().for_each(|(point, status)| {
        field.draw_cell(*point, *status);
        let cell = field.get_cell_value(*point);
        assert_eq!(cell, *status);
    })
}
#[test]
fn get_cell_value() {
    let mut field = GameField::new(random_number);
    let point = Point { row: 1, column: 1 };
    field.draw_cell(point, Status::Bound);
    let cell_value = field.get_cell_value(point);
    assert_eq!(cell_value, Status::Bound);
}
#[test]
fn draw_by_path() {
    let mut field = GameField::new(random_number);
    let path = vec![
        (Direction::Right, 1),
        (Direction::Right, 1),
        (Direction::Down, 1),
    ];
    let point = Point { row: 7, column: 6 };

    field.draw_by_path(Draw {
        start_point: point,
        path,
        draw_status: Status::Bound,
        allowed_status: vec![Status::Empty, Status::Bound],
    });
    let sum = point_sum(field, Status::Bound);
    assert_eq!(sum / 2, 3)
}
#[test]
fn draw_ship_core() {
    let mut field = GameField::new(random_number);
    let size = 4;
    let ship = Ship {
        direction: &ShipDirection::Horizontal,
        size,
        start_point: Point { row: 3, column: 5 },
    };
    let result = field.draw_ship_core(ship);
    let sum = point_sum(field, Status::Ship);
    assert_eq!(sum, size);
    assert_eq!(result, Some(()));
}
#[test]
fn draw_ship_core_performance() {
    let mut field = GameField::new(random_number);
    let size = 4;
    let ship = Ship {
        direction: &ShipDirection::Horizontal,
        size,
        start_point: Point { row: 3, column: 5 },
    };
    let result = field.draw_ship_core(ship);
    let sum = point_sum(field, Status::Ship);
    assert_eq!(sum, size);
    assert_eq!(result, Some(()));
}

#[test]
fn draw_ship_bounds() {
    let mut field = GameField::new(random_number);
    let size = 4;
    let bound_quantity = 14;
    let ship = Ship {
        direction: &ShipDirection::Horizontal,
        size,
        start_point: Point { row: 5, column: 5 },
    };
    field.draw_ship_bounds(ship);
    let sum = point_sum(field, Status::Bound);
    assert_eq!(sum / 2, bound_quantity)
}
#[test]
fn generate_ship_bounds() {
    let field = GameField::new(random_number);
    let size = 3;
    let bounds_path_horizontal = field.generate_ship_bounds(&ShipDirection::Horizontal, size);
    let bounds_path_vertical = field.generate_ship_bounds(&ShipDirection::Vertical, size);
    assert_eq!(bounds_path_horizontal.len(), 5);
    assert_eq!(bounds_path_vertical.len(), 5);
}
