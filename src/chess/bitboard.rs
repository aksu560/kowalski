use std::ops::{BitAnd, BitOr, BitXor, Not};

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Bitboard {
    pub value: u64
}
impl std::fmt::Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let bin_str: String = format!("{:064b}", self.value).chars().rev().collect();
        let out_str = format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            &bin_str[56..64],
            &bin_str[48..56],
            &bin_str[40..48],
            &bin_str[32..40],
            &bin_str[24..32],
            &bin_str[16..24],
            &bin_str[8..16],
            &bin_str[0..8],
        );

        write!(f, "{}", out_str)
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