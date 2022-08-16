#![allow(unused_imports)]
#![allow(dead_code)]
mod cube;
mod search;
use bincode::{serialize_into, deserialize_from};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::collections::HashMap;
use cube::*;
use search::*;

//generate lookup tables and store on disk
fn gen_lookups_3x3(limit1: usize, limit2: usize) -> (HashMap<Cube3x3Simple, Vec<Move>>, HashMap<Cube3x3, Vec<Move>>) {
    let lookup1: HashMap<Cube3x3Simple, Vec<Move>> = gen_lookups_3x3_phase_1(limit1);
    let lookup2: HashMap<Cube3x3, Vec<Move>> = gen_lookups_3x3_phase_2(limit2);
    return (lookup1, lookup2);
}

//generate scramble given lookup tables and depths
fn gen_scramble_3x3(lookup1: &HashMap<Cube3x3Simple, Vec<Move>>, lookup2: &HashMap<Cube3x3, Vec<Move>>, limit1: usize, limit2: usize) -> Vec<Move> {
    let mut cube3x3 = Cube3x3::new();
    cube3x3.scramble();

    let moves1: Vec<Move>;
    let moves2: Vec<Move>;
    match search3x3_phase_1(cube3x3, lookup1, limit1) { //first phase search
        None => {moves1 = Vec::new()}
        Some(x) => {moves1 = x}
    }
    println!("phase 1: {}", moves1.len());
    for m in &moves1 { //apply first phase solution once it is found
        cube3x3 = cube3x3.turn(*m);
    }
    match search3x3_phase_2(cube3x3, lookup2, limit2) { //second phase search
        None => {moves2 = Vec::new()}
        Some(x) => {moves2 = x}
    }
    println!("phase 2: {}", moves2.len());

    let mut scramble = moves1;
    scramble.extend(moves2.iter());
    scramble.reverse();
    return scramble;
}

fn main() {
    println!("making lookup tables");
    let (lookup1, lookup2) = gen_lookups_3x3(5, 7);
    println!("making scrambles");
    let mut scramble;
    for _ in 0..3 {
        scramble = gen_scramble_3x3(&lookup1, &lookup2, 7, 11);
        for m in &scramble {
            print!("{:?} ", m);
        }
        println!();
    }
}
