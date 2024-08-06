
use std::fmt;
use crate::Piece;
use std::ops::{Shl,Shr,BitAnd};

#[derive(Clone, Copy)]
pub struct BitBoard(pub u64);

impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in self.0.to_be_bytes() {
            write!(f, "{0}{i:b}\n", (0..(7-i.checked_ilog2().unwrap_or(0))).map(|_| "0").collect::<String>());
        }

        Ok(())
    }
}

impl Shl<usize> for BitBoard {
    type Output = BitBoard;

    fn shl(self, shift: usize) -> Self::Output {
        BitBoard(self.0 << shift)
    }
}

impl Shr<usize> for BitBoard {
    type Output = BitBoard;

    fn shr(self, shift: usize) -> Self::Output {
        BitBoard(self.0 >> shift)
    }
}

impl BitAnd for BitBoard {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

pub struct Board {
    piece: Piece,
    pawns: BitBoard,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(Pawns)\n{0}\n{1}", self.pawns, self.piece)
    }
}

impl Board {
    pub fn init(piece: Piece, pawns: BitBoard) -> Self {
        Self {
            piece,
            pawns
        }
    }

    pub fn moves(&self) {
        println!("aaa{}", *&self.piece.location >> 2 & *&self.pawns);
    }
}

