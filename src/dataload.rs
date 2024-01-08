use core::slice;
use std::vec;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};
use rand::seq::SliceRandom;
use termion::input;

use crate::board::{self, Teban};
use crate::csa;
use crate::neural;
use crate::piece_type;
use crate::position;

use std::io::{stdin, Read};

#[derive(Debug, Clone)]
pub struct DataLoader {
    input_tensors: Vec<(Tensor, Tensor)>,
    batch_size: usize,
}

impl DataLoader {
    pub fn new(batch_size: usize) -> Self {
        Self {
            input_tensors: Vec::new(),
            batch_size: batch_size,
        }
    }

    pub fn shuffle(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.input_tensors.shuffle(&mut rng);
        return self;
    }

    pub fn get_batch(&self, start_index: usize) -> (Tensor, Tensor, usize) {
        let end_index = start_index + self.batch_size;

        // println!("start_index: {}", start_index);
        // println!("end_index: {}", end_index);

        let sliced_input_tensor = &self.input_tensors[start_index..end_index];

        let mut input_tensors = Vec::new();
        let mut label_tensors = Vec::new();

        for tensor_tuple in sliced_input_tensor {
            input_tensors.push(&tensor_tuple.0);
            label_tensors.push(&tensor_tuple.1);
        }

        let concat_tensor = Tensor::stack(&input_tensors, 0).unwrap();
        let concat_label_tensor = Tensor::stack(&label_tensors, 0).unwrap();

        let concat_label_tensor = concat_label_tensor.flatten_all().unwrap();

        return (concat_tensor, concat_label_tensor, end_index);
    }

    pub fn load(mut self) -> Self {
        self.input_tensors = Vec::new();

        let csa_file_vector = csa::parse_csa_file();

        // println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

        let mut debug_count = 0;

        let mut progress = 0;

        for csa_file in csa_file_vector {
            progress += 1;
            if progress % 10 == 0 {
                println!("dataload progress: {}", progress);
                break;
            }

            let mut board = board::initialize_board();
            for next_move in csa_file.moves.iter() {
                let mut stdin_handle = stdin().lock();
                let mut byte = [0_u8];

                // stdin_handle.read_exact(&mut byte).unwrap();

                // print!("\x1B[2J\x1B[1;1H");

                // println!("next move {:?}", next_move);

                let label = next_move.to_label_tensor_2(&next_move.teban);
                let input_tensor = board.to_tensor(&next_move.teban);
                // println!("label: {:?}", label);

                board = board.move_koma(&next_move);

                // board.pprint_board(&input_tensor);
                self.input_tensors.push((input_tensor, label));

                // board.pprint();
            }
        }

        return self;
    }
}



// pub fn load_dataset() -> (Vec<Tensor>, Vec<Tensor>) {
//     let mut label_tensors: Vec<Tensor> = Vec::new();
//     let mut input_tensors: Vec<Tensor> = Vec::new();

//     let csa_file_vector = csa::parse_csa_file();

//     // println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

//     let mut debug_count = 0;

//     let mut progress = 0;

//     for csa_file in csa_file_vector {
//         progress += 1;
//         if progress % 10 == 0 {
//             println!("dataload progress: {}", progress)
//         }

//         let mut board = board::initialize_board();
//         for next_move in csa_file.moves.iter() {
//             let mut stdin_handle = stdin().lock();
//             let mut byte = [0_u8];

//             // stdin_handle.read_exact(&mut byte).unwrap();

//             // print!("\x1B[2J\x1B[1;1H");

//             // println!("next move {:?}", next_move);

//             let label = next_move.to_label_tensor_2();

//             // println!("label: {:?}", label);

//             label_tensors.push(label);

//             debug_count = debug_count + 1;

//             board = board.move_koma(&next_move);

//             let input_tensor = board.to_tensor();

//             // board.pprint_board(&input_tensor);

//             input_tensors.push(input_tensor);

//             // board.pprint();
//         }
//     }

//     return (input_tensors, label_tensors);

//     // let input_tensor = Tensor::rand(-1.0f32, 1.0, (1, 1, 28, 28), &Device::Cpu).unwrap();
// }
