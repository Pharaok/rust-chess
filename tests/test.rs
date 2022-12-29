use chess::{board::Board, r#move::Move, square::Square};

#[test]
fn test() {
    let sq = Square::from_an("90");
    println!("{:?}", sq);
    let mut board = Board::new();
    board
        .make_move(Move {
            from: Square::from_an("e2").unwrap(),
            to: Square::from_an("e4").unwrap(),
        })
        .unwrap();
    println!("{:?}", board);
    panic!();
}
