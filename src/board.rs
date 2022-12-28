#[derive(Debug)]
pub struct Board {
    bitboards: [u64; 14],
}

impl Board {
    const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";

    fn blank_board() -> Self {
        Self { bitboards: [0; 14] }
    }
    pub fn new() -> Self {
        Self::from_fen(Self::START_FEN).unwrap()
    }

    pub fn from_fen(fen: &str) -> Result<Self, ()> {
        let mut board = Self::blank_board();
        match board.set_fen(fen) {
            Ok(_) => Ok(board),
            Err(()) => Err(()),
        }
    }

    pub fn set_fen(&mut self, fen: &str) -> Result<(), ()> {
        let mut draft_board = Self::blank_board();
        let mut rank_index = 7; // start from black's side
        for rank in fen.split("/") {
            let mut file_index = 0;
            for c in rank.chars() {
                let mut piece_type = c.is_ascii_uppercase() as usize;
                if c.is_ascii_digit() {
                    file_index += c.to_digit(10).unwrap();
                    continue;
                }
                match c.to_ascii_uppercase() {
                    'P' => piece_type += 2,
                    'N' => piece_type += 4,
                    'B' => piece_type += 6,
                    'R' => piece_type += 8,
                    'Q' => piece_type += 10,
                    'K' => piece_type += 12,
                    _ => return Err(()),
                }
                if file_index > 7 {
                    break;
                }
                draft_board.bitboards[piece_type % 2] |= 1 << (rank_index * 8 + file_index);
                draft_board.bitboards[piece_type] |= 1 << (rank_index * 8 + file_index);
                file_index += 1;
            }
            if rank_index == 0 {
                break;
            }
            rank_index -= 1;
        }
        *self = draft_board;
        Ok(())
    }
}
