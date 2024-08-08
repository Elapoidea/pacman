
use crate::BitBoard;
use std::fmt;

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum PieceType {
    Queen,
    Rook,
    Bishop,
    Knight
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match self {
            Self::Queen  => "Queen",
            Self::Rook   => "Rook",
            Self::Bishop => "Bishop",
            Self::Knight => "Knight",
        })
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct Piece {
    pub type_: PieceType,
    pub id: u8,
    pub location: BitBoard,
    pub square: usize,
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
            id: id,
            location: BitBoard(2_u64.pow(square as u32)),
            square: square as usize,
        }
    }

    pub fn make_move(&mut self, square: u8) {
        let new = Self::init(self.id, square);

        self.location = new.location;
        self.square = new.square;
    }

    pub fn get_col(&self) -> usize {
        1 + &self.square % 8
    }

    pub fn get_row(&self) -> usize {
        1 + &self.square / 8
    }
}
