mod board;
mod csa;
mod piece_type;
mod position;
mod neural;

fn main() {
    let csa_file = csa::parse_csa_file();

    let mut board = board::initialize_board();

    println!("tensor shape: {:?}", board.to_tensor().shape().dims3());

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
