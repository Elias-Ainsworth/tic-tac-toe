pub enum Player {
    X,
    O,
}
impl Player {
    pub fn char(&self) -> char {
        match self {
            Player::X => 'X',
            Player::O => 'O',
        }
    }
    pub fn others(&self) -> Self {
        match self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}
pub fn get_winner(board: &Vec<Vec<Option<bool>>>) -> Option<bool> {
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
    None
}

fn main() {
    let mut board = vec![vec![None; 3]; 3];
    board[0][0] = Some(true);
    board[1][1] = Some(true);
    board[2][2] = Some(true);
    let winner = if let Some(winner) = get_winner(&board) {
        if winner == true {
            return Some(Player::X.char());
        } else if winner == false {
            return Some(Player::O.char());
        }
    }
    for row in &board {
        for cell in row {
            match cell {
                Some(true) => print!("{} ", Player::X.char()),
                Some(false) => print!("{} ", Player::O.char()),
                None => print!("_ "),
            }
        }
        println!();
        println!("Winner: {:#?}", winner);
    }
}
