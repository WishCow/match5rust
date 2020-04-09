use std::cmp::PartialEq;

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Player {
    pub name: String,
    pub sign: char,
}

impl Player {
    pub fn new(name: &str, sign: char) -> Self {
        Self {
            name: String::from(name),
            sign,
        }
    }
}

impl PartialEq<char> for Player {
    fn eq(&self, other: &char) -> bool {
        self.sign == *other
    }
}
