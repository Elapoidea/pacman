mod board;
use board::{Board, BitBoard};

mod piece;
use piece::Piece;

fn main() {
    println!("Hello, world!");

    let pawns: BitBoard = BitBoard(600);
    let piece: Piece = Piece::init(1, 5);

    let board: Board = Board::init(piece, pawns);

    println!("{}", board);
}
