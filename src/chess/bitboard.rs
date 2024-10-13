use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Bitboard {
    pub value: u64
}

impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut out = String::new();

        for rank in 0..8 {
            for file in 0..8 {
                let bit = rank * 8 + file;
                let bit_val = self.value & (1 << bit) != 0;

                out += if bit_val { "1 " } else { "0 "};
            }
            out += "\n"
        }
        out.pop();
        write!(f, "{}", out)
    }
}

impl BitAnd for Bitboard {
    type Output = Bitboard;
    fn bitand(self, rhs: Self) -> Self::Output {
        Bitboard { value: self.value & rhs.value }
    }
}

impl BitOr for Bitboard {
    type Output = Bitboard;
    fn bitor(self, rhs: Self) -> Self::Output {
        Bitboard { value: self.value | rhs.value }
    }
}

impl BitXor for Bitboard {
    type Output = Bitboard;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Bitboard { value: self.value ^ rhs.value }
    }
}

impl Not for Bitboard {
    type Output = Bitboard;
    fn not(self) -> Self::Output {
        Bitboard { value: !self.value }
    }
}