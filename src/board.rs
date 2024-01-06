use std::collections::HashMap;
use std::str::FromStr;

use crate::move_koma;
use crate::piece_type;
use crate::position;
use candle_core::IndexOp;
use candle_core::{DType, Device, Result, Tensor, D};

#[derive(Debug)]
pub struct Board {
    sente_board: HashMap<position::Position, piece_type::PieceType>,
    gote_board: HashMap<position::Position, piece_type::PieceType>,
    sente_komadai: KomaDai,
    gote_komadai: KomaDai,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Teban {
    Sente,
    Gote,
}

// hu, ky, ke, gi, ki, ka, hi
#[derive(Debug)]
pub struct KomaDai {
    hu: i8,
    ky: i8,
    ke: i8,
    gi: i8,
    ki: i8,
    ka: i8,
    hi: i8,
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

        println!("{:?}", self.sente_komadai);
        println!("{:?}", self.gote_komadai);


    }

    pub fn move_koma(mut self, move_koma: &move_koma::Move) -> Board {
        if move_koma.teban == Teban::Gote {
            if move_koma.prev_pos == position::Position::SQ_00 {
                match move_koma.piece_type {
                    piece_type::PieceType::Bishop => self.gote_komadai.ka -= 1,
                    piece_type::PieceType::Rook => self.gote_komadai.hi -= 1,
                    piece_type::PieceType::Gold => self.gote_komadai.ki -= 1,
                    piece_type::PieceType::Silver => self.gote_komadai.gi -= 1,
                    piece_type::PieceType::Knight => self.gote_komadai.ke -= 1,
                    piece_type::PieceType::Lance => self.gote_komadai.ky -= 1,
                    piece_type::PieceType::Pawn => self.gote_komadai.hu -= 1,
                    _ => {}
                }
                let copied_value = move_koma.piece_type.clone();
                self.gote_board
                    .insert(move_koma.next_pos.clone(), move_koma.piece_type.clone());

                return self;
            }
            match self.gote_board.get(&move_koma.prev_pos) {
                Some(value) => {
                    self.gote_board.remove(&move_koma.prev_pos);

                    //駒を取った時
                    match self.sente_board.get(&move_koma.next_pos) {
                        Some(value) => match value {
                            &piece_type::PieceType::Bishop => self.gote_komadai.ka += 1,
                            &piece_type::PieceType::ProBishop => self.gote_komadai.ka += 1,

                            &piece_type::PieceType::Rook => self.gote_komadai.hi += 1,
                            &piece_type::PieceType::ProRook => self.gote_komadai.hi += 1,

                            &piece_type::PieceType::Gold => self.gote_komadai.ki += 1,
 
                            &piece_type::PieceType::Silver => self.gote_komadai.gi += 1,
                            &piece_type::PieceType::ProSilver => self.gote_komadai.gi += 1,

                            &piece_type::PieceType::Knight => self.gote_komadai.ke += 1,
                            &piece_type::PieceType::ProKnight => self.gote_komadai.ke += 1,

                            &piece_type::PieceType::Lance => self.gote_komadai.ky += 1,
                            &piece_type::PieceType::ProLance => self.gote_komadai.ky += 1,

                            &piece_type::PieceType::Pawn => self.gote_komadai.hu += 1,
                            &piece_type::PieceType::ProPawn => self.gote_komadai.hu += 1,

                            _ => {}
                        },
                        _ => {}
                    }

                    self.sente_board.remove(&move_koma.next_pos);

                    self.gote_board
                        .insert(move_koma.next_pos.clone(), move_koma.piece_type.clone());
                    return self;
                }
                _ => {}
            }
        }

        if move_koma.teban == Teban::Sente {
            if move_koma.prev_pos == position::Position::SQ_00 {
                match move_koma.piece_type {
                    piece_type::PieceType::Bishop => self.sente_komadai.ka -= 1,
                    piece_type::PieceType::Rook => self.sente_komadai.hi -= 1,
                    piece_type::PieceType::Gold => self.sente_komadai.ki -= 1,
                    piece_type::PieceType::Silver => self.sente_komadai.gi -= 1,
                    piece_type::PieceType::Knight => self.sente_komadai.ke -= 1,
                    piece_type::PieceType::Lance => self.sente_komadai.ky -= 1,
                    piece_type::PieceType::Pawn => self.sente_komadai.hu -= 1,
                    _ => {}
                }
                let copied_value = move_koma.piece_type.clone();
                self.sente_board
                    .insert(move_koma.next_pos.clone(), copied_value);

                return self;
            }

            match self.sente_board.get(&move_koma.prev_pos) {
                Some(value) => {
                    self.sente_board.remove(&move_koma.prev_pos);

                    //駒を取った時
                    match self.gote_board.get(&move_koma.next_pos) {
                        Some(value) => match value {
                            &piece_type::PieceType::Bishop => self.sente_komadai.ka += 1,
                            &piece_type::PieceType::ProBishop => self.sente_komadai.ka += 1,

                            &piece_type::PieceType::Rook => self.sente_komadai.hi += 1,
                            &piece_type::PieceType::ProRook => self.sente_komadai.hi += 1,

                            &piece_type::PieceType::Gold => self.sente_komadai.ki += 1,
 
                            &piece_type::PieceType::Silver => self.sente_komadai.gi += 1,
                            &piece_type::PieceType::ProSilver => self.sente_komadai.gi += 1,

                            &piece_type::PieceType::Knight => self.sente_komadai.ke += 1,
                            &piece_type::PieceType::ProKnight => self.sente_komadai.ke += 1,

                            &piece_type::PieceType::Lance => self.sente_komadai.ky += 1,
                            &piece_type::PieceType::ProLance => self.sente_komadai.ky += 1,

                            &piece_type::PieceType::Pawn => self.sente_komadai.hu += 1,
                            &piece_type::PieceType::ProPawn => self.sente_komadai.hu += 1,


                            _ => {}
                        },
                        _ => {}
                    };

                    self.gote_board.remove(&move_koma.next_pos);

                    self.sente_board
                        .insert(move_koma.next_pos.clone(), move_koma.piece_type.clone());
                    return self;
                }
                _ => {}
            }
        }
        return self;
    }

    fn to_tensor_channel_index(&self, piece_type: &piece_type::PieceType) -> i32 {
        // channel order
        // on_board = 14
        // hu, ky, ke, gi, ki, ka, hi, ou, pro_hu, pro_ky, pro_ke, pro_gi, pro_ka, pro_hi
        // on komadai = 38
        // hu * 18, ky * 4, ke * 4, gi * 4, ki * 4, ka * 2, Hi * 2

        match piece_type {
            piece_type::PieceType::Pawn => 0,
            piece_type::PieceType::Lance => 1,
            piece_type::PieceType::Knight => 2,
            piece_type::PieceType::Silver => 3,
            piece_type::PieceType::Gold => 4,
            piece_type::PieceType::Bishop => 5,
            piece_type::PieceType::Rook => 6,
            piece_type::PieceType::King => 7,

            piece_type::PieceType::ProPawn => 8,
            piece_type::PieceType::ProLance => 9,
            piece_type::PieceType::ProKnight => 10,
            piece_type::PieceType::ProSilver => 11,
            piece_type::PieceType::ProBishop => 12,
            piece_type::PieceType::ProRook => 13,
        }
    }

    pub fn to_tensor(&self) -> Tensor {
        let board_tensor = self.board_to_tensor(Teban::Sente);

        let komadai_tensor = self.komadai_to_tensor(Teban::Sente);

        let sente_tensor = Tensor::cat(&[&board_tensor, &komadai_tensor], 0).unwrap();

        let board_tensor = self.board_to_tensor(Teban::Gote);

        let komadai_tensor = self.komadai_to_tensor(Teban::Gote);

        let gote_tensor = Tensor::cat(&[&board_tensor, &komadai_tensor], 0).unwrap();

        let tensor = Tensor::cat(&[&sente_tensor, &gote_tensor], 0).unwrap();

        println!("tensor shape11: {:?}", tensor.shape().dims3());

        return tensor;
    }

    pub fn pprint_board(&self, input_tensor: &Tensor) {

        // channel order
        // on_board = 14
        // hu, ky, ke, gi, ki, ka, hi, ou, pro_hu, pro_ky, pro_ke, pro_gi, pro_ka, pro_hi
        // on komadai = 38
        // hu * 18, ky * 4, ke * 4, gi * 4, ki * 4, ka * 2, Hi * 2

        println!("先手:歩");
        let input_vector = input_tensor.to_vec3::<f32>().unwrap();
        let pawn_vectors = &input_vector[0];

        for row_vectors in pawn_vectors {
            for bit_value in row_vectors.iter().rev()  {
                print!(" {} ", bit_value)
            }
            println!("");
        }
        println!("");

        println!("先手:香");
        let pawn_vectors = &input_vector[1];
        for row_vectors in pawn_vectors {
            for bit_value in row_vectors.iter().rev()  {
                print!(" {} ", bit_value)
            }
            println!("");
        }
        println!("");

        println!("先手:桂");
        let pawn_vectors = &input_vector[2];
        for row_vectors in pawn_vectors {
            for bit_value in row_vectors.iter().rev()  {
                print!(" {} ", bit_value)
            }
            println!("");
        }
        println!("");

        println!("先手:銀");
        let pawn_vectors = &input_vector[3];
        for row_vectors in pawn_vectors {
            for bit_value in row_vectors.iter().rev()  {
                print!(" {} ", bit_value)
            }
            println!("");
        }
        println!("");


        println!("先手:持歩1");
        let pawn_vectors = &input_vector[14];
        for row_vectors in pawn_vectors {
            for bit_value in row_vectors.iter().rev()  {
                print!(" {} ", bit_value)
            }
            println!("");
        }
        println!("");

        println!("先手:持歩2");
        let pawn_vectors = &input_vector[15];
        for row_vectors in pawn_vectors {
            for bit_value in row_vectors.iter().rev()  {
                print!(" {} ", bit_value)
            }
            println!("");
        }
        println!("");

        println!("先手:持歩3");
        let pawn_vectors = &input_vector[16];
        for row_vectors in pawn_vectors {
            for bit_value in row_vectors.iter().rev()  {
                print!(" {} ", bit_value)
            }
            println!("");
        }
        println!("");




    }

    fn board_to_tensor(&self, teban: Teban) -> Tensor {
        let on_board_channel_size = 14;

        let vec_size = on_board_channel_size * 9 * 9;

        let board = match teban {
            Teban::Sente => &self.sente_board,
            Teban::Gote => &self.gote_board,
        };

        let mut zero_vec: Vec<f32> = Vec::with_capacity(vec_size as usize);
        for i in 0..vec_size {
            zero_vec.push(0.0);
        }

        for (position, piece_type) in board.iter() {
            let (col, row) = position.to_tensor_index();

            let channel_index = self.to_tensor_channel_index(piece_type);
            let vector_index = 81 * channel_index + (col - 1) + (row - 1) * 9;

            zero_vec[vector_index as usize] = 1.0
        }

        let board_tensor =
            Tensor::from_vec(zero_vec, (on_board_channel_size, 9, 9), &Device::Cpu).unwrap();

        return board_tensor;
    }

    fn komadai_to_tensor(&self, teban: Teban) -> Tensor {
        // # hu < 18
        let mut hu_tensor: Tensor = Tensor::ones((1, 9, 9), DType::F32, &Device::Cpu).unwrap();

        let komadai = match teban {
            Teban::Gote => &self.gote_komadai,
            Teban::Sente => &self.sente_komadai,
        };

        let in_komadai_list = [
            (komadai.hu, 18),
            (komadai.ky, 4),
            (komadai.ke, 4),
            (komadai.gi, 4),
            (komadai.ki, 4),
            (komadai.ka, 2),
            (komadai.hi, 2),
        ];

        let mut j_flag = 0;

        for in_komadai in in_komadai_list {
            for i in 0..in_komadai.1 {
                if i == 0 && j_flag == 0 {
                    j_flag = 1;
                    if i < in_komadai.0 {
                        hu_tensor = Tensor::ones((1, 9, 9), DType::F32, &Device::Cpu).unwrap();
                    } else {
                        hu_tensor = Tensor::zeros((1, 9, 9), DType::F32, &Device::Cpu).unwrap();
                    }
                } else {
                    if i < in_komadai.0 {
                        hu_tensor = Tensor::cat(
                            &[
                                &hu_tensor,
                                &Tensor::ones((1, 9, 9), DType::F32, &Device::Cpu).unwrap(),
                            ],
                            0,
                        )
                        .unwrap();
                    } else {
                        hu_tensor = Tensor::cat(
                            &[
                                &hu_tensor,
                                &Tensor::zeros((1, 9, 9), DType::F32, &Device::Cpu).unwrap(),
                            ],
                            0,
                        )
                        .unwrap();
                    }
                }
            }
        }
        return hu_tensor;
    }
}

pub fn initialize_board() -> Board {
    let mut board = Board {
        sente_board: HashMap::new(),
        gote_board: HashMap::new(),
        sente_komadai: KomaDai {
            hu: 0,
            ky: 0,
            ke: 0,
            gi: 0,
            ki: 0,
            ka: 0,
            hi: 0,
        },
        gote_komadai: KomaDai {
            hu: 0,
            ky: 0,
            ke: 0,
            gi: 0,
            ki: 0,
            ka: 0,
            hi: 0,
        },
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
        .insert(position::Position::SQ_8B, piece_type::PieceType::Rook);
    board
        .gote_board
        .insert(position::Position::SQ_2B, piece_type::PieceType::Bishop);

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
