use crate::Board;

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