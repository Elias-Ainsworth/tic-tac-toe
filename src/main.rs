use tic_tac_toe::{get_input, get_winner, make_move, print_board, GameResult, Player};

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
