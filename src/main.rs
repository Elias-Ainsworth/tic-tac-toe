pub enum Player {
    X = 1,
    O = 2,
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
pub fn get_winner(board: &mut Vec<Vec<Option<bool>>>) {}
fn main() {
    let mut board = vec![vec![None; 3]; 3];
    board[0][0] = Some(true);
    board[1][1] = Some(false);
    board[2][2] = None;
    for row in &board {
        for cell in row {
            match cell {
                Some(true) => print!("{} ", Player::X.char()),
                Some(false) => print!("{} ", Player::O.char()),
                None => print!("_ "),
            }
        }
        println!();
    }
}
