mod board;
use board::{Board, BitBoard, MoveType};

mod piece;
use piece::Piece;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

    let mut pawns: BitBoard = BitBoard(0);

    let piece: Piece = Piece::init(1, rng.gen_range(0..63));

    pawns = !piece.location & pawns;

    let mut board: Board = Board::init(piece, pawns);

    println!("{}", board);

    for _ in 0..10 {
        board.random_move();
        println!("{}", board);
    }
    

    
}
