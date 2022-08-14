#![allow(unused_imports)]
mod cube;
mod search;
use std::collections::HashMap;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3 = Cube3x3::new();
    cube3x3 = cube3x3.turn(Move::R2);
    cube3x3 = cube3x3.turn(Move::F2);
    cube3x3 = cube3x3.turn(Move::U);
    cube3x3 = cube3x3.turn(Move::F2);
    cube3x3 = cube3x3.turn(Move::B2);
    cube3x3 = cube3x3.turn(Move::L2);
    cube3x3 = cube3x3.turn(Move::U2);

    println!("generating lookup table");
    let lookup: HashMap<Cube3x3, Vec<Move>> = gen_lookups_3x3_phase_2(4);

    println!("starting search");
    let moves: Vec<Move>;
    match search3x3_phase_2(cube3x3, lookup, 3, 9) {
        None => {moves = Vec::new()}
        Some(x) => {moves = x}
    }

    for m in moves {
        print!("{:?} ", m);
    }
    println!();
}
