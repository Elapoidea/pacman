mod bitboard;
use bitboard::BitBoard;

mod piece;
use piece::Piece;

fn main() {
    println!("Hello, world!");

    let x = BitBoard(256);
    println!("{}", x);

    let p: Piece = Piece::init(1, 5);

    println!("{}", p);
}
