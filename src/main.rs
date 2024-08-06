mod board;
use board::{Board, BitBoard, MoveType};

mod piece;
use piece::Piece;

use rand::Rng;

fn main() {
    println!("Hello, world!");

    let mut rng = rand::thread_rng();

    let pawns: BitBoard = BitBoard(rng.gen::<u64>());
    let piece: Piece = Piece::init(3, 0);

    let board: Board = Board::init(piece, pawns);

    println!("{}\n{}", board, board.moves(MoveType::Captures));
}
