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

        for row_num in 1..=9 {
            for col_num in (1..=9).rev() {
                let position_key = format!("{}{}", col_num, row_num);
                let position_value = position::Position::from_str(&position_key).unwrap();

                match self.gote_board.get(&position_value) {
                    Some(value) => {
                        print!("-{}", value);
                        continue;
                    }
                    _ => {}
                }

                match self.sente_board.get(&position_value) {
                    Some(value) => {
                        print!("+{}", value);
                        continue;
                    }
                    _ => {
                        print!(" * ");
                        continue;
                    }
                }
            }
            print!("\n")
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
        .insert(position::Position::SQ_8A, piece_type::PieceType::Knight);
    board
        .gote_board
        .insert(position::Position::SQ_9A, piece_type::PieceType::Lance);

    board
        .gote_board
        .insert(position::Position::SQ_8B, piece_type::PieceType::Bishop);
    board
        .gote_board
        .insert(position::Position::SQ_2B, piece_type::PieceType::Rook);

    board
        .gote_board
        .insert(position::Position::SQ_1C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_2C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_3C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_4C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_5C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_6C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_7C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_8C, piece_type::PieceType::Pawn);
    board
        .gote_board
        .insert(position::Position::SQ_9C, piece_type::PieceType::Pawn);

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
        .insert(position::Position::SQ_8I, piece_type::PieceType::Knight);
    board
        .sente_board
        .insert(position::Position::SQ_9I, piece_type::PieceType::Lance);

    board
        .sente_board
        .insert(position::Position::SQ_8H, piece_type::PieceType::Bishop);
    board
        .sente_board
        .insert(position::Position::SQ_2H, piece_type::PieceType::Rook);

    board
        .sente_board
        .insert(position::Position::SQ_1G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_2G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_3G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_4G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_5G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_6G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_7G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_8G, piece_type::PieceType::Pawn);
    board
        .sente_board
        .insert(position::Position::SQ_9G, piece_type::PieceType::Pawn);

    return board;
}
