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

#[derive(Debug)]
struct Board {
    sente_board: HashMap<position::Position, piece_type::PieceType>,
    gote_board: HashMap<position::Position, piece_type::PieceType>,
}

fn initialize_board() -> Board {
    let mut board = Board {
        sente_board: HashMap::new(),
        gote_board: HashMap::new(),
    };

    board.gote_board.insert(position::Position::SQ_1A, piece_type::PieceType::Lance);
    board.gote_board.insert(position::Position::SQ_2A, piece_type::PieceType::Knight);
    board.gote_board.insert(position::Position::SQ_3A, piece_type::PieceType::Silver);
    board.gote_board.insert(position::Position::SQ_4A, piece_type::PieceType::Gold);
    board.gote_board.insert(position::Position::SQ_5A, piece_type::PieceType::King);
    board.gote_board.insert(position::Position::SQ_6A, piece_type::PieceType::Gold);
    board.gote_board.insert(position::Position::SQ_7A, piece_type::PieceType::Silver);
    board.gote_board.insert(position::Position::SQ_7A, piece_type::PieceType::Knight);
    board.gote_board.insert(position::Position::SQ_7A, piece_type::PieceType::Lance);


    board.sente_board.insert(position::Position::SQ_1I, piece_type::PieceType::Lance);
    board.sente_board.insert(position::Position::SQ_2I, piece_type::PieceType::Knight);
    board.sente_board.insert(position::Position::SQ_3I, piece_type::PieceType::Silver);
    board.sente_board.insert(position::Position::SQ_4I, piece_type::PieceType::Gold);
    board.sente_board.insert(position::Position::SQ_5I, piece_type::PieceType::King);
    board.sente_board.insert(position::Position::SQ_6I, piece_type::PieceType::Gold);
    board.sente_board.insert(position::Position::SQ_7I, piece_type::PieceType::Silver);
    board.sente_board.insert(position::Position::SQ_7I, piece_type::PieceType::Knight);
    board.sente_board.insert(position::Position::SQ_7I, piece_type::PieceType::Lance);


    return board;
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


    let board = initialize_board();

    println!("{:#?}", csa_file)

    // let sample = 1;

    // for num in 0..10 {
    //     println!("Hello, world!");
    // }
}
