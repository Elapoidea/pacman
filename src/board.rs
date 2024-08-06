
use std::fmt;
use crate::Piece;
use std::ops::{Shl,Shr,BitAnd,BitOr};

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

impl PartialEq for BitBoard {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
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
        let mut m: BitBoard = *&self.piece.location;
        let c = *(&self.piece.get_col());
        let r = *(&self.piece.get_row());

        for i in 1..r {
            let s = *&self.piece.location >> 8*i;

            if s & *&self.pawns != BitBoard(0) {
                break;
            }

            m = m | s;
        }

        for i in 1..=8-r {
            let s = *&self.piece.location << 8*i;

            if s & *&self.pawns != BitBoard(0) {
                break;
            }

            m = m | s;
        }

        for i in 1..c {
            let s = *&self.piece.location >> i;

            if s & *&self.pawns != BitBoard(0) {
                break;
            }

            m = m | s;
        }

        for i in 1..=8-c {
            let s = *&self.piece.location << i;

            if s & *&self.pawns != BitBoard(0) {
                break;
            }

            m = m | s;
        }

        println!("{} {}",  c, 8-r);
        println!("{}\n{}",  *&self, m);
    }
}

