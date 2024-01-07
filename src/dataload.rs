use core::slice;
use std::vec;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};
use rand::seq::SliceRandom;

use crate::board;
use crate::csa;
use crate::neural;
use crate::piece_type;
use crate::position;

use std::io::{stdin, Read};

#[derive(Debug)]
pub struct DataLoader {
    input_tensors: Vec<Tensor>,
    label_tensors: Vec<Tensor>,
    batch_index: usize,
    batch_size: usize,
}

impl DataLoader {
    pub fn new(batch_size: usize) -> Self {
        Self {
            input_tensors: Vec::new(),
            label_tensors: Vec::new(),
            batch_index: 0,
            batch_size: batch_size,
        }
    }

    pub fn shuffle(mut self) -> Self {
        let mut rng = rand::thread_rng();
        self.input_tensors.shuffle(&mut rng);
        self.label_tensors.shuffle(&mut rng);
        return self;
    }

    pub fn get_batch(mut self) -> (Tensor, Self) {
        println!("self.batch_index: {}", self.batch_index);
        println!(
            "self.batch_index+self.batch_size: {}",
            self.batch_index + self.batch_size
        );

        let sliced_input_tensor =
            &self.input_tensors[self.batch_index..(self.batch_index + self.batch_size)];

        let concat_tensor = Tensor::cat(sliced_input_tensor, 0).unwrap();

        self.batch_index += self.batch_size;

        return (concat_tensor, self);
    }

    pub fn load(mut self) -> Self {
        self.input_tensors = Vec::new();
        self.label_tensors = Vec::new();

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

                let label = next_move.to_label_tensor_2();

                // println!("label: {:?}", label);

                self.label_tensors.push(label);

                debug_count = debug_count + 1;

                board = board.move_koma(&next_move);

                let input_tensor = board.to_tensor();

                // board.pprint_board(&input_tensor);

                self.input_tensors.push(input_tensor);

                // board.pprint();
            }
        }

        return self;
    }
}

pub fn load_dataset() -> (Vec<Tensor>, Vec<Tensor>) {
    let mut label_tensors: Vec<Tensor> = Vec::new();
    let mut input_tensors: Vec<Tensor> = Vec::new();

    let csa_file_vector = csa::parse_csa_file();

    // println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

    let mut debug_count = 0;

    let mut progress = 0;

    for csa_file in csa_file_vector {
        progress += 1;
        if progress % 10 == 0 {
            println!("dataload progress: {}", progress)
        }

        let mut board = board::initialize_board();
        for next_move in csa_file.moves.iter() {
            let mut stdin_handle = stdin().lock();
            let mut byte = [0_u8];

            // stdin_handle.read_exact(&mut byte).unwrap();

            // print!("\x1B[2J\x1B[1;1H");

            // println!("next move {:?}", next_move);

            let label = next_move.to_label_tensor_2();

            // println!("label: {:?}", label);

            label_tensors.push(label);

            debug_count = debug_count + 1;

            board = board.move_koma(&next_move);

            let input_tensor = board.to_tensor();

            // board.pprint_board(&input_tensor);

            input_tensors.push(input_tensor);

            // board.pprint();
        }
    }

    return (input_tensors, label_tensors);

    // let input_tensor = Tensor::rand(-1.0f32, 1.0, (1, 1, 28, 28), &Device::Cpu).unwrap();
}
