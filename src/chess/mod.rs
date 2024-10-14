use crate::chess::bitboard::Bitboard;

pub mod bitboard;
#[derive(Copy, Clone)]
enum Piece {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
    NONE,
}
#[derive(Copy, Clone)]
enum Side {
    WHITE,
    BLACK,
}

struct Board {
    pub bb_pieces: [[Bitboard; 6]; 2],
    pub pieces: [Piece; 64],
}

impl Board {
    pub fn new() -> Board {
        Board {
            bb_pieces: [[Bitboard::new(0); 6], [Bitboard::new(0); 6]],
            pieces: [Piece::NONE; 64],
        }
    }
    fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.bb_pieces[side as usize][piece as usize]
    }
    fn set_pieces(&mut self, side: Side, piece: Piece, bb: Bitboard) {
        self.bb_pieces[side as usize][piece as usize] = bb;
    }
}

#[cfg(test)]
mod tests {
    use crate::chess::{Board, Piece, Side};
    use crate::chess::bitboard::Bitboard;

    #[test]
    fn test_get_set_pieces() {
        let mut board: Board = Board::new();
        assert_eq!(board.get_pieces(Side::WHITE, Piece::BISHOP), Bitboard::new(0));

        board.set_pieces(Side::WHITE, Piece::BISHOP, Bitboard::new(256));
        assert_eq!(board.get_pieces(Side::WHITE, Piece::BISHOP), Bitboard::new(256));
    }
}