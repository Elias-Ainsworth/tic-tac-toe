use std::io;

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

pub fn get_winner(board: &[char; 9]) -> Option<Player> {
    let winning_combinations: [[usize; 3]; 8] = [
        [0, 1, 2],
        [3, 4, 5],
        [6, 7, 8],
        [0, 3, 6],
        [1, 4, 7],
        [2, 5, 8],
        [0, 4, 8],
        [2, 4, 6],
    ];
    let x = Player::X.char();

    let o = Player::O.char();

    for combo in winning_combinations {
        let [a, b, c] = combo;

        if board[a] != ' ' && board[a] == board[b] && board[b] == board[c] {
            return match board[a] {
                x => Some(Player::X),
                o => Some(Player::O),
                _ => None,
            };
        }
    }
    None
}

fn main() {
    let mut board = [' '; 9];

    print_board(board);
    let mut player = Player::X;
    loop {
        println!("Enter position for {}", player.char());

        let index = get_index_from_input();

        if let Err(e) = index {
            println!("{e}");

            continue;
        }

        let index = index.unwrap();

        if let None = index {
            break;
        }

        if let Some(index) = index {
            if board[index] != ' ' {
                println!("The cell at positon {} is already occupied", index + 1);

                continue;
            }

            board[index] = player.char();

            print_board(board);

            if let Some(winner) = get_winner(&board) {
                println!("Winner: {:?}", winner.char());
            }

            player = player.others();
        } else {
            break;
        }
    }
    todo!("Check for winner using get_winner function.")
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
