use crate::gamefield::*;
use crate::player::*;
use std::rc::Rc;

pub struct Game {
    players: Vec<Rc<Player>>,
    fields: GameField,
    turn: usize,
    is_finished: bool,
}

impl Game {
    pub fn new(players: Vec<(char, &str)>, fields: GameField) -> Game {
        let players: Vec<_> = players
            .into_iter()
            .map(|(sign, name)| Player::new(name, sign))
            .map(Rc::new)
            .collect();

        Game {
            players,
            fields,
            turn: 0,
            is_finished: false,
        }
    }
    pub fn draw(&self) {
        print!("{}[2J", 27 as char);
        let row_size = self.fields.row_size;

        let max = self.players.iter().map(|p| p.name.len()).max().unwrap();

        println!("{}", "-".repeat(max + 8));
        self.players
            .iter()
            .for_each(|p| println!("| {:>width$} | {} |", p.name, p.sign, width = max));
        println!("{}\n", "-".repeat(max + 8));

        println!("Current player is: {}\n", self.players[self.turn].sign);

        let signs: Vec<String> = self.fields.get().map(|f| f.draw()).collect();

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

    pub fn mark(&mut self, point: Point) -> Result<(), String> {
        let player = self.players[self.turn].clone();
        let result = self.fields.mark(player, point);
        if result.is_ok() {
            let next = self
                .players
                .get(self.turn + 1)
                .map(|_| self.turn + 1)
                .unwrap_or(0);
            self.turn = next;
        }
        result
    }

    pub fn mark_by_index(&mut self, i: usize) -> Result<(), String> {
        self.mark(Point::from_index(i, self.fields.row_size))
    }

    pub fn state(&self) -> GameState {
        self.fields.state()
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn quit(&mut self) {
        self.is_finished = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_win_game() {
        let players = vec![('X', "P1"), ('Y', "P2")];
        let fields = Default::default();
        let mut game = Game::new(players, fields);
        for (x, y) in [(0, 0), (1, 1), (1, 0), (2, 1), (2, 0)].iter() {
            assert_eq!(game.mark(Point::new(*x, *y)), Ok(()));
        }
        assert_eq!(
            game.fields.state(),
            GameState::WON(Rc::new(Player::new("P1", 'X')))
        );
    }
}
