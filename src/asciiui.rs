use crate::game::Game;
extern crate termion;

pub struct AsciiUI {
    considering: usize
}

impl AsciiUI {

    pub fn new() -> Self {
        Self { considering: 0 }
    }

    pub fn draw(&self, game: &Game) {
        print!("{}[2J", 27 as char);
        let row_size = game.fields.row_size;

        let max = game.players.iter().map(|p| p.name.len()).max().unwrap();

        println!("{}", "-".repeat(max + 8));
        game.players
            .iter()
            .for_each(|p| println!("| {:>width$} | {} |", p.name, p.sign, width = max));
        println!("{}\n", "-".repeat(max + 8));

        println!("Current player is: {}\n", game.players[game.turn].sign);

        let signs: Vec<String> = game.fields.get().map(|f| f.draw()).collect();

        let mut duh: Vec<Vec<String>> = Default::default();

        for i in 0..row_size {
            duh.push(signs[i * (row_size)..i * row_size + (row_size)].to_owned());
        }

        let mut draw: Vec<String> = vec![];
        draw.push(format!("┌{}┐","─".repeat((row_size * 2) - 1).to_string()));
        for v in duh {
            draw.push(format!("│{}│", v.join("│")));
            draw.push(format!("├{}┤", "─".repeat((row_size * 2) - 1)));
        }
        draw.pop();
        draw.push(format!("└{}┘", '─'.to_string().repeat((row_size * 2) - 1)));
        println!("{}", draw.join("\n"));
    }

}
