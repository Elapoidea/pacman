mod board;
use board::{Board, BitBoard, MoveType};

mod piece;
use piece::Piece;

mod utils;
use utils::{fen,coordinate};

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

   
    // let mut pawns: BitBoard = BitBoard(0);
    // let piece: Piece = Piece::init(2, 36);
    // // let piece: Piece = Piece::init(2, 14);

    // pawns = !piece.location & pawns;

    // let mut board: Board = Board::init(piece, pawns);



    // board.create_path(20);

    // println!("f\n{}", board.moves(MoveType::Moves));
    // println!("{}", board);

    for i in 0..100000 {
        let mut pawns: BitBoard = BitBoard(1128101618655312);
        // let mut pawns: BitBoard = BitBoard(0);
        // let piece: Piece = Piece::init(2, rng.gen_range(0..63));
        let piece: Piece = Piece::init(2, 11);
    
        pawns = !piece.location & pawns;
    
        let mut board: Board = Board::init(piece, pawns);

        // board.create_path(4);

        let s = board.random_attempt();

        if s {
            println!("{}", fen(board));
            println!("This has exactly one solution: \n{:?}\n{}\n{}", 
            board.get_path().into_iter().rev().map(
                |x| 
                coordinate(x)
            ).collect::<Vec<String>>(),
            board, 
            i);
            println!("{} {}", board.pawns.0, board.piece.square);
            break;
        }
    }
}
