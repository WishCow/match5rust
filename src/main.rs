use std::io;

mod asciiui;
mod game;
mod gamefield;
mod player;
use asciiui::AsciiUI;
use game::*;
use gamefield::*;

extern crate termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

#[derive(Debug, PartialEq, Eq)]
pub enum Input {
    INVALID,
    QUIT,
    MARK,
    LEFT,
    RIGHT,
    DOWN,
    UP,
    NOOP,
}

fn termion_key_to_input(key: Option<Result<termion::event::Key, std::io::Error>>) -> Input {
    match key {
        Some(Err(e)) => panic!("Cannot read from stdio: {}", e),
        Some(Ok(key)) => match key {
            termion::event::Key::Char('q') => Input::QUIT,
            termion::event::Key::Char(' ') => Input::MARK,
            termion::event::Key::Char('h') => Input::LEFT,
            termion::event::Key::Char('j') => Input::DOWN,
            termion::event::Key::Char('k') => Input::UP,
            termion::event::Key::Char('l') => Input::RIGHT,
            _ => Input::NOOP,
        },
        _ => Input::NOOP,
    }
}

fn main() {
    let players = vec![('X', "Red"), ('O', "Green")];
    let gamefield: GameField = GameField::new(5, 5, 5);
    let mut game = Game::new(players, gamefield);
    let mut ui = AsciiUI::new();
    let _stdout = io::stdout().into_raw_mode();
    let mut stdin = io::stdin().keys();
    while !game.is_finished() {
        println!("{}", ui.draw(&game));
        let entered = stdin.next();
        let input = termion_key_to_input(entered);
        match input {
            Input::INVALID => println!("Invalid input"),
            Input::QUIT => game.quit(),
            Input::UP | Input::DOWN | Input::LEFT | Input::RIGHT => ui.move_(&game, input),
            Input::MARK => {
                let result = game.mark_by_index(ui.considering);
                match result {
                    Err(error) => {
                        println!("Invalid input: {}", error);
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
            Input::NOOP => {}
        }
    }
}
