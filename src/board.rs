
use std::{cmp, fmt};
use crate::{piece::PieceType, Piece};
use std::ops::{Shl,Shr,BitAnd,BitOr,BitOrAssign,Not};
use rand::Rng;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Clone, Copy)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub fn to_string(&self) -> String {
        let mut v: Vec<u8> = vec![];
        let mut a = *&self.0;

        for _ in 0..64 {
            v.push((a % 2) as u8);
            a = (a - a % 2) / 2
        }

        let mut s: String = String::from("");

        v.reverse();

        for d in v {
            s += &d.to_string();
        }

        s
    }
}

#[allow(unused)]
impl fmt::Display for BitBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &mut self.to_string().chars().into_iter().enumerate() {
            if i.0 % 8 == 0 && i.0 > 0 {
                write!(f, "\n");
            }
           
            write!(f, "{}", if i.1 == '1' {"■ "} else {"• "});        
        }

        write!(f, "\n");

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

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> Self {
        BitBoard(!self.0)
    }
}

#[derive(Clone, Copy)]
pub struct Board {
    pub piece: Piece,
    path: [u8; 64],
    pub pawns: BitBoard,
}

#[allow(unused)]
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &mut self.pawns.to_string().chars().into_iter().enumerate() {
            if i.0 % 8 == 0 && i.0 > 0 {
                write!(f, "\n");
            }
           
            if i.0 == 63-self.piece.square {
                write!(f, "{}",         
                    match self.piece.type_ {
                        PieceType::Queen  => "♕ ",
                        PieceType::Rook   => "♖ ",
                        PieceType::Bishop => "♗ ",
                        PieceType::Knight => "♘ ",
                    });  
            } else {
                write!(f, "{}", if i.1 == '1' {"♟ "} else {"• "});  
            }
                     
        }

        write!(f, "\n");

        Ok(())
    }
}

impl Board {
    pub fn init(piece: Piece, pawns: BitBoard) -> Self {
        Self {
            piece: piece,
            path: [100; 64],
            pawns: pawns,
        }
    }

    fn generate_move_board(&self, move_type: &MoveType, range: usize, func: impl Fn(usize, BitBoard) -> BitBoard) -> BitBoard {
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
        m |= self.generate_move_board(&move_type, row-1,   |i: usize, b: BitBoard| -> BitBoard { b >> 8*i });

        // Up
        m |= self.generate_move_board(&move_type, 8-row,   |i, b| -> BitBoard { b << 8*i });

        // Right
        m |= self.generate_move_board(&move_type, col-1,   |i, b| -> BitBoard { b >> i });

        // Left
        m |= self.generate_move_board(&move_type, 8-col,   |i, b| -> BitBoard { b << i });

        m
    }

    fn bishop(&self, move_type: &MoveType, row: usize, col: usize) -> BitBoard {
        let mut m: BitBoard = BitBoard(0);
        
        // UL
        m |= self.generate_move_board(&move_type, cmp::min(8-row, 8-col),   |i: usize, b: BitBoard| -> BitBoard { b << 9*i });

        // UR
        m |= self.generate_move_board(&move_type, cmp::min(8-row, col-1),   |i: usize, b: BitBoard| -> BitBoard { b << 7*i });

        // DL PROBLEM
        m |= self.generate_move_board(&move_type, cmp::min(row-1, 8-col),   |i: usize, b: BitBoard| -> BitBoard { b >> 7*i });

        // DR
        m |= self.generate_move_board(&move_type, cmp::min(row-1, col-1),   |i: usize, b: BitBoard| -> BitBoard { b >> 9*i });

        // println!("m\n{}", m);
        // println!("UL{}", cmp::min(8-row, 8-col));
        // println!("UR{}", cmp::min(8-row, col-1));
        // println!("DL{}", cmp::min(row-1, 8-col));
        // println!("DR{}", cmp::min(row-1, col-1));
        // println!("Row: {} Col: {}\n\n", row, col);

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
            MoveType::Moves => m & !*&self.pawns,
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

    pub fn random_move(&mut self, move_type: MoveType) -> Result<u64, String> {
        let mut rng = rand::thread_rng();

        let moves = self.moves(move_type);
        let n = moves.0.count_ones();

        if n == 0 { 
            // println!("Fail at random_move\n{}\n", self);
            return Err("No legal moves!".to_string())
        }

        let r = rng.gen_range(1..=n);
        let mut s: u8 = 0;

        let mut c = 0;

        for i in &mut moves.to_string().chars().into_iter().enumerate() {
            if i.1 == '1' {
                c += 1;
            }

            if c == r {
                s = 63 - i.0 as u8;
                break;
            }
        }

        self.piece.make_move(s);
        self.pawns = !self.piece.location & self.pawns;
        self.path[self.path.iter().position(|&x| x == 100).unwrap()] = s;

        Ok(s.into())
    }

    pub fn create_path(&mut self, n: usize) {
        for i in 1..=10000 {
            let a = self.attempt_path(n);

            match a {
                Some(x) => {
                    self.pawns = x.pawns;
                    self.piece = x.piece;
                    self.path = x.path;
                    // println!("\n{:?}", self.path.iter().filter(|&&x| x != 100).map(|x| *x).collect::<Vec<u8>>());
                    break;
                },
                None => {},
            }
        }
    }

    fn attempt_path(&mut self, n: usize) -> Option<Board> {
        let mut b = *self;
        b.path[0] = b.piece.square as u8;


        for i in 1..=n {
            let last_position: BitBoard = b.piece.location;

            match b.random_move(MoveType::Moves) {
                Err(e) => {return None},
                Ok(s) => {b.path[i] = s as u8},
            }

            b.pawns |= last_position;
        }

        Some(b)
    }

    pub fn get_path(&self) -> Vec<u8> {
        self.path.iter().filter(|&&x| x != 100).map(|x| *x).collect()
    }

    pub fn attempt_solution(&self) -> bool {
        let mut solution: Vec<u8> = self.get_path();
        solution.reverse();

        if solution.len() == 0 {
            println!("Something went wrong!: {:?}\n", solution);
            println!("{} {}", self.piece.get_row(), self.piece.get_col());
            println!("{}", self);
            println!("{:?}", self.path);
        }


        solution.remove(0);
        

        let mut j = 0;

        use std::time::Instant;
        let now = Instant::now();

        for i in 0..100000 {
            let mut a = Board::init(self.piece, self.pawns);
            let mut p: Vec<u8> = vec![];

            loop {
                match a.random_move(MoveType::CapturesOnly) {
                    Err(_) => {p = a.get_path(); break},
                    Ok(_) => {},
                }
            }

            if p.len() == solution.len() && p != solution {
                // println!("Another solution was found: {:?}", p);
                // println!("{:?}", p);

                // if p == [] {
                //     println!("a\n{}{:?}\n", a, a.path);
                //     println!("b\n{}", a.moves(MoveType::Captures));
                // }

                // println!("p{:?}\ns{:?}", p, solution);

                // println!("{}", self);
                // let elapsed = now.elapsed();
                // println!("Elapsed1: {:.2?}", elapsed);

                return false;
            }

            j = i;
        }

        // println!("\n{:?}", solution);

        let elapsed = now.elapsed();
        println!("Elapsed2: {:.2?}", elapsed);

        println!("{}", j);

        true
    }
}

#[allow(dead_code)]
pub enum MoveType {
    Captures,
    CapturesOnly,
    Moves,
}
