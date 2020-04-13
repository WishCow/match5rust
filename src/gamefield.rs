use std::fmt;

use crate::player::Player;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Field {
    EMPTY,
    OWNED(Rc<Player>),
}

impl Default for Field {
    fn default() -> Self {
        Field::EMPTY
    }
}

#[derive(Debug)]
pub struct Point(usize, usize);

impl Point {
    pub fn to_index(&self, row_size: usize) -> usize {
        (self.1 * row_size) + self.0
    }

    pub fn new(x: usize, y: usize) -> Self {
        Point(x, y)
    }

    pub fn from_index(i: usize, row_size: usize) -> Self {
        let x = i % row_size;
        let y = i / row_size;
        Self::new(x, y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x: {}, y: {})", self.0, self.1)
    }
}

pub struct GameField {
    pub fields: Vec<Field>,
    pub row_size: usize,
    pub column_size: usize,
    win_count: usize,
}

#[derive(Debug)]
enum MatchDirection {
    Horizontal,
    Vertical,
    DiagonalRight,
    DiagonalLeft,
}

impl Default for GameField {
    fn default() -> Self {
        Self::new(3, 3, 3)
    }
}

impl GameField {
    pub fn new(row_size: usize, column_size: usize, win_count: usize) -> GameField {
        GameField {
            fields: (0..row_size * column_size).map(|_| Field::EMPTY).collect(),
            row_size,
            column_size,
            win_count,
        }
    }

    fn by_point(&self, point: &Point) -> Result<&Field, String> {
        let Point(x, y) = *point;
        if x > self.row_size - 1 {
            return Err(format!(
                "The X coordinate {} is out of bounds (max is {})",
                x,
                self.row_size - 1
            ));
        }
        if y > self.column_size - 1 {
            return Err(format!(
                "The Y coordinate {} is out of bounds (max is: {})",
                y,
                self.column_size - 1
            ));
        }

        Ok(self
            .fields
            .get(point.to_index(self.row_size))
            .expect("This should not happen"))
    }

    pub fn mark(&mut self, player: Rc<Player>, point: Point) -> Result<(), String> {
        let i: usize = point.to_index(self.row_size);
        let maybe_field = self.fields.get(i);
        if maybe_field.is_none() {
            return Err(format!("Field {} is out of bounds", i));
        }

        let field = maybe_field.unwrap();

        if let Field::OWNED(_) = field {
            return Err(String::from("This field is already taken"));
        }
        self.fields[i] = Field::OWNED(player);
        Ok(())
    }

    fn inspect(&self, from: &Point) -> Option<Rc<Player>> {
        let Point(x, y) = *from;
        let mut verify: Vec<MatchDirection> = vec![];

        let has_right = x + self.win_count <= self.row_size;
        let has_down = y + self.win_count <= self.column_size;
        let has_left = x as isize - self.win_count as isize >= -1;

        if has_right {
            verify.push(MatchDirection::Horizontal);
        }

        if has_down {
            verify.push(MatchDirection::Vertical);
        }

        if has_down && has_right {
            verify.push(MatchDirection::DiagonalRight);
        }

        if has_down && has_left {
            verify.push(MatchDirection::DiagonalLeft);
        }

        let tohash = |coords: &Vec<(usize, usize)>| -> HashMap<&Field, usize> {
            let mut hash = HashMap::new();
            coords
                .iter()
                .map(|(x, y)| self.by_point(&Point(*x, *y)))
                .filter_map(Result::ok)
                .for_each(|f| *hash.entry(f).or_insert(0) += 1);
            hash
        };

        let v: Vec<Vec<(usize, usize)>> = verify
            .iter()
            .map(|v| {
                (0..self.win_count)
                    .map(|c| match v {
                        MatchDirection::Horizontal => (x + c, y),
                        MatchDirection::Vertical => (x, y + c),
                        MatchDirection::DiagonalRight => (x + c, y + c),
                        MatchDirection::DiagonalLeft => (x - c, y + c),
                    })
                    .collect()
            })
            .collect();

        v.iter().find_map(|coord_set| {
            let hash = tohash(coord_set);
            for (k, v) in hash {
                if v == self.win_count {
                    if let Field::OWNED(what) = k {
                        return Some(what.clone());
                    }
                }
            }
            None
        })
    }

    pub fn state(&self) -> GameState {
        let mut non_empties = 0;
        for y in 0..self.column_size {
            for x in 0..self.row_size {
                let point = Point(x, y);
                match self.by_point(&point) {
                    Ok(Field::EMPTY) => {}
                    _ => non_empties += 1,
                }
                match self.inspect(&point) {
                    None => continue,
                    Some(player) => {
                        return GameState::WON(player);
                    }
                }
            }
        }
        if non_empties == self.fields.len() {
            GameState::TIE
        } else {
            GameState::OPEN
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum GameState {
    TIE,
    OPEN,
    WON(Rc<Player>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_to_index() {
        let test_values: Vec<(usize, usize, usize)> = vec![(0, 0, 0), (0, 1, 3), (1, 0, 1)];
        for i in test_values.into_iter() {
            let (x, y, expected) = i;
            let point = Point(x, y);
            let actual = point.to_index(3);
            assert!(
                actual == expected,
                "Failed for ({}, {}) -> {} but got {}",
                x,
                y,
                expected,
                actual
            );
        }
    }

    #[test]
    fn invalid_coordinates_should_err() {
        let game_field: GameField = Default::default();

        {
            let point = Point::new(0, 99);
            let result = game_field.by_point(&point);
            assert!(result.is_err());
        }
        {
            let point = Point::new(5, 0);
            let result = game_field.by_point(&point);
            assert!(result.is_err());
        }
    }

    #[test]
    fn marking_same_point_twice_should_err() {
        let player = Rc::new(Player::new("Hello", 'H'));
        let mut game_field: GameField = Default::default();
        assert!(game_field.mark(player.clone(), Point::new(0, 0)).is_ok());
        assert!(game_field.mark(player.clone(), Point::new(0, 0)).is_err());
    }

    #[test]
    fn rows_columns_diagonals_should_win() {
        let testdata = [
            [(0, 0), (1, 0), (2, 0)],
            [(0, 0), (0, 1), (0, 2)],
            [(0, 0), (1, 1), (2, 2)],
            [(0, 2), (1, 1), (2, 0)],
        ];
        for testrun in testdata.iter() {
            let player = Rc::new(Player::new("Hello", 'H'));
            let mut game_field: GameField = Default::default();
            for coords in testrun {
                game_field
                    .mark(player.clone(), Point::new(coords.0, coords.1))
                    .expect("Could not mark a location?");
            }
            match game_field.state() {
                GameState::WON(wonby) => {
                    assert_eq!(player.sign, wonby.sign);
                }
                _ => {
                    panic!("Game is not in a won state for coords {:?}", testrun);
                }
            }
        }
    }

    #[test]
    fn marking_all_fields_should_tie() {
        let players: Vec<_> = [("Hello", 'X'), ("Bye", 'O')]
            .iter()
            .map(|(name, sign)| Player::new(name, *sign))
            .map(Rc::new)
            .collect();

        #[rustfmt::skip]
        let marks = [
            'O', 'X', 'O',
            'X', 'X', 'O',
            'O', 'O', 'X'
        ];
        let mut game_field: GameField = Default::default();
        for (i, char) in marks.iter().enumerate() {
            let player = players.iter().find(|p| p.sign == *char).unwrap();
            game_field
                .mark(player.clone(), Point::from_index(i, 3))
                .expect("Could not mark a location?");
        }
        assert_eq!(game_field.state(), GameState::TIE);
    }
}
