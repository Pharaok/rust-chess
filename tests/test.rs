use chess::{board::Board, r#move::Move};

#[test]
fn test() {
    let mut board = Board::new();
    board.make_move(Move { from: 12, to: 28 }).unwrap();
    println!("{:?}", board);
    panic!();
}
