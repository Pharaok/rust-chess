use crate::{r#move::Move, square::Square};

#[derive(Debug)]
pub struct Board {
    bitboards: [u64; 14],
}

impl Board {
    const START_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR";
    const PIECES: [char; 6] = ['P', 'N', 'B', 'R', 'Q', 'K'];

    fn piece_to_index(c: char) -> Option<usize> {
        Self::PIECES
            .iter()
            .position(|&p| p == c.to_ascii_uppercase())
            .and_then(|p| Some((1 + p) * 2 + c.is_ascii_lowercase() as usize))
    }
    fn index_to_piece(i: usize) -> Option<char> {
        if !i < (1 + Self::PIECES.len()) * 2 {
            return None;
        }
        let mut piece = Self::PIECES[i / 2 - 1];
        if i % 2 == 1 {
            piece = piece.to_ascii_lowercase();
        }
        Some(piece)
    }

    fn blank_board() -> Self {
        Self {
            bitboards: [0; (1 + Self::PIECES.len()) * 2],
        }
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
                if file_index > 7 {
                    break;
                }

                if c.is_ascii_digit() {
                    file_index += c.to_digit(10).unwrap();
                    continue;
                }

                if let Some(piece_type) = Self::piece_to_index(c) {
                    draft_board.bitboards[piece_type % 2] |= 1 << (rank_index * 8 + file_index);
                    draft_board.bitboards[piece_type] |= 1 << (rank_index * 8 + file_index);
                } else {
                    return Err(());
                }
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
                    a.push(Self::index_to_piece(piece).unwrap());
                } else {
                    a.push('.')
                }
            }
            a.push('\n');
        }
        a
    }
}
