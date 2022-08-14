#![allow(unused_imports)]
mod cube;
mod search;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3 = Cube3x3::new();
    cube3x3 = cube3x3.turn(Move::U);
    cube3x3 = cube3x3.turn(Move::F2);
    cube3x3 = cube3x3.turn(Move::Li);
    cube3x3 = cube3x3.turn(Move::R);
    cube3x3 = cube3x3.turn(Move::F2);
    search3x3(cube3x3, 5);
}
