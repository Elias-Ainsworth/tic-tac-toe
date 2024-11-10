use std::{fmt, io};

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

pub fn print_board(board: &Vec<Vec<Option<bool>>>) {
    // for (i, row) in board.iter().enumerate() {
    //     for (j, cell) in row.iter().enumerate() {
    //         match cell {
    //             Some(true) => print!(" {}", Player::X),
    //             Some(false) => print!(" {}", Player::O),
    //             None => print!(" {} ", Player::Empty),
    //         }

    //         // Print column separators
    //         if j < 2 {
    //             print!("|");
    //         }
    //     }
    //     println!();

    //     // Print row separators
    //     if i < 2 {
    //         println!("---+---+---");
    //     }
    // }
    // println!();
    //
    println!("\n  0   1   2");
    println!(" ┌───┬───┬───┐");
    for (i, row) in board.iter().enumerate() {
        print!("{}│", i); // Print row index
        for cell in row {
            match cell {
                Some(true) => print!(" X │"),
                Some(false) => print!(" O │"),
                None => print!("   │"),
            }
        }
        println!();
        if i < 2 {
            println!(" ├───┼───┼───┤");
        }
    }
    println!(" └───┴───┴───┘\n");
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

pub fn make_move(
    board: &mut Vec<Vec<Option<bool>>>,
    player: Player,
    row: usize,
    col: usize,
) -> Result<(), String> {
    if board[row][col].is_some() {
        return Err("Cell is already occupied!".to_string());
    }

    board[row][col] = Some(player == Player::X);
    Ok(())
}

fn main() {
    let mut board = vec![vec![None; 3]; 3];
    let mut current_player = Player::X;

    loop {
        print_board(&board);
        println!("Player {}'s turn.", current_player);

        let input = match get_input() {
            Ok(Some(pos)) => pos,
            Ok(None) => {
                println!("Invalid move try again");
                continue;
            }
            Err(e) => {
                println!("Error: {}", e);
                continue;
            }
        };

        if let Err(e) = make_move(&mut board, current_player, input.0, input.1) {
            println!("Error: {}", e);
            continue;
        }

        match get_winner(&board) {
            GameResult::Win(player) => {
                print_board(&board);
                println!("Player {} wins!", player);
                break;
            }
            GameResult::Draw => {
                print_board(&board);
                println!("It's a draw!");
                break;
            }
            GameResult::NotOver => {
                current_player = current_player.others();
            }
        }
        println!();
    }
}
