use std::default;
use text_io::read;
use regex;

//use text_io::read
enum GameState{
    Startup,
    WhiteTurn,
    BlackTurn,
    End
}

#[derive(Copy, Clone)]
enum Owner {
    White,
    Black,
    None
}

#[derive(Copy, Clone)]
enum Type {
    King,
    Queen,
    Knight,
    Bishop,
    Rook,
    Pawn,
    None
}

#[derive(Copy, Clone)]
struct Piece{
    ptype: Type,
    owner: Owner
}

impl Default for Piece{
    fn default() -> Piece {
        Piece {
            ptype: Type::None,
            owner: Owner::None
        }
    }
}

fn setup_board(board: &mut Vec<Vec<Piece>>){
    board[0][0] = Piece {ptype: Type::Rook, owner: Owner::White};
    board[0][1] = Piece {ptype: Type::Knight, owner: Owner::White};
    board[0][2] = Piece {ptype: Type::Bishop, owner: Owner::White};
    board[0][3] = Piece {ptype: Type::Queen, owner: Owner::White};
    board[0][4] = Piece {ptype: Type::King, owner: Owner::White};
    board[0][5] = Piece {ptype: Type::Bishop, owner: Owner::White};
    board[0][6] = Piece {ptype: Type::Knight, owner: Owner::White};
    board[0][7] = Piece {ptype: Type::Rook, owner: Owner::White};
    board[1] = vec![Piece {ptype: Type::Pawn, owner: Owner::White}; 8];

    board[7][0] = Piece {ptype: Type::Rook, owner: Owner::Black};
    board[7][1] = Piece {ptype: Type::Knight, owner: Owner::Black};
    board[7][2] = Piece {ptype: Type::Bishop, owner: Owner::Black};
    board[7][3] = Piece {ptype: Type::Queen, owner: Owner::Black};
    board[7][4] = Piece {ptype: Type::King, owner: Owner::Black};
    board[7][5] = Piece {ptype: Type::Bishop, owner: Owner::Black};
    board[7][6] = Piece {ptype: Type::Knight, owner: Owner::Black};
    board[7][7] = Piece {ptype: Type::Rook, owner: Owner::Black};
    board[6] = vec![Piece {ptype: Type::Pawn, owner: Owner::Black}; 8];
}

fn draw_line(){
    println!("├───┼───┼───┼───┼───┼───┼───┼───┤");
}
fn piece_to_symbol(piece: Piece) -> char{
    match piece.owner {
        Owner::White =>{
            match piece.ptype{
                Type::King=>'♚',
                Type::Queen=>'♛',
                Type::Bishop=>'♝',
                Type::Knight=>'♞',
                Type::Rook=>'♜',
                Type::Pawn=>'♟',
                _ => ' '
            }
        },
        Owner::Black => {
            match piece.ptype{
                Type::King=>'♔',
                Type::Queen=>'♕',
                Type::Bishop=>'♗',
                Type::Knight=>'♘',
                Type::Rook=>'♖',
                Type::Pawn=>'♙',
                _ => ' '
            }
        },
        Owner::None => ' '
    }
}


fn draw_row(row: &Vec<Piece>){
    println!("| {0} | {1} | {2} | {3} | {4} | {5} | {6} | {7} |",
    piece_to_symbol(row[0]),piece_to_symbol(row[1]),piece_to_symbol(row[2]),
    piece_to_symbol(row[3]),piece_to_symbol(row[4]),piece_to_symbol(row[5]),
    piece_to_symbol(row[6]),piece_to_symbol(row[7]));
}

fn draw_board(board: &Vec<Vec<Piece>>){
    println!("┌───┬───┬───┬───┬───┬───┬───┬───┐");
    draw_row(&board[7]);
    for i in (0..7).rev(){
        draw_line();
        draw_row(&board[i]);
    }
    println!("└───┴───┴───┴───┴───┴───┴───┴───┘");
}

fn check_move(board: &Vec<Vec<Piece>>,tgt: (u8,u8), piece: Type) -> (u8,u8){
    match piece{
        _ =>
    }

    (0,0)
}

fn take_turn(board: &mut Vec<Vec<Piece>>){
    let command : String = read!();
    println!("{0}",command);
}


fn main() {
    let mut state : GameState = GameState::Startup;
    let mut board = vec![vec![Piece::default(); 8];8];
    loop{
        match state {
            GameState::Startup => {
                println!("Welcome to Rust Chess!");
                setup_board(&mut board);
                draw_board(&board);
                state = GameState::WhiteTurn;
            },
            GameState::WhiteTurn => {
                println!("Whites Turn:");
                take_turn(&mut board);
                state = GameState::BlackTurn;
            },
            GameState::BlackTurn => {
                println!("Black Turn:");
                take_turn(&mut board);
                state = GameState::End;
            }
            GameState::End =>{
                println!("Thanks for Playing Rust Chess!");
                println!("Play again?");
                state = GameState::Startup;
            }
        }
    }
}
