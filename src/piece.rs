
use crate::BitBoard;

enum PieceType {
    Queen,
    Rook,
    Bishop,
    Knight
}

pub struct Piece {
    type_: PieceType,
    location: BitBoard,
}

impl Piece {
    pub fn init(self, id: u8) -> Self {
        Self {
            type_: match id {
                1 => PieceType::Rook,
                2 => PieceType::Bishop,
                3 => PieceType::Knight,
                _ => PieceType::Queen,
            },
            location: BitBoard(0),
        }
    }
}