use std::io;

fn main() {
    let mut board = [' '; 9];

    let players = ['X', 'O'];

    let mut turn = 0;

    loop {
        print!("Enter position for {}", players[turn]);

        let index = get_index_from_input();

        if let Err(e) = index {
            println!("{e}");

            continue;
        }

        let index = index.unwrap();

        if let None = index {
            break;
        }

        let index = index.unwrap();

        if board[index] != ' ' {
            println!("The cell at positon {} is already occupied", index + 1);

            continue;
        }

        board[index] = players[turn];

        print_board(board);

        turn = (turn + 1) % 2;
    }
}

fn print_board(board: [char; 9]) {
    println!(
        "
                +---+---+---+
                | {} | {} | {} |
                +---+---+---+
                | {} | {} | {} |
                +---+---+---+
                | {} | {} | {} |
                +---+---+---+
        ",
        board[0], board[1], board[2], board[3], board[4], board[5], board[6], board[7], board[8]
    );
}

fn get_index_from_input() -> Result<Option<usize>, String> {
    let mut input = String::new();

    let _ = io::stdin()
        .read_line(&mut input)
        .map_err(|e| e.to_string())?;

    let input = input.trim();

    if input == ":q" {
        return Ok(None);
    }

    let index = input
        .parse::<usize>()
        .map_err(|_| format!("Input should be an integer."))?;

    if index < 1 || index > 9 {
        return Err(format!("The position should be an integer from 1 to 9."));
    }

    return Ok(Some(index - 1));
}
