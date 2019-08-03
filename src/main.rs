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
      mediator.ai.init();
      mediator.human.init();
      mediator.human.enemy_field.show();
      mediator.ai.own_field.show();
      mediator.human_move();
         mediator.human.enemy_field.show();
      mediator.ai.own_field.show();
}

#[cfg(test)]
mod test;
