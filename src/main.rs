mod board;
use board::{Board, BitBoard};

mod piece;
use piece::Piece;

fn main() {
    println!("Hello, world!");

    let pawns: BitBoard = BitBoard(1023);
    let piece: Piece = Piece::init(1, 4);

    let board: Board = Board::init(piece, pawns);

    // println!("{}", board);

    board.moves();

    println!("{}", board);
}
