use regex::Regex;
use std::collections::HashMap;
use std::{fs, str::FromStr};

mod piece_type;
mod position;
mod board;

#[derive(Debug)]
struct CSAFile {
    version: Option<String>,
    sente: Option<String>,
    gote: Option<String>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    prev_pos: position::Position,
    next_pos: position::Position,
    piece_type: piece_type::PieceType,
}

fn parse_csa_file() -> CSAFile {
    let data = fs::read_to_string("./sample.csa").expect("Unable to read file");
    let splitted_data = data.lines();

    let mut csa_file = CSAFile {
        version: None,
        sente: None,
        gote: None,
        moves: Vec::new(),
    };

    for line in splitted_data {
        let version_re = Regex::new(r"^V(?P<version>.*)$").unwrap();
        match version_re.captures(line) {
            Some(caps) => {
                let version = &caps["version"];
                csa_file.version = Some(version.to_string());
                continue;
            }
            None => {}
        }

        let sente_re = Regex::new(r"^N\+(?P<sente>.*)$").unwrap();
        match sente_re.captures(line) {
            Some(caps) => {
                let sente = &caps["sente"];
                csa_file.sente = Some(sente.to_string());
                continue;
            }
            None => {}
        }

        let gote_re = Regex::new(r"^N-(?P<gote>.*)$").unwrap();
        match gote_re.captures(line) {
            Some(caps) => {
                let sente = &caps["gote"];
                csa_file.gote = Some(sente.to_string());
                continue;
            }
            None => {}
        }

        let move_re =
            Regex::new(r"^[\+-](?P<prev_pos>\d{2})(?P<next_pos>\d{2})(?P<piece_type>[A-Z]{2})$")
                .unwrap();
        match move_re.captures(line) {
            Some(caps) => {
                let prev_pos_str = &caps["prev_pos"];
                let prev_pos = position::Position::from_str(prev_pos_str);

                let prev_pos = match prev_pos {
                    Ok(pos) => pos,
                    Err(error) => {
                        panic!("{:?}", error)
                    }
                };

                let next_pos_str = &caps["next_pos"];
                let next_pos = position::Position::from_str(next_pos_str).unwrap();

                let piece_type = &caps["piece_type"];

                let koma_move = Move {
                    prev_pos: prev_pos,
                    next_pos: next_pos,
                    piece_type: piece_type::PieceType::from_str(piece_type).unwrap(),
                };
                csa_file.moves.push(koma_move);

                continue;
            }
            None => {}
        }
    }

    return csa_file;
}

fn main() {
    let csa_file = parse_csa_file();


    let board = board::initialize_board();

    board.pprint();

    // println!("{:#?}", csa_file)

    // let sample = 1;

    // for num in 0..10 {
    //     println!("Hello, world!");
    // }
}
