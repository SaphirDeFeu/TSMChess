#![allow(unused_parens)]

pub mod d;
pub mod position;

use d::display;
use position::Position;

fn main() -> std::process::ExitCode {
    let mut x: String = String::new();
    let mut current_position: Position = Position::new();
    current_position.state.to_string();
    println!("TSMChess by TSM Studios");
    loop {
        std::io::stdin()
            .read_line(&mut x)
            .expect("Unable to read command");
        let vec_x: Vec<&str> = x.split(" ")
            .collect::<Vec<&str>>();
        let cleaned_vec: Vec<String> = vec_x.iter()
            .map(|&s| s.replace("\r", "").replace("\n", ""))
            .collect::<Vec<String>>();
        let cmd: &str = &cleaned_vec[0];
        match(cmd) {
            "position" => {
                let move_start_index: usize;
                if(&cleaned_vec[1] == "fen") {
                    let fen_string = &cleaned_vec[2..=7];
                    current_position = Position::from(&fen_string.join(" "));
                    move_start_index = 8;
                } else {
                    current_position = Position::new();
                    move_start_index = 2;
                }

                if((&cleaned_vec).len() > move_start_index) {
                    if(&cleaned_vec[move_start_index] == "moves") {
                        for proposed_move in &cleaned_vec[(move_start_index + 1)..] {
                            if(proposed_move == &String::from("!")) {
                                match(current_position.unmake_move()) {
                                    Ok(_) => (),
                                    Err(e) => return std::process::ExitCode::from(e),
                                };
                                continue;
                            }
                            let mut move_parts: Vec<String> = proposed_move.chars()
                                .collect::<Vec<char>>()
                                .chunks(2)
                                .map(|chunk| chunk.iter().collect())
                                .collect();

                            if(move_parts.len() <= 2) {
                                move_parts.push(String::new());
                            }

                            match(current_position.make_move(&move_parts[0], &move_parts[1], &move_parts[2])) {
                                Ok(_) => (),
                                Err(e) => return std::process::ExitCode::from(e),
                            }
                        }
                    }
                }
            }
            "d" => {
                display(&current_position);
            }
            "debug" => {
                match(cleaned_vec[1].as_str()) {
                    "gen" => {
                        dbg!(current_position.generate_legal_moves());
                        ()
                    }
                    _ => ()
                };
            }
            "quit" | "exit" => return std::process::ExitCode::SUCCESS,
            _ => {
                println!("Unknown command: '{}'. Type help for more information.", cmd);
            }
        }
        x = String::new();
    }
}