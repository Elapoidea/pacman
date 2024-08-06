
use crate::BitBoard;
use std::fmt;

enum PieceType {
    Queen,
    Rook,
    Bishop,
    Knight
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Queen => "Queen",
            Self::Rook => "Rook",
            Self::Bishop => "Bishop",
            Self::Knight => "Knight",
        })
    }
}

pub struct Piece {
    type_: PieceType,
    location: BitBoard,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({0})\n{1}", self.type_, self.location)
    }
}

impl Piece {
    pub fn init(id: u8, square: u8) -> Self {
        Self {
            type_: match id {
                1 => PieceType::Rook,
                2 => PieceType::Bishop,
                3 => PieceType::Knight,
                _ => PieceType::Queen,
            },
            location: BitBoard(2_u64.pow(square as u32)),
        }
    }
}

