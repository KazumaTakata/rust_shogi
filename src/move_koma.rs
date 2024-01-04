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
    pub fn to_label_tensor(&self) -> Tensor {
        let (next_x, next_y) = self.next_pos.to_tensor_index();

        println!("{}, {}", next_x, next_y);

        let base_index = 27 * ((next_x - 1) + (next_y - 1) * 9);

        let move_direction: MoveDirection = self.csa_move_to_move_direction();

        println!("move_direction {:?}", &move_direction);

        let index = base_index + (move_direction as i32);

        let mut zero_vec: Vec<f32> = vec![0.0; 81 * 27];
        zero_vec[index as usize] = 1.0;

        let tensor = Tensor::from_vec(zero_vec, 81 * 27, &Device::Cpu).unwrap();

        return tensor;
    }

    fn csa_move_to_move_direction(&self) -> MoveDirection {
        let (next_x, next_y) = self.next_pos.to_tensor_index();
        let (prev_x, prev_y) = self.prev_pos.to_tensor_index();

        if prev_x == 0 && prev_y == 0 {
            match &self.piece_type {
                piece_type::PieceType::Rook => MoveDirection::Rook,
                piece_type::PieceType::Bishop => MoveDirection::Bishop,
                piece_type::PieceType::Gold => MoveDirection::Gold,
                piece_type::PieceType::Silver => MoveDirection::Silver,
                piece_type::PieceType::Knight => MoveDirection::Knight,
                piece_type::PieceType::Lance => MoveDirection::Lance,
                piece_type::PieceType::Pawn => MoveDirection::Pawn,
                _ => MoveDirection::Pawn,
            };
        };

        let diff_x = next_x - prev_x;
        let diff_y = next_y - prev_y;

        if diff_x == 0 && diff_y < 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::UpPromote;
            }

            return MoveDirection::UP;
        }

        if diff_x == 0 && diff_y > 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::DownPromote;
            }

            return MoveDirection::DOWN;
        }

        if diff_x > 0 && diff_y == 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::LeftPromote;
            }

            return MoveDirection::LEFT;
        }

        if diff_x < 0 && diff_y == 0 {
            if self.piece_type.is_promoted() {
                return MoveDirection::RightPromote;
            }

            return MoveDirection::RIGHT;
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

#[derive(Debug)]
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
    Rook,
    Bishop,
    Gold,
    Silver,
    Knight,
    Lance,
    Pawn,
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
