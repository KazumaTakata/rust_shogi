use core::slice;
use std::vec;

use candle_core::{DType, Device, Result, Tensor, D};
use candle_nn::batch_norm;
use candle_nn::{loss, ops, Conv2d, Linear, Module, ModuleT, Optimizer, VarBuilder, VarMap};
use rand::seq::SliceRandom;
use termion::input;

use crate::board::{self, Teban};
use crate::csa::{self, CSAFile};
use crate::neural;
use crate::piece_type;
use crate::position;

use std::io::{stdin, Read};

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::SyncSender<Message>,
    result: Arc<Mutex<Vec<(Tensor, Tensor)>>>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Message {
    message: String,
    csa_file: CSAFile,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize, device_type: Device) -> ThreadPool {
        assert!(size > 0);

        let input_tensor = Arc::new(Mutex::new(Vec::new()));

        let (sender, receiver) = mpsc::sync_channel(0);

        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(
                id,
                Arc::clone(&receiver),
                Arc::clone(&input_tensor),
                device_type.clone(),
            ));
        }

        ThreadPool {
            workers: workers,
            sender: sender,
            result: input_tensor,
        }
    }
    // --snip--
}

// --snip--

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        result: Arc<Mutex<Vec<(Tensor, Tensor)>>>,
        device_type: Device,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(message) => {
                    let csa_file = message.csa_file;

                    let mut temp_input_tensor = vec![];

                    let mut board = board::initialize_board();
                    for next_move in csa_file.moves.iter() {
                        let label = next_move.to_label_tensor_2(&next_move.teban, &device_type);
                        let input_tensor = board.to_tensor(&next_move.teban, &device_type);
                        // println!("label: {:?}", label);

                        board = board.move_koma(&next_move);

                        temp_input_tensor.push((input_tensor, label));

                        // board.pprint();
                        // board.pprint_board(&input_tensor);
                    }

                    result.lock().unwrap().append(&mut temp_input_tensor);
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down.");
                    break;
                }
            }
        });

        Worker { id, thread }
    }
}

#[derive(Debug, Clone)]
pub struct DataLoader {
    pub input_tensors: Vec<(Tensor, Tensor)>,
    batch_size: usize,
    device_type: Device,
}

impl DataLoader {
    pub fn new(batch_size: usize, device_type: Device) -> Self {
        Self {
            input_tensors: Vec::new(),
            batch_size: batch_size,
            device_type: device_type,
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

    pub fn load(mut self, device_type: Device) -> Self {
        let thread_pool = ThreadPool::new(8, device_type);

        let csa_file_vector = csa::parse_csa_file();

        // println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

        let mut progress = 0;

        for csa_file in csa_file_vector {
            let message = Message {
                csa_file: csa_file,
                message: "message1".to_string(),
            };

            thread_pool.sender.send(message).unwrap();

            progress += 1;
            println!("progress: {}", progress);

            // if progress % 10 == 0 {
            //     println!("dataload progress: {}", progress);
            //     break;
            // }
        }

        
        drop(thread_pool.sender);

        for worker in thread_pool.workers {
            worker.thread.join().unwrap();
        }
        


        self.input_tensors = Arc::try_unwrap(thread_pool.result)
            .unwrap()
            .into_inner()
            .unwrap();

        return self;
    }
}
