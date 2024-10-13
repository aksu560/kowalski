mod chess;

fn main() {
    println!("{} engine v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    println!("authors: {}", env!("CARGO_PKG_AUTHORS"));
}