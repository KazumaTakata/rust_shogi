use std::vec;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};

use crate::board;
use crate::csa;
use crate::neural;
use crate::piece_type;
use crate::position;
use crate::dataload::{self, DataLoader};


pub fn train_neuralnet() {

    let dataloader = DataLoader::new(8);
    let dataloader = dataloader.load();
    let dataloader = dataloader.shuffle();

    let (batch_data, dataloader) = dataloader.get_batch();
    



    let (input_tensors, label_tensors) = dataload::load_dataset();

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

        let maxindex = logits.argmax(D::Minus1).unwrap();
        println!("maxindex: {}", maxindex);

        let log_sm_unsqueeze = log_sm.unsqueeze(0).unwrap();
        let label_tensors_unsqueeze = label_tensors[0].unsqueeze(0).unwrap();

        println!("tensor shape: {:?}", log_sm_unsqueeze.shape().dims());
        println!("tensor shape: {:?}", label_tensors[0].shape().dims());

        let loss = loss::nll(&log_sm_unsqueeze, &label_tensors[0]).unwrap();

        opt.backward_step(&loss);

        println!("{}", loss.to_vec0::<f32>().unwrap());
    }
}
