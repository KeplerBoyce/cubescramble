#![allow(unused_imports)]
mod cube;
mod search;
use std::collections::HashMap;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3 = Cube3x3::new();
    let hashmap: HashMap<Cube3x3, Vec<Move>> = gen_lookups_3x3(1);
    cube3x3 = cube3x3.turn(Move::R);
    cube3x3 = cube3x3.turn(Move::U);
}
