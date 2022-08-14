#![allow(unused_imports)]
mod cube;
mod search;
use std::collections::HashMap;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3 = Cube3x3::new();
    cube3x3 = cube3x3.turn(Move::R);
    cube3x3 = cube3x3.turn(Move::F2);

    let hashmap: HashMap<Cube3x3, Vec<Move>> = gen_lookups_3x3(2);

    let mut moves: Vec<Move> = Vec::new();
    match hashmap.get(&cube3x3) {
        None => {println!("Key not found in lookup table")},
        Some(x) => {moves = x.clone()},
    }

    for m in moves {
        print!("{:?} ", m);
    }
    println!();
}
