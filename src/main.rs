mod field;
mod mediator;
mod player;
mod structures;
mod utils;
use mediator::Mediator;

fn main() {
      let mut mediator = Mediator::new();

      loop {
            mediator.human_move();
            mediator.ai_move();

            if mediator.human.own_field.sunked_ships == 20 {
                  println!("AI wins!");
                  break;
            }
            if mediator.ai.own_field.sunked_ships == 20 {
                  println!("Player wins!");
                  break;
            }
      }
}

#[cfg(test)]
mod test;
