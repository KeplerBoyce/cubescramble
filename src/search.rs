#![allow(unused_must_use)]
#![allow(dead_code)]
use std::collections::VecDeque;
use std::collections::HashMap;
use crate::cube::*;
use crate::cube;
use std::iter;

trait Append<Move> { //helper for inline clone, push, and return
    fn append_move(&self, m: Move) -> Vec<Move>;
}

impl<Move: Clone> Append<Move> for Vec<Move> {
    fn append_move(&self, m: Move) -> Vec<Move> {
        let mut moves = self.to_vec();
        moves.push(m);
        return moves;
    }
}

//DFS from scrambled state to g1 state
pub fn search3x3_phase_1(cube3x3: Cube3x3, lookup: HashMap<Cube3x3Simple, Vec<Move>>, max_depth: usize) -> Option<Vec<Move>> {
    let mut depth: usize = 1;

    //iterative deepening
    while depth <= max_depth {
        let mut stack: Vec<(Face, Face, Vec<Move>)> = vec!((Face::None, Face::None, Vec::new()));

        //run until stack is empty (all nodes within max depth have been checked)
        while stack.len() > 0 {
            let (last_face, next_last_face, moves) = stack.pop().unwrap(); //destructure current state

            let mut test_cube = cube3x3;
            for m in moves.clone() {
                test_cube = test_cube.turn(m);
            }
            if lookup.contains_key(&test_cube.simplify()) {
                let finish_moves = lookup.get(&test_cube.simplify());
                match finish_moves {
                    None => {}
                    Some(x) => {
                        let mut start = moves;
                        let mut end = x.clone();
                        end.reverse();
                        for m in &mut end {
                            m.invert();
                        }
                        start.extend(end.iter());
                        return Some(start);
                    }
                }
            } else if moves.len() == depth {
                continue; //stop this branch if max depth reached
            }
            //push next possible cube states
            if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
                stack.push((Face::F, last_face, moves.append_move(Move::F)));
                stack.push((Face::F, last_face, moves.append_move(Move::Fi)));
                stack.push((Face::F, last_face, moves.append_move(Move::F2)));
            }
            if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
                stack.push((Face::R, last_face, moves.append_move(Move::R)));
                stack.push((Face::R, last_face, moves.append_move(Move::Ri)));
                stack.push((Face::R, last_face, moves.append_move(Move::R2)));
            }
            if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
                stack.push((Face::U, last_face, moves.append_move(Move::U)));
                stack.push((Face::U, last_face, moves.append_move(Move::Ui)));
                stack.push((Face::U, last_face, moves.append_move(Move::U2)));
            }
            if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
                stack.push((Face::B, last_face, moves.append_move(Move::B)));
                stack.push((Face::B, last_face, moves.append_move(Move::Bi)));
                stack.push((Face::B, last_face, moves.append_move(Move::B2)));
            }
            if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
                stack.push((Face::L, last_face, moves.append_move(Move::L)));
                stack.push((Face::L, last_face, moves.append_move(Move::Li)));
                stack.push((Face::L, last_face, moves.append_move(Move::L2)));
            }
            if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
                stack.push((Face::D, last_face, moves.append_move(Move::D)));
                stack.push((Face::D, last_face, moves.append_move(Move::Di)));
                stack.push((Face::D, last_face, moves.append_move(Move::D2)));
            }
        }
        depth += 1;
    }
    return None;
}

//DFS from g1 to solved state
pub fn search3x3_phase_2(cube3x3: Cube3x3, lookup: HashMap<Cube3x3, Vec<Move>>, max_depth: usize) -> Option<Vec<Move>> {
    let mut depth: usize = 1;

    //iterative deepening
    while depth <= max_depth {
        let mut stack: Vec<(Face, Face, Vec<Move>)> = vec!((Face::None, Face::None, Vec::new()));

        //run until stack is empty (all nodes within max depth have been checked)
        while stack.len() > 0 {
            let (last_face, next_last_face, moves) = stack.pop().unwrap(); //destructure current state

            let mut test_cube = cube3x3;
            for m in moves.clone() {
                test_cube = test_cube.turn(m);
            }
            if lookup.contains_key(&test_cube) {
                let finish_moves = lookup.get(&test_cube);
                match finish_moves {
                    None => {}
                    Some(x) => {
                        let mut start = moves;
                        let mut end = x.clone();
                        end.reverse();
                        for m in &mut end {
                            m.invert();
                        }
                        start.extend(end.iter());
                        return Some(start);
                    }
                }
            } else if moves.len() == depth {
                continue; //stop this branch if max depth reached
            }
            //push next possible cube states
            if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
                stack.push((Face::F, last_face, moves.append_move(Move::F2)));
            }
            if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
                stack.push((Face::R, last_face, moves.append_move(Move::R2)));
            }
            if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
                stack.push((Face::U, last_face, moves.append_move(Move::U)));
                stack.push((Face::U, last_face, moves.append_move(Move::Ui)));
                stack.push((Face::U, last_face, moves.append_move(Move::U2)));
            }
            if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
                stack.push((Face::B, last_face, moves.append_move(Move::B2)));
            }
            if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
                stack.push((Face::L, last_face, moves.append_move(Move::L2)));
            }
            if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
                stack.push((Face::D, last_face, moves.append_move(Move::D)));
                stack.push((Face::D, last_face, moves.append_move(Move::Di)));
                stack.push((Face::D, last_face, moves.append_move(Move::D2)));
            }
        }
        depth += 1;
    }
    return None;
}

//generate lookup hashmap with specified depth from generic g1 state
pub fn gen_lookups_3x3_phase_1(depth: usize) -> HashMap<Cube3x3Simple, Vec<Move>> {
    let mut hashmap = HashMap::new();
    let mut deque: VecDeque<(Face, Face, Vec<Move>)> = VecDeque::from(vec!((Face::None, Face::None, Vec::new())));
    while deque.len() > 0 {
        let (last_face, next_last_face, moves) = deque.pop_front().unwrap(); //destructure current state

        let mut cube3x3 = Cube3x3Simple::new();
        for m in moves.clone() {
            cube3x3 = cube3x3.turn(m);
        }
        if !hashmap.contains_key(&cube3x3) {
            hashmap.insert(cube3x3, moves.clone());
        }
        if moves.len() == depth {
            continue;
        }
        //push next possible cube states (moves in move list are inverted)
        if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
            deque.push_back((Face::F, last_face, moves.append_move(Move::F)));
            deque.push_back((Face::F, last_face, moves.append_move(Move::Fi)));
            deque.push_back((Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            deque.push_back((Face::R, last_face, moves.append_move(Move::R)));
            deque.push_back((Face::R, last_face, moves.append_move(Move::Ri)));
            deque.push_back((Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            deque.push_back((Face::U, last_face, moves.append_move(Move::U)));
            deque.push_back((Face::U, last_face, moves.append_move(Move::Ui)));
            deque.push_back((Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            deque.push_back((Face::B, last_face, moves.append_move(Move::B)));
            deque.push_back((Face::B, last_face, moves.append_move(Move::Bi)));
            deque.push_back((Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            deque.push_back((Face::L, last_face, moves.append_move(Move::L)));
            deque.push_back((Face::L, last_face, moves.append_move(Move::Li)));
            deque.push_back((Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            deque.push_back((Face::D, last_face, moves.append_move(Move::D)));
            deque.push_back((Face::D, last_face, moves.append_move(Move::Di)));
            deque.push_back((Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
    hashmap.insert(Cube3x3Simple::new(), Vec::new()); //include g1 state itself
    return hashmap;
}

//generate lookup hashmap with specified depth from solved state
pub fn gen_lookups_3x3_phase_2(depth: usize) -> HashMap<Cube3x3, Vec<Move>> {
    let mut hashmap = HashMap::new();
    let mut deque: VecDeque<(Face, Face, Vec<Move>)> = VecDeque::from(vec!((Face::None, Face::None, Vec::new())));
    while deque.len() > 0 {
        let (last_face, next_last_face, moves) = deque.pop_front().unwrap(); //destructure current state

        let mut cube3x3 = Cube3x3::new();
        for m in moves.clone() {
            cube3x3 = cube3x3.turn(m);
        }
        if !hashmap.contains_key(&cube3x3) {
            hashmap.insert(cube3x3, moves.clone());
        }
        if moves.len() == depth {
            continue;
        }
        //push next possible cube states (moves in move list are inverted)
        if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
            deque.push_back((Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            deque.push_back((Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            deque.push_back((Face::U, last_face, moves.append_move(Move::U)));
            deque.push_back((Face::U, last_face, moves.append_move(Move::Ui)));
            deque.push_back((Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            deque.push_back((Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            deque.push_back((Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            deque.push_back((Face::D, last_face, moves.append_move(Move::D)));
            deque.push_back((Face::D, last_face, moves.append_move(Move::Di)));
            deque.push_back((Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
    hashmap.insert(Cube3x3::new(), Vec::new()); //include solved cube
    return hashmap;
}
