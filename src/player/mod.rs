use crate::field;
use crate::structures;
use field::GameField;
use structures::{Move, Point, RandomNumber, Status};

pub struct Player {
   pub game_field: GameField,
   pub enemy_field: GameField,
}
impl Player {
    pub fn new(randomizer: RandomNumber) -> Player {
        let mut game_field = GameField::new(randomizer);
        game_field.generate_random_field();
        let enemy_field = GameField::new(randomizer);

        Player {
            game_field,
            enemy_field,
        }
    }
    pub fn enemy_move(&mut self, point: Point) -> Move {
        match self.game_field.get_cell_value(point) {
            Status::Ship => {
                self.game_field.draw_cell(point, Status::Kill);
                Move::Kill(point)
            }
            _ => Move::Miss(point),
        }
    }
    pub fn player_move(&mut self, result:Move){
        match result {
            Move::Kill(point) => {
                self.enemy_field.draw_cell(point, Status::Kill);
            }
            Move::Miss(point)=>{
                self.enemy_field.draw_cell(point, Status::Bound);
            }
        }
    }
}
