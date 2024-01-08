use super::Position;
use super::position::piece::Piece;

pub fn display(position: &Position) {
    print!("\n ┌───┬───┬───┬───┬───┬───┬───┬───┐\n │");
    let board: &[Piece; 64] = &position.state.board;
    let mut row: usize = 7;
    let mut col: usize = 0;
    let mut square_index: usize = 8 * row + col;
    let mut i: usize = 0;
    while(i < board.len()) {
        // print!(" │");
        // for col in board[row] {
        //     print!(" {} │", super::position::piece::get_char_from_type(&col.data));
        // }
        // print!(" {}\n ", 8 - row);
        // if(row != 7) {
        //     print!("├───┼───┼───┼───┼───┼───┼───┼───┤");
        // } else {
        //     print!("└───┴───┴───┴───┴───┴───┴───┴───┘");
        // }
        // print!("\n");
        if(col == 8) {
            print!(" {}\n ├───┼───┼───┼───┼───┼───┼───┼───┤\n │", row + 1);
            row -= 1;
            col = 0;
            square_index = 8 * row + col;
            continue;
        }
        let character = board[square_index].to_string();
        print!(" {} │", character);
        
        col += 1;
        square_index = 8 * row + col;
        i += 1;
    }
    print!(" 1\n └───┴───┴───┴───┴───┴───┴───┴───┘\n");
    println!("   a   b   c   d   e   f   g   h");
    println!("\nFen: {}", position.fen);
    // println!("Key: ");
    println!("Checkers: ");
}