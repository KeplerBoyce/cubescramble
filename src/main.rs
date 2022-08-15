#![allow(unused_imports)]
mod cube;
mod search;
use std::collections::HashMap;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3 = Cube3x3::new();
    //D B2 R2 U2 F2 U B2 L2 U2 B2 U' R F' U R B L' B2 L F U
    cube3x3 = cube3x3.turn(Move::D);
    cube3x3 = cube3x3.turn(Move::B2);
    cube3x3 = cube3x3.turn(Move::R2);
    cube3x3 = cube3x3.turn(Move::U2);
    cube3x3 = cube3x3.turn(Move::F2);
    cube3x3 = cube3x3.turn(Move::U);
    cube3x3 = cube3x3.turn(Move::B2);
    cube3x3 = cube3x3.turn(Move::L2);
    cube3x3 = cube3x3.turn(Move::U2);
    cube3x3 = cube3x3.turn(Move::B2);
    cube3x3 = cube3x3.turn(Move::Ui);
    cube3x3 = cube3x3.turn(Move::R);
    cube3x3 = cube3x3.turn(Move::Fi);
    cube3x3 = cube3x3.turn(Move::U);
    cube3x3 = cube3x3.turn(Move::R);
    cube3x3 = cube3x3.turn(Move::B);
    cube3x3 = cube3x3.turn(Move::Li);
    cube3x3 = cube3x3.turn(Move::B2);
    cube3x3 = cube3x3.turn(Move::L);
    cube3x3 = cube3x3.turn(Move::F);
    cube3x3 = cube3x3.turn(Move::U);

    println!("generating lookup table 1");
    let lookup1: HashMap<Cube3x3Simple, Vec<Move>> = gen_lookups_3x3_phase_1(4);
    println!("generating lookup table 2");
    let lookup2: HashMap<Cube3x3, Vec<Move>> = gen_lookups_3x3_phase_2(6);

    println!("starting search phase 1");
    let moves1: Vec<Move>;
    match search3x3_phase_1(cube3x3, lookup1, 6) {
        None => {moves1 = Vec::new()}
        Some(x) => {moves1 = x}
    }
    print!("phase 1 solution: ");
    for m in moves1 {
        print!("{:?} ", m);
        cube3x3 = cube3x3.turn(m);
    }
    println!();

    println!("starting search phase 2");
    let moves2: Vec<Move>;
    match search3x3_phase_2(cube3x3, lookup2, 9) {
        None => {moves2 = Vec::new()}
        Some(x) => {moves2 = x}
    }
    print!("phase 2 solution: ");
    for m in moves2 {
        print!("{:?} ", m);
    }
    println!();
}
