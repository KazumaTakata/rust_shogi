use regex::Regex;
use std::{fs, str::FromStr};

use crate::board;
use crate::csa;
use crate::move_koma;
use crate::piece_type;
use crate::position;

#[derive(Debug)]
pub struct CSAFile {
    version: Option<String>,
    sente: Option<String>,
    gote: Option<String>,
    pub moves: Vec<move_koma::Move>,
}

pub fn parse_csa_file() -> Vec<CSAFile> {
    let paths = fs::read_dir("./data/2018").unwrap();

    let mut csa_file_vector: Vec<CSAFile> = Vec::new();

    let version_re = Regex::new(r"^V(?P<version>.*)$").unwrap();
    let sente_re = Regex::new(r"^N\+(?P<sente>.*)$").unwrap();
    let gote_re = Regex::new(r"^N-(?P<gote>.*)$").unwrap();
    let move_re = Regex::new(
        r"^(?P<teban>[\+-])(?P<prev_pos>\d{2})(?P<next_pos>\d{2})(?P<piece_type>[A-Z]{2})$",
    )
    .unwrap();

    for path in paths {
        let path = path.unwrap().path();
        println!("Name: {}", path.display());

        let data = fs::read_to_string(path).expect("Unable to read file");

        let splitted_data = data.lines();

        let mut csa_file = CSAFile {
            version: None,
            sente: None,
            gote: None,
            moves: Vec::new(),
        };

        for line in splitted_data {
            match version_re.captures(line) {
                Some(caps) => {
                    let version = &caps["version"];
                    csa_file.version = Some(version.to_string());
                    continue;
                }
                None => {}
            }

            match sente_re.captures(line) {
                Some(caps) => {
                    let sente = &caps["sente"];
                    csa_file.sente = Some(sente.to_string());
                    continue;
                }
                None => {}
            }

            match gote_re.captures(line) {
                Some(caps) => {
                    let sente = &caps["gote"];
                    csa_file.gote = Some(sente.to_string());
                    continue;
                }
                None => {}
            }
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

                    let teban = match &caps["teban"] {
                        "+" => board::Teban::Sente,
                        "-" => board::Teban::Gote,
                        _ => board::Teban::Sente,
                    };

                    let koma_move = move_koma::Move {
                        prev_pos: prev_pos,
                        next_pos: next_pos,
                        piece_type: piece_type::PieceType::from_str(piece_type).unwrap(),
                        teban: teban,
                    };
                    csa_file.moves.push(koma_move);

                    continue;
                }
                None => {}
            }
        }

        csa_file_vector.push(csa_file)
    }

    return csa_file_vector;
}
