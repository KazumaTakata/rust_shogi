use candle_core::{DType, Device, Tensor};

use crate::board;
use crate::neural;
use crate::piece_type;
use crate::position;

#[derive(Debug)]
pub struct Move {
    pub prev_pos: position::Position,
    pub next_pos: position::Position,
    pub piece_type: piece_type::PieceType,
}

impl Move {
    fn to_label_tensor(&self) -> Tensor {
        let (next_x, next_y) = self.next_pos.to_tensor_index();
        let mut index = 20 * (next_x + next_y * 9);

        let move_direction = self.csa_move_to_move_direction();
        index = index + (move_direction as i32);

        let tensor = Tensor::zeros(81 * 20, DType::F32, &Device::Cpu).unwrap();

        return tensor;

    }

    fn csa_move_to_move_direction(&self) -> MoveDirection {
        let (next_x, next_y) = self.next_pos.to_tensor_index();
        let (prev_x, prev_y) = self.prev_pos.to_tensor_index();

        let diff_x = next_x - prev_x;
        let diff_y = next_y - prev_y;

        if diff_x == 0 && diff_y < 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::RightPromote;
            }

            return MoveDirection::RIGHT;
        }

        if diff_x == 0 && diff_y > 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::LeftPromote;
            }

            return MoveDirection::LEFT;
        }

        if diff_x > 0 && diff_y == 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::DownPromote;
            }

            return MoveDirection::DOWN;
        }

        if diff_x < 0 && diff_y == 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::UpPromote;
            }

            return MoveDirection::UP;
        }

        if diff_x < 0 && diff_y < 0 && diff_x == diff_y {
            if self.piece_type.is_promoted() {
                return MoveDirection::UpRightPromote;
            }

            return MoveDirection::UpRight;
        }

        if diff_x > 0 && diff_y > 0 && diff_x == diff_y {
            if self.piece_type.is_promoted() {
                return MoveDirection::DownLeftPromote;
            }

            return MoveDirection::DownLeft;
        }

        if diff_x < 0 && diff_y > 0 && diff_x == diff_y {
            if self.piece_type.is_promoted() {
                return MoveDirection::DownRightPromote;
            }

            return MoveDirection::DownRight;
        }

        if diff_x > 0 && diff_y < 0 && diff_x == diff_y {
            if self.piece_type.is_promoted() {
                return MoveDirection::UpLeftPromote;
            }

            return MoveDirection::UpLeft;
        }

        if diff_x == 1 && diff_y == -2 {
            if self.piece_type.is_promoted() {
                return MoveDirection::Up2LeftPromote;
            }

            return MoveDirection::Up2Left;
        }

        if diff_x == -1 && diff_y == -2 {
            if self.piece_type.is_promoted() {
                return MoveDirection::Up2RightPromote;
            }

            return MoveDirection::Up2Right;
        }

        return MoveDirection::DOWN;
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
    Up2RightPromote,
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
