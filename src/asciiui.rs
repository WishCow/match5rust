use crate::game::Game;
use crate::gamefield::Field;
extern crate termion;

pub struct AsciiUI {
    considering: usize
}

impl AsciiUI {

    pub fn new() -> Self {
        Self { considering: 0 }
    }

    pub fn draw(&self, game: &Game) -> String {
        let mut draw: Vec<_> = vec![format!("{}[2J", 27 as char)];
        let row_size = game.fields.row_size;

        let max = game.players.iter().map(|p| p.name.len()).max().unwrap();

        draw.push("-".repeat(max + 8));
        draw.extend(game.players
            .iter()
            .map(|p| format!("| {:>width$} | {} |", p.name, p.sign, width = max))
        );
        draw.push(format!("{}\n", "-".repeat(max + 8)));

        draw.push(format!("Current player is: {}\n", game.players[game.turn].sign));

        let rows = game.fields.fields[..].chunks(game.fields.row_size);

        draw.push(format!("┌{}┐","─".repeat((row_size * 2) - 1).to_string()));
        for row in rows {
            let signs: Vec<String> = row.iter().map(|f| f.draw()).collect();
            draw.push(format!("│{}│", signs.join("│")));
            draw.push(format!("├{}┤", "─".repeat((row_size * 2) - 1)));
        }
        draw.pop();
        draw.push(format!("└{}┘", '─'.to_string().repeat((row_size * 2) - 1)));
        draw.join("\n")
    }

}
