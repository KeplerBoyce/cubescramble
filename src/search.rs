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

        //create Vec of possible cube states from current state
        let mut next_nodes: Vec<(Cube3x3, Face, Face, Vec<Move>, usize)> = Vec::new();
        if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
            next_nodes.push((cube3x3.turn(Move::F), Face::F, last_face, moves.append_move(Move::F), cube3x3.turn(Move::F).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::Fi), Face::F, last_face, moves.append_move(Move::Fi), cube3x3.turn(Move::Fi).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::F2), Face::F, last_face, moves.append_move(Move::F2), cube3x3.turn(Move::F2).manhattan_dist()));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            next_nodes.push((cube3x3.turn(Move::R), Face::R, last_face, moves.append_move(Move::R), cube3x3.turn(Move::R).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::Ri), Face::R, last_face, moves.append_move(Move::Ri), cube3x3.turn(Move::Ri).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::R2), Face::R, last_face, moves.append_move(Move::R2), cube3x3.turn(Move::R2).manhattan_dist()));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            next_nodes.push((cube3x3.turn(Move::U), Face::U, last_face, moves.append_move(Move::U), cube3x3.turn(Move::U).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::Ui), Face::U, last_face, moves.append_move(Move::Ui), cube3x3.turn(Move::Ui).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::U2), Face::U, last_face, moves.append_move(Move::U2), cube3x3.turn(Move::U2).manhattan_dist()));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            next_nodes.push((cube3x3.turn(Move::B), Face::B, last_face, moves.append_move(Move::B), cube3x3.turn(Move::B).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::Bi), Face::B, last_face, moves.append_move(Move::Bi), cube3x3.turn(Move::Bi).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::B2), Face::B, last_face, moves.append_move(Move::B2), cube3x3.turn(Move::B2).manhattan_dist()));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            next_nodes.push((cube3x3.turn(Move::L), Face::L, last_face, moves.append_move(Move::L), cube3x3.turn(Move::L).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::Li), Face::L, last_face, moves.append_move(Move::Li), cube3x3.turn(Move::Li).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::L2), Face::L, last_face, moves.append_move(Move::L2), cube3x3.turn(Move::L2).manhattan_dist()));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            next_nodes.push((cube3x3.turn(Move::D), Face::D, last_face, moves.append_move(Move::D), cube3x3.turn(Move::D).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::Di), Face::D, last_face, moves.append_move(Move::Di), cube3x3.turn(Move::Di).manhattan_dist()));
            next_nodes.push((cube3x3.turn(Move::D2), Face::D, last_face, moves.append_move(Move::D2), cube3x3.turn(Move::D2).manhattan_dist()));
        }
        next_nodes.sort_by_key(|x| x.4);
        next_nodes.reverse(); //sort by manhattan distance, decreasing
        for n in next_nodes {
            stack.push((n.0, n.1, n.2, n.3)); //push with manhattan distance removed
        }
    }
}
