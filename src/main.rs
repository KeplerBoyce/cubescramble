#![allow(unused_imports)]
mod cube;
mod search;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3 = Cube3x3::new();
    cube3x3.print_pieces();
    cube3x3.scramble();
    cube3x3.print_pieces();
}
