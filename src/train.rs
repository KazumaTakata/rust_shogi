use std::vec;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};

use crate::board;
use crate::csa;
use crate::dataload::{self, DataLoader};
use crate::neural;
use crate::piece_type;
use crate::position;

pub fn train_neuralnet() {

    let device_type = Device::cuda_if_available(0).unwrap();

    let dataloader = DataLoader::new(8);
    let dataloader = dataloader.load(&device_type);

    // let dataloader = dataloader.shuffle();

    // let (input_tensors, label_tensors) = dataload::load_dataset();

    let mut varmap = VarMap::new();
    let vs = VarBuilder::from_varmap(&varmap, DType::F32, &device_type);

    let model = neural::Resnet::new(vs.clone()).unwrap();

    let mut opt = candle_nn::SGD::new(varmap.all_vars(), 0.0).unwrap();

    let adamw_params = candle_nn::ParamsAdamW {
        lr: 0.01,
        ..Default::default()
    };

    let mut opt = candle_nn::AdamW::new(varmap.all_vars(), adamw_params).unwrap();

    // let input_tensor = input_tensors[0].unsqueeze(0).unwrap();

    let mut start_index = 0;

    let mut sum_loss = 0.0;

    loop {
        let (input_tensor, label_tensor, _start_index) = dataloader.get_batch(start_index);
        start_index = _start_index;

        if start_index == 8 * 10 {
            start_index = 0;


            println!("sum_loss: {}", sum_loss);
            sum_loss = 0.0;
        }

        // println!("input_tensor shape: {:?}", input_tensor.shape().dims());

        // println!("input_tensor shape: {:?}", input_tensor.shape().dims());

        let logits = model.forward(&input_tensor, true).unwrap();

        // println!("logits shape: {:?}", logits.shape().dims());

        let log_sm = ops::log_softmax(&logits, D::Minus1).unwrap();

        // println!("log_sm shape: {:?}", log_sm.shape().dims());

        // println!(" label_tensor shape: {:?}", label_tensor.shape().dims());

        // println!(" label_tensor: {:?}", label_tensor.to_vec1::<u32>());

        let maxindex = logits.argmax(D::Minus1).unwrap();

        // println!("maxindex: {}", maxindex);

        // println!("tensor shape: {:?}", log_sm.shape().dims());
        // println!("tensor shape: {:?}", label_tensor.shape().dims());

        let loss = loss::nll(&log_sm, &label_tensor).unwrap();

        // println!("loss shape: {:?}", loss.shape().dims());

        opt.backward_step(&loss);

        // println!("{}", loss.to_vec0::<f32>().unwrap());

        sum_loss += loss.to_vec0::<f32>().unwrap();
    }
}
