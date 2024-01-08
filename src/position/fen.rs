pub const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

use super::piece::Piece;

#[derive(Clone)]
pub struct ParsedFEN {
    pub board: [Piece; 64],
    pub color: bool, // Such that WHITE = FALSE and BLACK = TRUE
    pub castle: u8, // 0001 = black queenside, 0010 = black kingside, 0100 = white queenside, 1000 = white kingside
    pub en_passant: u8,
    pub halfmove_clock: u8,
    pub fullmove_clock: u16,
}

impl ParsedFEN {
    pub fn from(fen_string: &str) -> ParsedFEN {
        let mut square_index: usize = 56;
        let mut char_index: usize = 0;
        let fen_parts: Vec<&str> = fen_string.split(" ").collect::<Vec<&str>>();
        let board_chars: Vec<char> = fen_parts[0].chars().collect::<Vec<char>>();
        let mut board: [Piece; 64] = [Piece {data:0,pos:0}; 64];
        while(char_index < board_chars.len()) {
            let character: &char = &board_chars[char_index];
            let space_num: u8 = match(character.to_string().parse::<u8>()) {
                Ok(v) => v,
                Err(_) => 0, // numbers outside of range 1..=8 are impossible in fen strings
            };
            if(character == &'/') {
                square_index -= 16;
                char_index += 1;
                continue;
            }
            if(space_num != 0) {
                for _ in 0..space_num {
                    board[square_index] = Piece::from(&' ', &(square_index as u8));
                    square_index += 1;
                }
                char_index += 1;
                continue;
            }
            board[square_index] = Piece::from(&character, &(square_index as u8));
            char_index += 1;
            square_index += 1;
        }
        return ParsedFEN {
            board,
            color: match(fen_parts[1]) {
                "w" => false,
                _ => true,
            },
            castle: ParsedFEN::parse_castle_text(fen_parts[2]),
            en_passant: super::parse_square(fen_parts[3]),
            halfmove_clock: match(fen_parts[4].parse::<u8>()) {
                Ok(v) => v,
                Err(_) => 0,
            },
            fullmove_clock: match(fen_parts[5].parse::<u16>()) {
                Ok(v) => v,
                Err(_) => 1,
            },
        }
    }

    pub fn new() -> ParsedFEN {
        return ParsedFEN::from(START_FEN);
    }

    pub fn to_string(&self) -> String {
        let mut square_index: usize = 56;
        let mut empty_space_counter: u8 = 0;
        let mut fen_string: String = String::new();
        for _i in 0..(&self).board.len() {
            let piece: Piece = (&self).board[square_index];
            if(piece.data != 0) {
                let piece_char: String = piece.to_string();
                if(empty_space_counter != 0) {
                    fen_string += &empty_space_counter.to_string();
                    empty_space_counter = 0;
                }
                fen_string += &piece_char;
            } else {
                empty_space_counter += 1;
            }
            if(square_index % 8 == 7 && square_index != 7) {
                // New row achieved
                square_index -= 15;
                if(empty_space_counter != 0) {
                    fen_string += &empty_space_counter.to_string();
                    empty_space_counter = 0;
                }
                fen_string += "/";
                continue;
            }
            square_index += 1;
        }
        let char_color: char = match((&self).color) {
            true => 'b',
            false => 'w',
        };
        fen_string += &format!(" {}", char_color);
        fen_string += &format!(" {}", ParsedFEN::parse_castle_value(&(&self).castle));
        fen_string += &format!(" {}", super::to_square(&(&self).en_passant));
        fen_string += &format!(" {}", (&self).halfmove_clock);
        fen_string += &format!(" {}", (&self).fullmove_clock);
        return fen_string;
    }

    pub fn parse_castle_text(castle_string: &str) -> u8 {
        let castle_parts: Vec<char> = castle_string.chars().collect::<Vec<char>>();
        if(castle_parts[0] == '-') {
            return 0;
        }
        let mut total: u8 = 0;
        for part in castle_parts {
            total += match(part) {
                'K' => 0b1000,
                'Q' => 0b0100,
                'k' => 0b0010,
                'q' => 0b0001,
                _ => 0,
            }
        }
        return total;
    }

    pub fn parse_castle_value(castle_value: &u8) -> String {
        if(castle_value == &0) {
            return String::from("-");
        }
        let mut castle: String = String::new();
        if(castle_value & &0b1000 != 0) {
            castle += "K";
        }
        if(castle_value & &0b100 != 0) {
            castle += "Q";
        }
        if(castle_value & &0b10 != 0) {
            castle += "k";
        }
        if(castle_value & &0b1 != 0) {
            castle += "q";
        }
        return castle;
    }
}

impl std::fmt::Debug for ParsedFEN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ParsedFEN {{\n    board: {:#?},\n    color: {},\n    castle: 0b{:04b},\n    en_passant: {},\n    halfmove_clock: {},\n    fullmove_clock: {},\n}}",
            self.board,
            match(self.color) {
                true => 1,
                false => 0,
            },
            self.castle,
            self.en_passant,
            self.halfmove_clock,
            self.fullmove_clock,
        )
    }
}