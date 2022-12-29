use chess::board::Board;

#[test]
fn test() {
    let mut board = Board::new();
    println!("{:?} {:?}", board, board.piece_at(0, 0));
    panic!();
}
