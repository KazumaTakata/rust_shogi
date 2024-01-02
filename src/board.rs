use std::collections::HashMap;

use crate::position;
use crate::piece_type;




#[derive(Debug)]
pub struct Board {
    sente_board: HashMap<position::Position, piece_type::PieceType>,
    gote_board: HashMap<position::Position, piece_type::PieceType>,
}

pub fn initialize_board() -> Board {
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

