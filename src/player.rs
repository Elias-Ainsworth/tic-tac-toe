use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
    Empty,
}

impl Player {
    pub fn others(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
            Player::Empty => Player::Empty,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Player::X => write!(f, "X "),
            Player::O => write!(f, "O "),
            Player::Empty => write!(f, "_"),
        }
    }
}
