use std::io::{stdin, stdout, Write};

//https://github.com/athre0z/color-backtrace
pub enum Player {
    AI,
    Human,
}
#[derive(PartialEq)]
pub enum GameState {
    Pending,
    Play,
    Finish,
}

pub struct GameLoop {
    turn: Player,
    state: GameState,
}

impl GameLoop {
    pub fn start() {
        let mut x = 10;
        while x > 5 {
            println!("Your turn");
            if x % 2 == 0 {
                let mut s = String::new();
                print!("Please enter some text: ");
                let _ = stdout().flush();
                stdin()
                    .read_line(&mut s)
                    .expect("Did not enter a correct string");
            }
            x -= 1;
        }
    }
}
