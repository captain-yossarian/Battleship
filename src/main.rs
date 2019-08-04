mod field;
mod game;
mod mediator;
mod player;
mod structures;
mod utils;
use field::GameField;
use mediator::Mediator;
use player::Player;
use structures::Status;
use utils::{random_number, status_u8};

fn point_sum(field: &GameField, status: Status) -> u8 {
      field.field.iter().flatten().fold(0, |acc, elem| {
            if *elem == status {
                  acc + status_u8(*elem)
            } else {
                  acc
            }
      })
}

fn all_filled(field: &GameField) -> usize {
      field.field
            .iter()
            .flatten()
            .filter(|elem| status_u8(Status::Kill) == status_u8(**elem))
            .collect::<Vec<&Status>>()
            .len()
}

fn main() {
      let mut mediator = Mediator::new();
      mediator.ai.init();
      mediator.human.init();
      loop {
            &mediator.human_move();
            &mediator.ai_move();
            println!("AI, enemy field {:?} ", all_filled(&mediator.ai.enemy_field));
            if all_filled(&mediator.ai.enemy_field) == 20 {
                  mediator.ai.enemy_field.show();
                  mediator.human.own_field.show();
                  break;
            }
      }
}

#[cfg(test)]
mod test;
