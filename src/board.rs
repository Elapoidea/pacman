
use std::{collections::btree_map::Range, fmt};
use crate::Piece;
use std::ops::{Shl,Shr,BitAnd,BitOr,BitOrAssign};

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

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
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

    fn generate_path(&self, range: usize, func: impl Fn(usize, BitBoard) -> BitBoard) -> BitBoard {
        let mut result = BitBoard(0);
        
        for i in 1..range {
            let s = func(i, *&self.piece.location);

            println!("result{}", s);

            if s & *&self.pawns != BitBoard(0) {
                break;
            }

            result = result | s;
        }

        

        result
    }

    pub fn moves(&self) {
        let mut m: BitBoard = BitBoard(0);
        let c = *(&self.piece.get_col());
        let r = *(&self.piece.get_row());

        m |= self.generate_path(r,   |i: usize, b: BitBoard| -> BitBoard { b >> 8*i });
        m |= self.generate_path(9-r, |i, b| -> BitBoard { b << 8*i });
        m |= self.generate_path(c,   |i, b| -> BitBoard { b >> i });
        m |= self.generate_path(9-c, |i, b| -> BitBoard { b << i });


        println!("{} {}",  c, 8-r);
        println!("{}\n{}",  *&self, m);
    }
}

