use std::fmt::{Display, Formatter};

#[derive(Clone, Eq, PartialEq, Debug)]
enum Player {
    Empty,
    X,
    O,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::X => {
                write!(f, "X")
            }
            Player::O => {
                write!(f, "O")
            }
            Player::Empty => {
                write!(f, " ")
            }
        }
    }
}

struct Board<'a>(&'a mut [Player], usize);

impl<'a> Board<'a> {
    fn new(size: usize) -> Board<'a> {
        Self(vec![Player::Empty; size * size].leak(), size)
    }

    fn set_player(&mut self, player: Player, place: usize) {
        self.0[place] = player;
    }

    fn get_winner(&self) -> Option<Player> {
        for col in (0..self.0.len()).step_by(self.1) {
            let mut row = self.0.iter().enumerate().filter_map(|(i, p)| {
                if i >= col && i < col + self.1 {
                    Some(p)
                } else {
                    None
                }
            });
            let mut col = self.0.iter().enumerate().filter_map(|(i, p)| {
                if i % self.1 == col / self.1 {
                    Some(p)
                } else {
                    None
                }
            });

            if row.all(|p| p == &Player::X) {
                return Some(Player::X);
            }
            if row.all(|p| p == &Player::O) {
                return Some(Player::O);
            }
            if col.all(|p| p == &Player::X) {
                return Some(Player::X);
            }
            if col.all(|p| p == &Player::O) {
                return Some(Player::O);
            }
        }

        let mut dig1 = self.0.iter().enumerate().filter_map(|(i, p)| {
            if i % (self.1 + 1) == 0 {
                Some(p)
            } else {
                None
            }
        });
        let mut dig2 = self.0[1..(self.0.len() - 1)]
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                if i % (self.1 - 1) == self.1 - 2 {
                    Some(p)
                } else {
                    None
                }
            });

        if dig1.all(|p| p == &Player::X) {
            return Some(Player::X);
        }
        if dig1.all(|p| p == &Player::O) {
            return Some(Player::O);
        }
        if dig2.all(|p| p == &Player::X) {
            return Some(Player::X);
        }
        if dig2.all(|p| p == &Player::O) {
            return Some(Player::O);
        }

        None
    }
}

impl<'a> Default for Board<'a> {
    fn default() -> Self {
        Self::new(3)
    }
}

impl<'a> Display for Board<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}+", "+---".repeat(self.1))?;
        for row in self.0.chunks(self.1) {
            let row_string = row
                .iter()
                .fold(String::new(), |acc, p| format!("{acc}| {p} "))
                + "|";
            writeln!(f, "{row_string}")?;
            writeln!(f, "{}+", "+---".repeat(self.1))?;
        }

        Ok(())
    }
}

fn main() {
    let mut b = Board::new(3);
    b.set_player(Player::O, 4);
    b.set_player(Player::O, 8);
    // b.set_player(Player::O, 12);
    // b.set_player(Player::O, 16);
    // b.set_player(Player::O, 20);

    dbg!(b.get_winner());

    println!("{}", b);
}
