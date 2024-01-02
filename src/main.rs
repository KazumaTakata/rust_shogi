mod board;
mod piece_type;
mod position;
mod csa;


fn main() {
    let csa_file = csa::parse_csa_file();

    let board = board::initialize_board();

    board.pprint();

    let board = board.move_koma(&csa_file.moves[0]);
    board.pprint();

 
    let board = board.move_koma(&csa_file.moves[1]);
    board.pprint();

 
    let board = board.move_koma(&csa_file.moves[2]);
    board.pprint();


    // println!("{:#?}", csa_file)

    // let sample = 1;

    // for num in 0..10 {
    //     println!("Hello, world!");
    // }
}
