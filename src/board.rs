use crate::{r#move::Move, square::Square};

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
                let mut piece_type = c.is_ascii_lowercase() as usize;
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

    pub fn piece_at(&self, square: Square) -> Option<usize> {
        // skip the color bitboards
        for (piece_type, bitboard) in self.bitboards.iter().enumerate().skip(2) {
            if bitboard & 1 << square.0 != 0 {
                return Some(piece_type);
            }
        }
        None
    }

    pub fn make_move(&mut self, r#move: Move) -> Result<(), ()> {
        if let Some(piece) = self.piece_at(r#move.from) {
            if let Some(captured_piece) = self.piece_at(r#move.to) {
                self.bitboards[captured_piece] ^= 1 << r#move.to.0;
                self.bitboards[captured_piece % 2] ^= 1 << r#move.to.0;
            }
            self.bitboards[piece] ^= 1 << r#move.from.0 | 1 << r#move.to.0;
            self.bitboards[piece % 2] ^= 1 << r#move.from.0 | 1 << r#move.to.0;
        } else {
            return Err(());
        }

        Ok(())
    }
    pub fn ascii(&self) -> String {
        let mut a = String::new();
        for rank in (0..8).rev() {
            for file in 0..8 {
                let square = Square(rank * 8 + file);

                a.push(' ');
                if let Some(piece) = self.piece_at(square) {
                    let mut c;
                    match piece / 2 {
                        1 => c = 'P',
                        2 => c = 'N',
                        3 => c = 'B',
                        4 => c = 'R',
                        5 => c = 'Q',
                        6 => c = 'K',
                        _ => panic!(),
                    }
                    if piece % 2 == 1 {
                        c = c.to_ascii_lowercase();
                    }
                    a.push(c);
                } else {
                    a.push('.')
                }
            }
            a.push('\n');
        }
        a
    }
}
