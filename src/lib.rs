pub mod board;
pub mod game;
pub mod player;

pub use board::{make_move, print_board};
pub use game::{get_input, get_winner, GameResult};
pub use player::Player;
