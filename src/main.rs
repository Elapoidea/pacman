mod board;
use board::{Board, BitBoard, MoveType};

mod piece;
use piece::Piece;

use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();

   
    // let mut pawns: BitBoard = BitBoard(0);
    // let piece: Piece = Piece::init(2, 36);
    // // let piece: Piece = Piece::init(2, 14);

    // pawns = !piece.location & pawns;

    // let mut board: Board = Board::init(piece, pawns);



    // board.create_path(20);

    // println!("f\n{}", board.moves(MoveType::Moves));
    // println!("{}", board);
    

    for i in 0..100000 {
        let mut pawns: BitBoard = BitBoard(0);
        let piece: Piece = Piece::init(2, rng.gen_range(0..63));
        // let piece: Piece = Piece::init(2, 14);
    
        pawns = !piece.location & pawns;
    
        let mut board: Board = Board::init(piece, pawns);

        // println!("Before:\n{}", board);

        board.create_path(20);

        // println!("After:\n{}", board);

        if board.pawns.0 == 0 {
            println!("{}", board.moves(MoveType::Moves));
            break;
        }
    }


    // for i in 0..100000 {
    //     let mut pawns: BitBoard = BitBoard(0);
    //     let piece: Piece = Piece::init(2, rng.gen_range(0..63));
    //     // let piece: Piece = Piece::init(2, 14);
    
    //     pawns = !piece.location & pawns;
    
    //     let mut board: Board = Board::init(piece, pawns);

    //     board.create_path(20);

    //     println!("{}", board);

    //     let s = board.attempt_solution();

    //     if s {
    //         println!("This has exactly one solution: \n{:?}\n{}\n{}", 
    //         board.get_path().into_iter().rev().map(
    //             |x| 
    //             (["a", "b", "c", "d", "e", "f", "g", "h"][8-(1 + x as usize % 8)].to_owned() + 
    //             ["1", "2", "3", "4", "5", "6", "7", "8"][x as usize / 8])
    //         ).collect::<Vec<String>>(),
    //         board, 
    //         i
    //     );
    //         println!("{} {}", board.pawns.0, board.piece.square);
    //         break;
    //     }
    // }
}
