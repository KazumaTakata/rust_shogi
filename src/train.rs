use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};

use crate::board;
use crate::piece_type;
use crate::position;
use crate::neural;



pub fn train_neuralnet() {
    let mut board = board::initialize_board();
    let input_tensor = board.to_tensor();
    println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

    
    let mut varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &Device::Cpu);
    let model = neural::Resnet::new(vs).unwrap();
    let y = model.forward(&input_tensor, true).unwrap();


}