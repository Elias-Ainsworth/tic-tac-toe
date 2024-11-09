use std::{fmt, io};

#[derive(Debug)]
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
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
            Player::Empty => write!(f, "_"),
        }
    }
}

#[derive(Debug)]
pub enum GameResult {
    Win(Player),
    Draw,
    NotOver,
}

pub fn get_input() {
    todo!()
}

pub fn print_board(board: &Vec<Vec<Option<bool>>>) {
    for row in board {
        for cell in row {
            match cell {
                Some(true) => print!("{}", Player::X),
                Some(false) => print!("{}", Player::O),
                None => print!("_"),
            }
        }
        println!();

        if board.iter().next().unwrap() != row {
            println!("-------------");
        }
    }
}

pub fn get_winner(board: &Vec<Vec<Option<bool>>>) -> GameResult {
    let mut winner: Option<bool> = None;
    assert_eq!(board.len(), 3);

    for row in board {
        // check rows
        if row[0] == row[1] && row[1] == row[2] && row[0].is_some() {
            winner = row[0];
        }
    }

    for i in 0..3_usize {
        // check columns
        if &board[0][i] == &board[1][i] && &board[1][i] == &board[2][i] && board[0][i].is_some() {
            winner = board[0][i];
        }
        // check diagonals
        if &board[0][0] == &board[1][1] && &board[1][1] == &board[2][2] && board[0][0].is_some() {
            winner = board[0][0];
        }
        if &board[0][2] == &board[1][1] && &board[1][1] == &board[2][0] && board[0][2].is_some() {
            winner = board[0][2];
        }
    }

    if winner.is_none() {
        if board.iter().flatten().all(|&cell| cell.is_some()) {
            GameResult::Draw
        } else {
            GameResult::NotOver
        }
    } else {
        let winner = match winner {
            Some(true) => Some(Player::X),
            Some(false) => Some(Player::O),
            None => Some(Player::Empty),
        };
        GameResult::Win(winner.unwrap())
    }
}

fn main() {
    // for row in &board {
    //     for cell in row {
    //         match cell {
    //             Some(true) => print!("{} ", Player::X.char()),
    //             Some(false) => print!("{} ", Player::O.char()),
    //             None => print!("_ "),
    //         }
    //     }
    //     println!();
    // }
    // match get_winner(&board) {
    //     GameResult::Win(player) => println!("Winner: {:#?}", player),
    //     GameResult::Draw => println!("Draw"),
    //     GameResult::NotOver => println!(),
    // }
}
