mod field;
mod game;
mod mediator;
mod player;
mod structures;
mod utils;
use mediator::Mediator;
use player::Player;
use structures::Point;
use utils::random_number;

fn main() {
 let mut mediator = Mediator::new();
       let point = Point { row: 2, column: 3 };
      mediator.human_move();
      println!("Game start 1");
       mediator.human.enemy_field.show();
      mediator.ai.game_field.show()
}

#[cfg(test)]
mod test;
