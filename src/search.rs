#![allow(unused_must_use)]
#![allow(dead_code)]
use std::collections::VecDeque;
use std::collections::HashMap;
use queues::*;
use crate::cube::*;
use std::iter;

trait Append<Move> {
    fn append_move(&self, m: Move) -> Vec<Move>;
}

impl<Move: Clone> Append<Move> for Vec<Move> {
    fn append_move(&self, m: Move) -> Vec<Move> {
        let mut moves = self.to_vec();
        moves.push(m);
        return moves;
    }
}

//recursive DFS search
pub fn search3x3(cube3x3: Cube3x3, depth: usize) {
    let mut stack: Vec<(Cube3x3, Face, Face, Vec<Move>)> = vec!((cube3x3, Face::None, Face::None, Vec::new()));
    
    //run until stack is empty (all nodes within max depth have been checked)
    while stack.len() > 0 {
        let (cube3x3, last_face, next_last_face, moves) = stack.pop().unwrap(); //destructure current state

        if cube3x3.check_g1() { //print solution if in g1 state
            print!("Found:");
            for m in moves.iter().enumerate() {
                print!(" {:?}", m.1);
            }
            print!("\n");
            return;
        } else if moves.len() == depth {
            continue; //stop this branch if max depth reached
        }
        //push next possible cube states
        if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
            stack.push((cube3x3.turn(Move::F), Face::F, last_face, moves.append_move(Move::F)));
            stack.push((cube3x3.turn(Move::Fi), Face::F, last_face, moves.append_move(Move::Fi)));
            stack.push((cube3x3.turn(Move::F2), Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            stack.push((cube3x3.turn(Move::R), Face::R, last_face, moves.append_move(Move::R)));
            stack.push((cube3x3.turn(Move::Ri), Face::R, last_face, moves.append_move(Move::Ri)));
            stack.push((cube3x3.turn(Move::R2), Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            stack.push((cube3x3.turn(Move::U), Face::U, last_face, moves.append_move(Move::U)));
            stack.push((cube3x3.turn(Move::Ui), Face::U, last_face, moves.append_move(Move::Ui)));
            stack.push((cube3x3.turn(Move::U2), Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            stack.push((cube3x3.turn(Move::B), Face::B, last_face, moves.append_move(Move::B)));
            stack.push((cube3x3.turn(Move::Bi), Face::B, last_face, moves.append_move(Move::Bi)));
            stack.push((cube3x3.turn(Move::B2), Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            stack.push((cube3x3.turn(Move::L), Face::L, last_face, moves.append_move(Move::L)));
            stack.push((cube3x3.turn(Move::Li), Face::L, last_face, moves.append_move(Move::Li)));
            stack.push((cube3x3.turn(Move::L2), Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            stack.push((cube3x3.turn(Move::D), Face::D, last_face, moves.append_move(Move::D)));
            stack.push((cube3x3.turn(Move::Di), Face::D, last_face, moves.append_move(Move::Di)));
            stack.push((cube3x3.turn(Move::D2), Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
}

//generate lookup hashmap with specified depth from solved state
pub fn gen_lookups_3x3(depth: usize) -> HashMap<Cube3x3, Vec<Move>> {
    let mut frontier = HashMap::new();
    let mut deque: VecDeque<(Cube3x3, Face, Face, Vec<Move>)> = VecDeque::from(vec!((Cube3x3::new(), Face::None, Face::None, Vec::new())));
    while deque.len() > 0 {
        let (cube3x3, last_face, next_last_face, moves) = deque.pop_front().unwrap(); //destructure current state

        frontier.insert(cube3x3, moves.clone());
        if moves.len() == depth {
            continue;
        }
        //push next possible cube states
        if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
            deque.push_back((cube3x3.turn(Move::F), Face::F, last_face, moves.append_move(Move::F)));
            deque.push_back((cube3x3.turn(Move::Fi), Face::F, last_face, moves.append_move(Move::Fi)));
            deque.push_back((cube3x3.turn(Move::F2), Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            deque.push_back((cube3x3.turn(Move::Ri), Face::R, last_face, moves.append_move(Move::Ri)));
            deque.push_back((cube3x3.turn(Move::R2), Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            deque.push_back((cube3x3.turn(Move::U), Face::U, last_face, moves.append_move(Move::U)));
            deque.push_back((cube3x3.turn(Move::Ui), Face::U, last_face, moves.append_move(Move::Ui)));
            deque.push_back((cube3x3.turn(Move::U2), Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            deque.push_back((cube3x3.turn(Move::B), Face::B, last_face, moves.append_move(Move::B)));
            deque.push_back((cube3x3.turn(Move::Bi), Face::B, last_face, moves.append_move(Move::Bi)));
            deque.push_back((cube3x3.turn(Move::B2), Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            deque.push_back((cube3x3.turn(Move::L), Face::L, last_face, moves.append_move(Move::L)));
            deque.push_back((cube3x3.turn(Move::Li), Face::L, last_face, moves.append_move(Move::Li)));
            deque.push_back((cube3x3.turn(Move::L2), Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            deque.push_back((cube3x3.turn(Move::D), Face::D, last_face, moves.append_move(Move::D)));
            deque.push_back((cube3x3.turn(Move::Di), Face::D, last_face, moves.append_move(Move::Di)));
            deque.push_back((cube3x3.turn(Move::D2), Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
    return frontier;
}
