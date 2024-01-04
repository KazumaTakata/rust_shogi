use std::vec;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};

use crate::board;
use crate::csa;
use crate::neural;
use crate::piece_type;
use crate::position;

pub fn load_dataset() -> (Vec<Tensor>, Vec<Tensor>) {
    let mut label_tensors: Vec<Tensor> = Vec::new();
    let mut input_tensors: Vec<Tensor> = Vec::new();

    let csa_file = csa::parse_csa_file();
    let mut board = board::initialize_board();

    let input_tensor = board.to_tensor();

    // println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

    let mut debug_count = 0;

    for next_move in csa_file.moves.iter() {

        println!("next move {:?}", next_move);
        let label = next_move.to_label_tensor();


        let vector = label.to_vec1::<f32>().unwrap();

        // println!("label: {:?}", label);

        label_tensors.push(label);

        let input_tensor = board.to_tensor();

        debug_count = debug_count + 1;


        // board.pprint_board(&input_tensor);

        input_tensors.push(input_tensor);

        board.pprint();

        board = board.move_koma(&next_move);
    }

    return (input_tensors, label_tensors);

    // let input_tensor = Tensor::rand(-1.0f32, 1.0, (1, 1, 28, 28), &Device::Cpu).unwrap();
}

pub fn train_neuralnet() {}
