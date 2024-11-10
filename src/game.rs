use crate::player::Player;
use std::io;

#[derive(Debug)]
pub enum GameResult {
    Win(Player),
    Draw,
    NotOver,
}

pub fn get_input() -> Result<Option<(usize, usize)>, String> {
    let mut input = String::new();
    println!("Enter your move (row && column):");

    io::stdin()
        .read_line(&mut input)
        .map_err(|_| "Failed to read input".to_string())?;

    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    if parts.len() != 2 {
        return Err("Invalid input. Please enter two numbers.".to_string());
    }

    let row: usize = parts[0].parse().map_err(|_| "Invalid number".to_string())?;
    let col: usize = parts[1].parse().map_err(|_| "Invalid number".to_string())?;

    if row >= 3 || col >= 3 {
        return Err("Invalid position. Must be between 0 and 2".to_string());
    }

    Ok(Some((row, col)))
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
