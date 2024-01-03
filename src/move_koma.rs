
use crate::board;
use crate::piece_type;
use crate::position;
use crate::neural;


#[derive(Debug)]
pub struct Move {
    pub prev_pos: position::Position,
    pub next_pos: position::Position,
    pub piece_type: piece_type::PieceType,
}