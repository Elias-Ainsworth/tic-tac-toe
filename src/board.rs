use crate::player::Player;

pub fn make_move(
    board: &mut [Vec<Option<bool>>],
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

pub fn print_board(board: &[Vec<Option<bool>>]) {
    println!("\n    0   1   2");
    println!("  ┌───┬───┬───┐");
    for (i, row) in board.iter().enumerate() {
        print!("{} │", i); // Print row index
        for cell in row {
            match cell {
                Some(true) => print!(" X │"),
                Some(false) => print!(" O │"),
                None => print!("   │"),
            }
        }
        println!();
        if i < 2 {
            println!("  ├───┼───┼───┤");
        }
    }
    println!("  └───┴───┴───┘\n");
}
