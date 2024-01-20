mod board;
mod csa;
mod dataload;
mod move_koma;
mod neural;
mod piece_type;
mod position;
mod train;

use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
    result: Arc<Mutex<Vec<String>>>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

struct Message {
    message: String,
}

impl ThreadPool {
    // --snip--
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let x = Arc::new(Mutex::new(Vec::new()));

        let (sender, receiver) = mpsc::channel();

        let mut workers = Vec::with_capacity(size);

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver), Arc::clone(&x)));
        }

        ThreadPool {
            workers: workers,
            sender: sender,
            result: x,
        }
    }
    // --snip--
}

// --snip--

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
        result: Arc<Mutex<Vec<String>>>,
    ) -> Worker {
        let thread = thread::spawn(move || loop {
            while let Ok(job) = receiver.lock().unwrap().recv() {
                // println!("Worker {} got a job; executing.", job.message);
                result.lock().unwrap().push(job.message);

                for i in result.lock().unwrap().iter() {
                    println!("Worker {} got a job; executing.", i);
                }
            }
        });

        Worker { id, thread }
    }
}

fn main() {
    let message = Message {
        message: "message".to_string(),
    };
    let message2 = Message {
        message: "message2".to_string(),
    };
    let message3 = Message {
        message: "message3".to_string(),
    };
    let message4 = Message {
        message: "message4".to_string(),
    };
    let message5 = Message {
        message: "message5".to_string(),
    };
    let message6 = Message {
        message: "message6".to_string(),
    };
    let message7 = Message {
        message: "message7".to_string(),
    };
    let message8 = Message {
        message: "message8".to_string(),
    };

    let message_vec = vec![
        message, message2, message3, message4, message5, message6, message7, message8,
    ];

    let thread_pool = ThreadPool::new(10);

    for message in message_vec {
        thread_pool.sender.send(message).unwrap();
    }

    for worker in thread_pool.workers {
        worker.thread.join().unwrap()
    }

    for i in thread_pool.result.lock().unwrap().iter() {
        println!("Worker {} got a job; executing.", i);
    }

    // train::load_dataset();

    // train::train_neuralnet();

    // for next_move in csa_file.moves.iter() {
    //     board.pprint();
    //     board = board.move_koma(&next_move);
    //     println!("")
    // }

    // let input_tensor = Tensor::rand(-1.0f32, 1.0, (1, 1, 28, 28), &Device::Cpu).unwrap();

    // let mut varmap = VarMap::new();
    // let vs = VarBuilder::from_varmap(&varmap, DType::F32, &Device::Cpu);
    // let model = ConvNet::new(vs.clone()).unwrap();

    // let y = model.forward(&input_tensor, true);

    // println!("{:?}", y);

    // println!("{:#?}", csa_file)

    // let sample = 1;

    // for num in 0..10 {
    //     println!("Hello, world!");
    // }
}
