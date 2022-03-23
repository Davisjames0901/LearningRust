use console::Term;
use std::fmt;

pub fn main() {
    println!("Welcome to Tic-Tac-Toe!\n");
    let mut game = get_new_game_board();
    let mut game_won = false;
    while !game_won {
        //go to next player
        game.current_player = flip(&game);
        // this prints the game state using the impl for fmt::Display (seen below)
        println!("{}", game);
        //Get the move and set the board (That is why the game is passed with the mut flag)
        make_move(&mut game);
        //check to see if we win
        game_won = check_win(&game);
    }
    println!("{}", game);
    println!("Congrats {}! You won the game!", game.current_player);
}

fn check_win(game: &GameState) -> bool {
    //check our horizontal lines
    for i in 0..2 {
        if is_line(game.board, 0 + i, 3 + i, 6 + i) {
            return true;
        }
    }

    //Is there really not a normal for loop in this language??? oh, this is our vertical lines
    let mut i = 0;
    loop {
        if is_line(game.board, 0 + i, 1 + i, 2 + i) {
            return true;
        }
        if i > 5 {
            break ();
        }
        i += 3;
    }
    //Here are the diagonal lines
    if is_line(game.board, 0, 4, 8) || is_line(game.board, 6, 4, 2) {
        return true;
    }

    //if nothing makes a line, then no one won.
    return false;
}

fn is_line(board: [char; 9], one: usize, two: usize, three: usize) -> bool {
    return board[one] == board[two] && board[one] == board[three] && board[one] != ' ';
}

//recursive function that will not return until a valid move has been made
fn make_move(game: &mut GameState) {
    let term = Term::stdout();
    const RADIX: u32 = 10;
    //this hangs and waits for a char, then converts to a u32
    match term.read_char().unwrap().to_digit(RADIX) {
        //we only care if the input is 1-9
        Some(m @ 1..=9) => {
            let index = (m - 1) as usize;
            //make sure this move has not been made
            if game.board[index] != ' ' {
                println!("You cant move there! someone is already there!");
                make_move(game);
                return;
            }
            //make the move
            game.board[index] = game.current_player;
        }
        _ => {
            println!("That is not a valid move");
            make_move(game);
        }
    }
}

//get the opposite player
fn flip(game: &GameState) -> char {
    if game.current_player == 'x' {
        return 'o';
    } else {
        return 'x';
    }
}

struct GameState {
    board: [char; 9],
    current_player: char,
}

fn get_new_game_board() -> GameState {
    return GameState {
        board: [' ', ' ', ' ', ' ', ' ', ' ', ' ', ' ', ' '],
        current_player: 'o',
    };
}

//This is how println knows how to display the board.
impl fmt::Display for GameState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "7 | 8 | 9    {} | {} | {}\n\
                    ---------    ---------\n\
                    4 | 5 | 6    {} | {} | {}\n\
                    ---------    ---------\n\
                    1 | 2 | 3    {} | {} | {}\n\n\
                    {} Make your move!", 
            self.board[6], self.board[7], self.board[8], 
            self.board[3], self.board[4], self.board[5],
            self.board[0], self.board[1], self.board[2],
            self.current_player)
    }
}
