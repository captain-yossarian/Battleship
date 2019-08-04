use crate::field;
use crate::structures;
use field::GameField;
use structures::{Move, Point, RandomNumber, Status};

pub struct Player {
    pub own_field: GameField,
    pub enemy_field: GameField,
}
impl Player {
    pub fn new(randomizer: RandomNumber) -> Player {
        Player {
            own_field:GameField::new(randomizer),
            enemy_field:GameField::new(randomizer),
        }
    }
    pub fn init(&mut self) {
        self.own_field.generate_random_field();
    }
    pub fn enemy_attack(&mut self, point: Point) -> Move {
        match self.own_field.get_cell_value(point) {
            Status::Ship => {
                self.own_field.draw_cell(point, Status::Kill);
                self.own_field.sink_ship();
                Move::Kill(point)
            }
            _ => Move::Miss(point),
        }
    }
    pub fn player_move(&mut self, result: &Move) {
        match result {
            Move::Kill(point) => {
                self.enemy_field.draw_cell(*point, Status::Kill);
            }
            Move::Miss(point) => {
                self.enemy_field.draw_cell(*point, Status::Bound);
            }
        }
    }
}
