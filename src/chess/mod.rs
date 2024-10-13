pub mod bitboard;

pub fn test() {
    let bb = bitboard::Bitboard {
        value: 3653
    };
    println!("{}", bb);
}