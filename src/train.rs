use std::vec;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};

use crate::board;
use crate::csa;
use crate::neural;
use crate::piece_type;
use crate::position;

use std::io::{stdin, Read};

pub fn load_dataset() -> (Vec<Tensor>, Vec<Tensor>) {
    let mut label_tensors: Vec<Tensor> = Vec::new();
    let mut input_tensors: Vec<Tensor> = Vec::new();

    let csa_file = csa::parse_csa_file();
    let mut board = board::initialize_board();

    let input_tensor = board.to_tensor();

    // println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

    let mut debug_count = 0;

    for next_move in csa_file.moves.iter() {
        let mut stdin_handle = stdin().lock();
        let mut byte = [0_u8];

        // stdin_handle.read_exact(&mut byte).unwrap();

        print!("\x1B[2J\x1B[1;1H");

        println!("next move {:?}", next_move);
        let label = next_move.to_label_tensor_2();

        // println!("label: {:?}", label);

        label_tensors.push(label);

        debug_count = debug_count + 1;

        board = board.move_koma(&next_move);

        let input_tensor = board.to_tensor();
        board.pprint_board(&input_tensor);
        input_tensors.push(input_tensor);
        board.pprint();
    }

    return (input_tensors, label_tensors);

    // let input_tensor = Tensor::rand(-1.0f32, 1.0, (1, 1, 28, 28), &Device::Cpu).unwrap();
}

pub fn train_neuralnet() {
    let (input_tensors, label_tensors) = load_dataset();

    let mut varmap = VarMap::new();

    let adamw_params = candle_nn::ParamsAdamW {
        lr: 0.001,
        ..Default::default()
    };

    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &Device::Cpu);
    let model = neural::Resnet::new(vs.clone()).unwrap();

    let mut opt = candle_nn::AdamW::new(varmap.all_vars(), adamw_params).unwrap();

    let input_tensor = input_tensors[0].unsqueeze(0).unwrap();

    loop {
        let logits = model.forward(&input_tensor, true).unwrap();
        let log_sm = ops::log_softmax(&logits, D::Minus1).unwrap();

        let log_sm_unsqueeze = log_sm.unsqueeze(0).unwrap();
        let label_tensors_unsqueeze = label_tensors[0].unsqueeze(0).unwrap();

        println!("tensor shape: {:?}", log_sm_unsqueeze.shape().dims());
        println!("tensor shape: {:?}", label_tensors[0].shape().dims());

        let loss = loss::nll(&log_sm_unsqueeze, &label_tensors[0]).unwrap();

        opt.backward_step(&loss);

        println!("{}", loss.to_vec0::<f32>().unwrap());
    }
}
