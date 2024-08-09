mod board;
use board::{Board, BitBoard, MoveType};

mod piece;
use piece::Piece;

mod utils;
use utils::{fen,coordinate};

use rand::Rng;

fn find_puzzle(n: usize, piece_type: u8, max_tries: Option<usize>) -> Option<Board> {
    let mut rng = rand::thread_rng();
    
    let mut i = 0;
   
    loop {
        match max_tries {
            Some(m) => { if i == m { return None } },
            None => {},
        }

        i += 1;

        let mut pawns: BitBoard = BitBoard(0);
        // let mut pawns = BitBoard(5490939986534379144);
        let piece: Piece = Piece::init(piece_type, rng.gen_range(0..63));
        // let piece: Piece = Piece::init(3, 27);
    
        pawns = !piece.location & pawns;
    
        let mut board: Board = Board::init(piece, pawns);

        match board.create_path(n, max_tries) {
            Err(_) => {continue},
            Ok(_) => {},
        }

        let s = board.find_unique_solution();

        if s {
            println!("{}", fen(board.clone()));
            println!("This has exactly one solution: \n{}\ni: {}", board, i);
            println!("{} {}", board.pawns.0, board.piece.square);
            
            return Some(board);
        }
    }  
}

fn main() {
    use std::time::Instant;

    let now = Instant::now();

    // 5490939986534379144
    // Knight 27

    find_puzzle(20, 3, None);

    let elapsed = now.elapsed();

    println!("Elapsed: {:.2?}", elapsed);
    
}
