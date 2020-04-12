use std::io;
use std::io::prelude::*;

mod asciiui;
mod game;
mod gamefield;
mod player;
use game::*;
use gamefield::*;
use asciiui::AsciiUI;

#[derive(Debug, PartialEq, Eq)]
enum Input {
    INVALID,
    QUIT,
    MARK(usize),
}

impl Input {
    fn from_string(input: &str) -> Self {
        let input = input.trim();
        if input == "q" {
            return Self::QUIT;
        }
        let m = input.parse::<usize>();
        match m {
            Ok(val) => Self::MARK(val),
            Err(_e) => Self::INVALID,
        }
    }
}

fn main() {
    let players = vec![('X', "Red"), ('O', "Green")];
    let gamefield: GameField = GameField::new(10, 10, 5);
    let mut game = Game::new(players, gamefield);
    let ui = AsciiUI::new();
    io::stdout().flush().expect("Could not flush input buffer");
    while !game.is_finished() {
        let mut entered = String::new();
        ui.draw(&game);
        let stdin = std::io::stdin();
        stdin.read_line(&mut entered).expect("Could not read stdin");
        let input = Input::from_string(&entered);
        match input {
            Input::INVALID => println!("Invalid input: {}", &entered),
            Input::QUIT => game.quit(),
            Input::MARK(i) => {
                let result = game.mark_by_index(i);
                match result {
                    Err(error) => {
                        println!("Invalid input: {}", error);
                        let mut discard = String::new();
                        stdin.read_line(&mut discard).expect("Failed to flush io");
                        continue;
                    }
                    Ok(_) => match game.state() {
                        GameState::WON(player) => {
                            println!("Game won by {}!", player.sign.to_string());
                            game.quit();
                        }
                        GameState::TIE => println!("Game is a tie"),
                        _ => (),
                    },
                }
            }
        }
    }
}
