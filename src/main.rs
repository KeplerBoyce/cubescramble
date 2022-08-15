#![allow(unused_imports)]
mod cube;
mod search;
use std::collections::HashMap;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3 = Cube3x3::new();
    cube3x3.scramble();

    println!("generating lookup table 1");
    let lookup1: HashMap<Cube3x3Simple, Vec<Move>> = gen_lookups_3x3_phase_1(5);
    println!("generating lookup table 2");
    let lookup2: HashMap<Cube3x3, Vec<Move>> = gen_lookups_3x3_phase_2(7);

    println!("starting search phase 1");
    let moves1: Vec<Move>;
    match search3x3_phase_1(cube3x3, lookup1, 7) {
        None => {moves1 = Vec::new()}
        Some(x) => {moves1 = x}
    }
    print!("phase 1 solution: ");
    for m in &moves1 {
        print!("{:?} ", m);
        cube3x3 = cube3x3.turn(*m);
    }
    println!();

    println!("starting search phase 2");
    let moves2: Vec<Move>;
    match search3x3_phase_2(cube3x3, lookup2, 11) {
        None => {moves2 = Vec::new()}
        Some(x) => {moves2 = x}
    }
    print!("phase 2 solution: ");
    for m in &moves2 {
        print!("{:?} ", m);
    }
    println!();

    let mut scramble = moves1;
    scramble.extend(moves2.iter());
    scramble.reverse();
    print!("scramble: ");
    for m in &mut scramble {
        m.invert();
        print!("{:?} ", m);
    }
    println!();
}
