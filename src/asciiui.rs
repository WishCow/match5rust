use crate::game::Game;
use crate::gamefield::Field;
extern crate termion;

use crate::Input;
use termion::color;

pub struct AsciiUI {
    pub considering: usize,
}

impl AsciiUI {
    pub fn new() -> Self {
        Self { considering: 0 }
    }

    pub fn move_(&mut self, game: &Game, input: Input) {
        let total_fields = game.fields.fields.len() as i8;
        let field_len = game.fields.row_size as i8;
        let current_row = self.considering as i8 / field_len;

        let change = match input {
            n @ Input::RIGHT | n @ Input::LEFT => {
                let delta = if n == Input::RIGHT { 1 } else { -1 };
                let maybe = self.considering as i8 + delta;
                let new_row = maybe / field_len;
                if maybe < 0 {
                    field_len - 1
                } else if new_row != current_row {
                    maybe
                        + if new_row > current_row {
                            -field_len
                        } else {
                            field_len
                        }
                } else {
                    maybe
                }
            }
            n @ Input::UP | n @ Input::DOWN => {
                let delta = field_len * if n == Input::DOWN { 1 } else { -1 };
                let maybe = self.considering as i8 + delta;
                if maybe < 0 {
                    total_fields + maybe
                } else if maybe > total_fields {
                    maybe - total_fields
                } else {
                    maybe
                }
            }
            _ => self.considering as i8,
        } as usize;
        self.considering = change;
    }

    pub fn draw(&self, game: &Game) -> String {
        let mut draw: Vec<_> = vec![format!("{}[2J", 27 as char)];
        let row_size = game.fields.row_size;

        let max = game.players.iter().map(|p| p.name.len()).max().unwrap();

        draw.push("-".repeat(max + 8));
        draw.extend(
            game.players
                .iter()
                .map(|p| format!("| {:>width$} | {} |", p.name, p.sign, width = max)),
        );
        draw.push(format!("{}\n", "-".repeat(max + 8)));

        draw.push(format!(
            "Current player is: {}\n",
            game.players[game.turn].sign
        ));

        let rows = game.fields.fields[..].chunks(game.fields.row_size);

        draw.push(format!("┌{}┐", "─".repeat((row_size * 2) - 1)));
        for (i, row) in rows.enumerate() {
            let signs: Vec<String> = row
                .iter()
                .enumerate()
                .map(|(j, f)| self.draw_field(f, (i * row_size) + j == self.considering))
                .collect();
            draw.push(format!("│{}│", signs.join("│")));
            draw.push(format!("├{}┤", "─".repeat((row_size * 2) - 1)));
        }
        draw.pop();
        draw.push(format!("└{}┘", "─".repeat((row_size * 2) - 1)));
        draw.join("\r\n")
    }

    fn draw_field(&self, field: &Field, is_considering: bool) -> String {
        match field {
            Field::EMPTY if is_considering => {
                format!("{}?{}", color::Fg(color::Red), color::Fg(color::Reset))
            }
            Field::EMPTY => " ".to_string(),
            Field::OWNED(player) if is_considering => format!(
                "{}{}{}",
                color::Fg(color::Red),
                player.sign.to_string(),
                color::Fg(color::Reset)
            ),
            Field::OWNED(player) => player.sign.to_string(),
        }
    }
}
