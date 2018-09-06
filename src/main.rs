mod tile;
mod board;

use board::*;

fn main() {
    let b = Board::new(10, 10);
    println!("{}", b);
}
