mod board;
use board::{Board, BitBoard, MoveType};

mod piece;
use piece::Piece;

mod utils;
use utils::{fen,coordinate};

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    use std::time::Instant;
    
   
    for l in 0..1000 {
        
        // let mut pawns: BitBoard = BitBoard(1128101618655312);
        // let mut pawns: BitBoard = BitBoard(541073424);
        let mut pawns: BitBoard = BitBoard(0);
        let piece: Piece = Piece::init(0, rng.gen_range(0..63));
        // let piece: Piece = Piece::init(0, 3);
    
        pawns = !piece.location & pawns;
    
        let mut board: Board = Board::init(piece, pawns);

        let now = Instant::now();

        board.create_path(5);

        let elapsed = now.elapsed();
        println!("Elapsed: {:.2?}", elapsed);

        let now2 = Instant::now();

        let s = board.hash_map_attempt();

        let elapsed2 = now2.elapsed();
        println!("Elapsed2: {:.2?}", elapsed2);

        if s {
            println!("{}", fen(board.clone()));
            println!("This has exactly one solution: \n{}\ni: {}", board, l);
            println!("{} {}", board.pawns.0, board.piece.square);
            break;
        }
    }
}
