mod bitboard;
use bitboard::BitBoard;

mod piece;
use piece::Piece;

fn main() {
    println!("Hello, world!");

    let x = BitBoard(100000000000);
    println!("{}", x);
}
