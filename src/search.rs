#![allow(unused_must_use)]
#![allow(dead_code)]
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

        if cube3x3.check() {
            print!("Found:");
            for m in moves.iter().enumerate() {
                print!(" {:?}", m.1);
            }
            print!("\n");
            return;
        } else if moves.len() == depth {
            continue; //stop this branch if max depth reached
        }
        //push next possible moves to stack
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
