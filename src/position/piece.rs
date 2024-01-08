#[derive(Copy, Clone)]
pub struct Piece {
    pub data: u8, // ffffCTTT where f = flag, C = color (see fen.rs), T = type
    pub pos: u8,
}

/*
    Flag table
    - fffF where F = hasMovedFlag : 1 = piece has moved at least once ; 0 = piece has never moved once
    - Ffff (for rooks) where F = queenKingside : 1 = rook is queenside, 0 = rook is kingside
 */

impl Piece {
    pub fn from(piece_char: &char, position: &u8) -> Piece {
        let mut data: u8 = 0;
        let pos: u8 = position.clone();
        if(piece_char.is_ascii_lowercase()) {
            data = data | 0b1000;
        }
        data = data | match(piece_char.to_ascii_lowercase()) {
            'p' => 0b001,
            'n' => 0b010,
            'b' => 0b011,
            'r' => 0b100,
            'q' => 0b101,
            'k' => 0b110,
            _ => 0,
        };
        return Piece {
            data,
            pos,
        };
    }

    pub fn to_string(&self) -> String {
        let final_string: String = String::from(match((&self).data & 0b111) {
            0b001 => "p",
            0b010 => "n",
            0b011 => "b",
            0b100 => "r",
            0b101 => "q",
            0b110 => "k",
            _ => " ",
        });
        if((&self).data & 0b1000 == 0) {
            return final_string.to_ascii_uppercase();
        } else {
            return final_string;
        }
    }
}

impl std::fmt::Debug for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Piece {{ data: 0b{:08b}, pos: {:02} }}",
            self.data,
            self.pos
        )
    }
}
