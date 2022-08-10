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

pub fn search3x3(cube3x3: Cube3x3) {
    //queue of cube state, last color, 2nd last color, and moves so far
    let mut q: Queue<(Cube3x3, Color, Color, Vec<Move>)> = queue![(cube3x3, Color::None, Color::None, Vec::new())];
    while q.size() > 0 {
        let state: (Cube3x3, Color, Color, Vec<Move>) = q.remove().unwrap();
        if state.0.check() {
            print!("Found:");
            for m in state.3.iter().enumerate() {
                print!(" {:?}", m.1);
            }
            print!("\n");
            return;
        }
        //make sure we are not doing redundant moves (e.g. L R L2 == Li R)
        if state.1 != Color::G && !(state.1 == Color::B && state.2 == Color::G) {
            q.add((state.0.turn(Move::F), Color::G, state.1, state.3.append_move(Move::F)));
            q.add((state.0.turn(Move::Fi), Color::G, state.1, state.3.append_move(Move::Fi)));
            q.add((state.0.turn(Move::F2), Color::G, state.1, state.3.append_move(Move::F2)));
        }
        if state.1 != Color::R && !(state.1 == Color::O && state.2 == Color::R) {
            q.add((state.0.turn(Move::R), Color::R, state.1, state.3.append_move(Move::R)));
            q.add((state.0.turn(Move::Ri), Color::R, state.1, state.3.append_move(Move::Ri)));
            q.add((state.0.turn(Move::R2), Color::R, state.1, state.3.append_move(Move::R2)));
        }
        if state.1 != Color::W && !(state.1 == Color::Y && state.2 == Color::W) {
            q.add((state.0.turn(Move::U), Color::W, state.1, state.3.append_move(Move::U)));
            q.add((state.0.turn(Move::Ui), Color::W, state.1, state.3.append_move(Move::Ui)));
            q.add((state.0.turn(Move::U2), Color::W, state.1, state.3.append_move(Move::U2)));
        }
        if state.1 != Color::B && !(state.1 == Color::G && state.2 == Color::B) {
            q.add((state.0.turn(Move::B), Color::B, state.1, state.3.append_move(Move::B)));
            q.add((state.0.turn(Move::Bi), Color::B, state.1, state.3.append_move(Move::Bi)));
            q.add((state.0.turn(Move::B2), Color::B, state.1, state.3.append_move(Move::B2)));
        }
        if state.1 != Color::O && !(state.1 == Color::R && state.2 == Color::O) {
            q.add((state.0.turn(Move::L), Color::O, state.1, state.3.append_move(Move::L)));
            q.add((state.0.turn(Move::Li), Color::O, state.1, state.3.append_move(Move::Li)));
            q.add((state.0.turn(Move::L2), Color::O, state.1, state.3.append_move(Move::L2)));
        }
        if state.1 != Color::Y && !(state.1 == Color::W && state.2 == Color::Y) {
            q.add((state.0.turn(Move::D), Color::Y, state.1, state.3.append_move(Move::D)));
            q.add((state.0.turn(Move::Di), Color::Y, state.1, state.3.append_move(Move::Di)));
            q.add((state.0.turn(Move::D2), Color::Y, state.1, state.3.append_move(Move::D2)));
        }
    }
}
