pub mod fen;
pub mod piece;

use self::{fen::ParsedFEN, piece::Piece};

#[derive(Debug, Clone)]
pub struct Position {
    pub state: ParsedFEN,
    pub fen: String,
    pub old_position: Option<ParsedFEN>,
}

impl Position {
    pub fn from(fen_string: &str) -> Position {
        return Position {
            state: ParsedFEN::from(&fen_string),
            fen: fen_string.to_string(),
            old_position: None,
        };
    }

    pub fn new() -> Position {
        return Position {
            state: ParsedFEN::from(fen::START_FEN),
            fen: fen::START_FEN.to_string(),
            old_position: None,
        };
    }

    pub fn make_move(&mut self, origin: &str, target: &str, promotion: &str) -> Result<(), String> {

        let origin_as_u8: u8 = parse_square(origin);
        let target_as_u8: u8 = parse_square(target);

        let origin_row: i8 = ((origin_as_u8 & 0b111000) >> 3).try_into().unwrap();
        let target_row: i8 = ((target_as_u8 & 0b111000) >> 3).try_into().unwrap();
        let row_diff: i8 = target_row - origin_row;

        let origin_col: i8 = (origin_as_u8 & 0b111).try_into().unwrap();
        let target_col: i8 = (target_as_u8 & 0b111).try_into().unwrap();
        let col_diff: i8 = target_col - origin_col;

        let legal_moves: Vec<(u8, u8)> = self.generate_legal_moves();
        
        if(!legal_moves.contains(&(origin_as_u8, target_as_u8))) {
            return Err(format!("{}{} ({:2}-{:2}) is not a legal move!", origin, target, origin_as_u8, target_as_u8));
        }

        let old_board: ParsedFEN = self.state.clone();
        
        // Is a pawn capturing en passant?
        if(target_as_u8 == self.state.en_passant) {
            let modifier: i8 = match(target_row) {
                5 => -1,
                _ => 1,
            };
            let en_passant_target_row: u8 = (target_row as i8 + modifier).try_into().unwrap();
            let en_passant_target: u8 = (en_passant_target_row << 3) | (target_as_u8 & 0b111);
            
            self.state.board[en_passant_target as usize].data = 0;
        }

        // Is a king castling?
        if((self.state.board[origin_as_u8 as usize].data & 0b111 == 0b110) && (col_diff == 2 || col_diff == -2)) {
            let origin_rook_col: i8 = match(col_diff) {
                2 => 7,
                _ => 0,
            };
            let target_rook_col: i8 = match(col_diff) {
                2 => 5,
                _ => 3,
            };
            let origin_rook: u8 = ((origin_row << 3) | origin_rook_col).try_into().unwrap();
            let target_rook: u8 = ((target_row << 3) | target_rook_col).try_into().unwrap();

            let castling_invert: u8 = match(self.state.board[origin_rook as usize].data & 0b1000) {
                0b1000 => 0b1100,
                _ => 0b11,
            };

            self.state.board[target_rook as usize].data = self.state.board[origin_rook as usize].data;
            self.state.board[origin_rook as usize].data = 0;
            
            self.state.castle &= castling_invert;
        }
        
        // Is a rook moving?
        if(self.state.board[origin_as_u8 as usize].data & 0b111 == 0b100) {
            let mut castling_mask: u8 = match(origin_col) {
                7 => 0b10,
                _ => 0b1,
            };
            if(self.state.board[origin_as_u8 as usize].data & 0b1000 == 0) {
                castling_mask <<= 2;
            }

            castling_mask ^= 0b1111;

            self.state.castle &= castling_mask;
        }

        // Has a pawn reached the end row?
        if((self.state.board[origin_as_u8 as usize].data & 0b111 == 0b1) && (target_row == 7 || target_row == 0)) {
            let mut promotion_as_u8: u8 = piece::Piece::from(&promotion.chars().next().unwrap(), &0).data & 0b111;
            promotion_as_u8 |= (self.state.board[origin_as_u8 as usize].data & 0b1000);
            
            self.state.board[origin_as_u8 as usize].data = promotion_as_u8;
        }

        // Has a pawn moved 2 squares forward?
        if((self.state.board[origin_as_u8 as usize].data & 0b111 == 0b1) && (row_diff == 2 || row_diff == -2)) {
            let modifier: i8 = row_diff / 2;

            let en_passant_row: i8 = origin_row + modifier;
            let en_passant_row_as_u8: u8 = en_passant_row.try_into().unwrap();
            
            let en_passant_square: u8 = (en_passant_row_as_u8 << 3) | (origin_as_u8 & 0b111);
            self.state.en_passant = en_passant_square;
        } else {
            self.state.en_passant = 64;
        }

        // Are we capturing a piece or is this a pawn we're moving?
        if(self.state.board[target_as_u8 as usize].data & 0b111 != 0) {
            self.state.halfmove_clock = 0;
        } else if(self.state.board[origin_as_u8 as usize].data & 0b111 == 1) {
            self.state.halfmove_clock = 0;
        } else {
            self.state.halfmove_clock += 1;
        }

        self.state.board[origin_as_u8 as usize].data |= 0b10000; // Has moved
        self.state.board[target_as_u8 as usize].data = self.state.board[origin_as_u8 as usize].data;
        self.state.board[origin_as_u8 as usize].data = 0;

        if(self.state.color) {
            self.state.fullmove_clock += 1;
        }

        self.state.color = !self.state.color;

        self.fen = self.state.to_string();

        self.old_position = Some(old_board);
        return Ok(());
    }

    pub fn unmake_move(&mut self) {
        self.state = match(&self.old_position) {
            Some(old_state) => old_state.clone(),
            None => {
                eprintln!("Cannot unmake move: No former state found!");
                self.state.clone()
            }
        };
        self.old_position = None;
        self.fen = self.state.to_string();
    }

    pub fn generate_legal_moves(&self) -> Vec<(u8, u8)> {
        let mut all_legal_moves: Vec<(u8, u8)> = Vec::new();
        let color: u8 = (self.state.color as u8) << 3;

        let mut piece_amounts: u8 = 0;

        for i in 0..self.state.board.len() {
            let piece: &Piece = &self.state.board[i];

            // if the square we're on isn't a piece of our color (= empty or enemy)
            if(piece.data & 0b1000 != color || piece.data & 0b111 == 0) {
                continue;
            } else {
                piece_amounts += 1;
            }

            match(piece.data & 0b111) {
                // PAWNS
                1 => {
                    let row_modifier: isize = ((color >> 3) as isize) * -2 + 1;

                    // Check if we're moving to the square in front of us
                    if(self.state.board[(i as isize + row_modifier * 8) as usize].data == 0) {
                        all_legal_moves.push((i as u8, (i as isize + row_modifier * 8) as u8));

                        // Can we move 2 squares forward?
                        let tmp: usize = (i as isize + row_modifier * 16) as usize;
                        if(tmp < self.state.board.len()) {
                            if(self.state.board[tmp].data == 0 && piece.data & 0b10000 == 0) {
                                all_legal_moves.push((i as u8, (i as isize + row_modifier * 16) as u8));
                            }   
                        }
                    }

                    if((i % 8) != 7) {
                        let index: usize = (i as isize + row_modifier * 8 + 1) as usize;
                        let square_at_index = &self.state.board[index];
                        let piece_color: u8 = square_at_index.data & 0b1000;
                        if(square_at_index.data & 0b111 != 0 && piece_color != color) {
                            all_legal_moves.push((i as u8, index as u8));
                        }
                    }

                    if((i % 8) != 0) {
                        let index: usize = (i as isize + row_modifier * 8 - 1) as usize;
                        let square_at_index: &Piece = &self.state.board[index];
                        let piece_color: u8 = square_at_index.data & 0b1000;
                        if(square_at_index.data & 0b111 != 0 && piece_color != color) {
                            all_legal_moves.push((i as u8, index as u8));
                        }
                    }
                }
                // KNIGHTS
                2 => {
                    let mut move_one: u8 = 64;
                    let mut move_two: u8 = 64;
                    let mut move_three: u8 = 64;
                    let mut move_four: u8 = 64;
                    let mut move_five: u8 = 64;
                    let mut move_six: u8 = 64;
                    let mut move_seven: u8 = 64;
                    let mut move_eight: u8 = 64;

                    // at least third row
                    if(16 <= i) {
                        // not first column
                        if(i % 8 != 0) {
                            move_seven = i as u8 - 17;
                        }
                        // not last/eighth column
                        if(i % 8 != 7) {
                            move_eight = i as u8 - 15;
                        }
                    }

                    // at least second row
                    if(10 <= i) {
                        // not first nor second column
                        if(i % 8 != 0 && i % 8 != 1) {
                            move_five = i as u8 - 10;
                        }
                        // not seventh or last/eighth column
                        if(i % 8 != 6 && i % 8 != 7) {
                            move_six = i as u8 - 6;
                        }
                    }

                    // before third-to-last row
                    if(i <= 47) {
                        // not first column
                        if(i % 8 != 0) {
                            move_one = i as u8 + 15;
                        }
                        // not last/eighth column
                        if(i % 8 != 7) {
                            move_two = i as u8 + 17;
                        }
                    }

                    // before second-to-last row
                    if(i <= 53) {
                        // not last/eigth nor seventh column
                        if(i % 8 != 6 && i % 8 != 7) {
                            move_four = i as u8 + 10;
                        }
                        // not first/second column
                        if(i % 8 != 0 && i % 8 != 1) {
                            move_three = i as u8 + 6;
                        }
                    }

                    // checks for same-colored pieces
                    if(move_one != 64) {
                        if(self.state.board[move_one as usize].data & 0b111 == 0 || self.state.board[move_one as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_one));
                        }
                    }
                    if(move_two != 64) {
                        if(self.state.board[move_two as usize].data & 0b111 == 0 || self.state.board[move_two as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_two));
                        }
                    }
                    if(move_three != 64) {
                        if(self.state.board[move_three as usize].data & 0b111 == 0 || self.state.board[move_three as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_three));
                        }
                    }
                    if(move_four != 64) {
                        if(self.state.board[move_four as usize].data & 0b111 == 0 || self.state.board[move_four as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_four));
                        }
                    }
                    if(move_five != 64) {
                        if(self.state.board[move_five as usize].data & 0b111 == 0 || self.state.board[move_five as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_five));
                        }
                    }
                    if(move_six != 64) {
                        if(self.state.board[move_six as usize].data & 0b111 == 0 || self.state.board[move_six as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_six));
                        }
                    }
                    if(move_seven != 64) {
                        if(self.state.board[move_seven as usize].data & 0b111 == 0 || self.state.board[move_seven as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_seven));
                        }
                    }
                    if(move_eight != 64) {
                        if(self.state.board[move_eight as usize].data & 0b111 == 0 || self.state.board[move_eight as usize].data & 0b1000 != color) {
                            all_legal_moves.push((i as u8, move_eight));
                        }
                    }
                }
                // BISHOPS
                3 => {
                    all_legal_moves.append(&mut self.generate_sliding_move(9, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(7, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-7, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-9, i));
                }
                // ROOKS
                4 => {
                    all_legal_moves.append(&mut self.generate_sliding_move(8, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(1, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-1, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-8, i));
                }
                // QUEENS
                5 => {
                    all_legal_moves.append(&mut self.generate_sliding_move(8, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(1, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-1, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-8, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(9, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(7, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-7, i));
                    all_legal_moves.append(&mut self.generate_sliding_move(-9, i));
                }
                // KINGS
                6 => {
                    all_legal_moves.append(&mut self.generate_one_move(8, i));
                    all_legal_moves.append(&mut self.generate_one_move(1, i));
                    all_legal_moves.append(&mut self.generate_one_move(-1, i));
                    all_legal_moves.append(&mut self.generate_one_move(-8, i));
                    all_legal_moves.append(&mut self.generate_one_move(9, i));
                    all_legal_moves.append(&mut self.generate_one_move(7, i));
                    all_legal_moves.append(&mut self.generate_one_move(-7, i));
                    all_legal_moves.append(&mut self.generate_one_move(-9, i));
                }
                _ => ()
            }

            if(piece_amounts > 15) {
                // Stop the loop short if we have visited all pieces of a single color
                break;
            }
        }

        return all_legal_moves;
    }

    fn generate_one_move(&self, offset: isize, origin_square: usize) -> Vec<(u8, u8)> {
        let mut arr: Vec<(u8, u8)> = Vec::new();
        let tmp_square: isize = origin_square as isize + offset;
        let modulo: isize = origin_square as isize % 8;
        if(!(modulo + 1 == tmp_square % 8 || modulo - 1 == tmp_square % 8 || modulo == tmp_square % 8)) {
            return arr;
        }
        if(tmp_square < 0 || tmp_square >= 64) {
            return arr;
        }
        if(self.state.board[tmp_square as usize].data & 0b111 != 0) {
            if((self.state.board[tmp_square as usize].data & 0b1000) >> 3 != self.state.color as u8) {
                arr.push((origin_square as u8, tmp_square as u8));
                return arr;
            } else {
                return arr;
            }
        }
        arr.push((origin_square as u8, tmp_square as u8));
        return arr;
    }

    fn generate_sliding_move(&self, offset: isize, origin_square: usize) -> Vec<(u8, u8)> {
        let mut arr: Vec<(u8, u8)> = Vec::new();
        let mut tmp_square: isize = origin_square as isize;
        let mut modulo: isize = tmp_square % 8;
        while(modulo + 1 == tmp_square % 8 || modulo - 1 == tmp_square % 8 || modulo == tmp_square % 8) {
            modulo = tmp_square % 8;
            if(tmp_square < 0 || tmp_square >= 64) {
                break;
            }
            if(self.state.board[tmp_square as usize].data & 0b111 != 0) {
                if(tmp_square == origin_square as isize) {
                    tmp_square += offset;
                    continue;
                }
                if((self.state.board[tmp_square as usize].data & 0b1000) >> 3 != self.state.color as u8) {
                    arr.push((origin_square as u8, tmp_square as u8));
                    break;
                } else {
                    break;
                }
            }
            if(tmp_square != origin_square as isize) {
                arr.push((origin_square as u8, tmp_square as u8));
            }
            tmp_square += offset;
        }
        return arr;
    }
}

pub fn to_square(square: &u8) -> String {
    if(square == &0b1000000) {
        return String::from("-");
    }
    let row: u8 = (square & 0b111000) >> 3;
    let col: &str = match(square & 0b111) {
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => "a",
    };
    return format!("{}{}", col, row + 1);
}

pub fn parse_square(square: &str) -> u8 {
    if(square == "-") {
        return 0b1000000;
    }
    let parts: Vec<char> = square.chars().collect::<Vec<char>>();
    let row: u8 = match(parts[1].to_string().parse::<u8>()) {
        Ok(v) => v - 1,
        Err(_) => 8,
    };
    return (row << 3) + (match(parts[0]) {
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => 0,
    });
}