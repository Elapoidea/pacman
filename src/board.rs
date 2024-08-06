
use std::{cmp, fmt};
use crate::{piece::PieceType, Piece};
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

    fn generate_path(&self, move_type: &MoveType, range: usize, func: impl Fn(usize, BitBoard) -> BitBoard) -> BitBoard {
        let mut result = BitBoard(0);
        
        for i in 1..=range {
            let s = func(i, *&self.piece.location);
            let empty = s & *&self.pawns == BitBoard(0);

            match move_type {
                MoveType::Captures => {
                    result = result | s;
                    
                    if !empty {
                        break;
                    }
                },
                MoveType::CapturesOnly => {
                    if !empty {
                        result = result | s;

                        break;
                    }
                },
                MoveType::Moves => {
                    if !empty {
                        break;
                    }
        
                    result = result | s;
                },
            }
        }

        result
    }

    fn queen(&self, move_type: &MoveType, row: usize, col: usize) -> BitBoard {
        self.rook (move_type, row, col) | self.bishop(move_type, row, col)
    }

    fn rook(&self, move_type: &MoveType, row: usize, col: usize) -> BitBoard {
        let mut m: BitBoard = BitBoard(0);
        
        // Down
        m |= self.generate_path(&move_type, row-1,   |i: usize, b: BitBoard| -> BitBoard { b >> 8*i });

        // Up
        m |= self.generate_path(&move_type, 8-row,   |i, b| -> BitBoard { b << 8*i });

        // Right
        m |= self.generate_path(&move_type, col-1,   |i, b| -> BitBoard { b >> i });

        // Left
        m |= self.generate_path(&move_type, 8-col,   |i, b| -> BitBoard { b << i });

        m
    }

    fn bishop(&self, move_type: &MoveType, row: usize, col: usize) -> BitBoard {
        let mut m: BitBoard = BitBoard(0);
        
        // Down left
        m |= self.generate_path(&move_type, cmp::min(row, 8-col),   |i: usize, b: BitBoard| -> BitBoard { b >> 7*i });

        // Down right
        m |= self.generate_path(&move_type, cmp::min(row, col)-1,   |i: usize, b: BitBoard| -> BitBoard { b >> 9*i });

        // Up right
        m |= self.generate_path(&move_type, cmp::min(row, col)-1,   |i: usize, b: BitBoard| -> BitBoard { b << 7*i });

        // Up left
        m |= self.generate_path(&move_type, cmp::min(row, 8-col),   |i: usize, b: BitBoard| -> BitBoard { b << 9*i });

        m
    }

    fn knight(&self, move_type: &MoveType, row: usize, col: usize) -> BitBoard {
        let mut m: BitBoard = BitBoard(0);
        let p = *&self.piece.location;
        let c = 8-col+1;

        m |= if c < 7 && row < 8 {p << 6}  else {BitBoard(0)};
        m |= if c > 2 && row < 8 {p << 10} else {BitBoard(0)};
        m |= if c < 8 && row < 7 {p << 15} else {BitBoard(0)};
        m |= if c > 1 && row < 7 {p << 17} else {BitBoard(0)};

        m |= if c > 2 && row > 1 {p >> 6}  else {BitBoard(0)};
        m |= if c < 7 && row > 1 {p >> 10} else {BitBoard(0)};
        m |= if c > 1 && row > 2 {p >> 15} else {BitBoard(0)};
        m |= if c < 8 && row > 2 {p >> 17} else {BitBoard(0)};

        match move_type {
            MoveType::CapturesOnly => m & *&self.pawns,
            _ => m,
        }
    }

    pub fn moves(&self, move_type: MoveType) -> BitBoard {
        let row = *(&self.piece.get_row());
        let col = *(&self.piece.get_col());

        match self.piece.type_ {
            PieceType::Queen  => { self.queen (&move_type, row, col)},
            PieceType::Rook   => { self.rook  (&move_type, row, col) },
            PieceType::Bishop => { self.bishop(&move_type, row, col) },
            PieceType::Knight => { self.knight(&move_type, row, col)},
        }
    }
}

#[allow(dead_code)]
pub enum MoveType {
    Captures,
    CapturesOnly,
    Moves,
}
