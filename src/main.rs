mod field;
mod mediator;
mod player;
mod structures;
mod utils;
use mediator::Mediator;
use utils::{ALL_SHIPS};

fn main() {

      let mut mediator = Mediator::new();

      loop {
            mediator.human_move();
            mediator.ai_move();

            if mediator.human.own_field.sunked_ships == ALL_SHIPS {
                  println!("AI wins!");
                  break;
            }
            if mediator.ai.own_field.sunked_ships == ALL_SHIPS {
                  println!("Player wins!");
                  break;
            }
      }
}

#[cfg(test)]
mod test;
