use crate::gamefield::*;
use crate::player::*;
use std::rc::Rc;

pub struct Game {
    pub players: Vec<Rc<Player>>,
    pub fields: GameField,
    pub turn: usize,
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
            is_finished: false
        }
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
