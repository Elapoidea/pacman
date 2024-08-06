mod board;
use board::{Board, BitBoard, MoveType};

mod piece;
use piece::Piece;

fn main() {
    println!("Hello, world!");

    let pawns: BitBoard = BitBoard(3498623452);
    let piece: Piece = Piece::init(0, 27);

    let board: Board = Board::init(piece, pawns);

    println!("{}\n{}", board, board.moves(MoveType::Captures));
}
