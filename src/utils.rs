use crate::Board;

pub fn coordinate(square: u8) -> String {
    ["a", "b", "c", "d", "e", "f", "g", "h"][8-(1 + square as usize % 8)].to_owned() + ["1", "2", "3", "4", "5", "6", "7", "8"][square as usize / 8]
}

pub fn fen (position: Board) -> String {
    let mut f = String::from("");
    let mut space = 0;

    for i in position.to_string().chars().filter(|&c| c != ' ').collect::<Vec<char>>() {
        if i == '•' {
            space += 1;

            continue;
        }

        if space != 0 {
            f += space.to_string().as_str();
        }

        if i == '\n' {
            f += "/";
        }
        
        space = 0;
        
        if i == '♟' {
            f += "p";
        }

        if i == '♕' {
            f += "Q";
        }

        if i == '♖' {
            f += "R";
        }

        if i == '♗' {
            f += "B";
        }

        if i == '♘' {
            f += "N";
        }
    }

    f.remove(f.len() - 1);
    f += " w - - 0 1";

    println!("{}", f);

    "".to_string()
}