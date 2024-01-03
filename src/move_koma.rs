
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


impl Move {
    pub fn csa_move_to_move_direction(&self) -> MoveDirection {


        return MoveDirection::DOWN
    }
}


pub enum MoveDirection {
    UP = 0,
    UpLeft,
    UpRight,
    LEFT,
    RIGHT,
    DOWN,
    DownLeft,
    DownRight,
    Up2Left,
    Up2Right,
    UpPromote,
    UpLeftPromote,
    UpRightPromote,
    LeftPromote,
    RightPromote,
    DownPromote,
    DownLeftPromote,
    DownRightPromote,
    Up2LeftPromote,
    Up2RightPromote
}



// # 移動方向を表す定数
// MOVE_DIRECTION = [

//     UP,
//     UP_LEFT,
//     UP_RIGHT,
//     LEFT,
//     RIGHT,
//     DOWN,
//     DOWN_LEFT,
//     DOWN_RIGHT,
//     UP2_LEFT,
//     UP2_RIGHT,
//     UP_PROMOTE,
//     UP_LEFT_PROMOTE,
//     UP_RIGHT_PROMOTE,
//     LEFT_PROMOTE,
//     RIGHT_PROMOTE,
//     DOWN_PROMOTE,
//     DOWN_LEFT_PROMOTE,
//     DOWN_RIGHT_PROMOTE,
//     UP2_LEFT_PROMOTE,
//     UP2_RIGHT_PROMOTE,
// ] = range(20)