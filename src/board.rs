use std::collections::HashMap;
use std::str::FromStr;

use crate::piece_type;
use crate::position;

#[derive(Debug)]
pub struct Board {
    sente_board: HashMap<position::Position, piece_type::PieceType>,
    gote_board: HashMap<position::Position, piece_type::PieceType>,
}

impl Board {
    pub fn pprint(&self) {
        let mut position_string_vector: Vec<String> = Vec::new();

        for col_num in (1..=9).rev() {
            for row_num in 1..=9 {
                let position_key = format!("{}{}", col_num, row_num);
                let position_value = position::Position::from_str(&position_key).unwrap();
            }
        }
    }
}

pub fn initialize_board() -> Board {
    let mut board = Board {
        sente_board: HashMap::new(),
        gote_board: HashMap::new(),
    };

    board
        .gote_board
        .insert(position::Position::SQ_1A, piece_type::PieceType::Lance);
    board
        .gote_board
        .insert(position::Position::SQ_2A, piece_type::PieceType::Knight);
    board
        .gote_board
        .insert(position::Position::SQ_3A, piece_type::PieceType::Silver);
    board
        .gote_board
        .insert(position::Position::SQ_4A, piece_type::PieceType::Gold);
    board
        .gote_board
        .insert(position::Position::SQ_5A, piece_type::PieceType::King);
    board
        .gote_board
        .insert(position::Position::SQ_6A, piece_type::PieceType::Gold);
    board
        .gote_board
        .insert(position::Position::SQ_7A, piece_type::PieceType::Silver);
    board
        .gote_board
        .insert(position::Position::SQ_7A, piece_type::PieceType::Knight);
    board
        .gote_board
        .insert(position::Position::SQ_7A, piece_type::PieceType::Lance);

    board
        .sente_board
        .insert(position::Position::SQ_1I, piece_type::PieceType::Lance);
    board
        .sente_board
        .insert(position::Position::SQ_2I, piece_type::PieceType::Knight);
    board
        .sente_board
        .insert(position::Position::SQ_3I, piece_type::PieceType::Silver);
    board
        .sente_board
        .insert(position::Position::SQ_4I, piece_type::PieceType::Gold);
    board
        .sente_board
        .insert(position::Position::SQ_5I, piece_type::PieceType::King);
    board
        .sente_board
        .insert(position::Position::SQ_6I, piece_type::PieceType::Gold);
    board
        .sente_board
        .insert(position::Position::SQ_7I, piece_type::PieceType::Silver);
    board
        .sente_board
        .insert(position::Position::SQ_7I, piece_type::PieceType::Knight);
    board
        .sente_board
        .insert(position::Position::SQ_7I, piece_type::PieceType::Lance);

    return board;
}
