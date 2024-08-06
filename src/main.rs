mod board;
use board::{Board, BitBoard};

mod piece;
use piece::Piece;

fn main() {
    println!("Hello, world!");

    let pawns: BitBoard = BitBoard(23659295935);
    let piece: Piece = Piece::init(1, 28);

    let board: Board = Board::init(piece, pawns);

    // println!("{}", board);

    board.moves();

    // println!("{}", board);
}
