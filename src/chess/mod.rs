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
    pub pieces: [[Bitboard; 6]; 2]
}

impl Board {
    fn get_pieces(&self, side: Side, piece: Piece) -> Bitboard {
        self.pieces[side as u8][piece as u8]
    }
}