#![allow(unused_must_use)]
use queues::*;
use crate::cube::*;

pub fn search3x3(cube3x3: Cube3x3) {
    //queue of cube state, last color, and 2nd last color
    let mut q: Queue<(Cube3x3, Color, Color)> = queue![(cube3x3, Color::None, Color::None)];
    while q.size() > 0 {
        let state: (Cube3x3, Color, Color) = q.remove().unwrap();
        if state.0.check_g1() {
            println!("Found");
            return;
        }
        //make sure we are not doing redundant moves (e.g. L R L2 == Li R)
        if state.1 != Color::G && !(state.1 == Color::B && state.2 == Color::G) {
            q.add((state.0.turn(Move::F), Color::G, state.1));
            q.add((state.0.turn(Move::Fi), Color::G, state.1));
            q.add((state.0.turn(Move::F2), Color::G, state.1));
        }
        if state.1 != Color::R && !(state.1 == Color::O && state.2 == Color::R) {
            q.add((state.0.turn(Move::R), Color::R, state.1));
            q.add((state.0.turn(Move::Ri), Color::R, state.1));
            q.add((state.0.turn(Move::R2), Color::R, state.1));
        }
        if state.1 != Color::W && !(state.1 == Color::Y && state.2 == Color::W) {
            q.add((state.0.turn(Move::U), Color::W, state.1));
            q.add((state.0.turn(Move::Ui), Color::W, state.1));
            q.add((state.0.turn(Move::U2), Color::W, state.1));
        }
        if state.1 != Color::B && !(state.1 == Color::G && state.2 == Color::B) {
            q.add((state.0.turn(Move::B), Color::B, state.1));
            q.add((state.0.turn(Move::Bi), Color::B, state.1));
            q.add((state.0.turn(Move::B2), Color::B, state.1));
        }
        if state.1 != Color::O && !(state.1 == Color::R && state.2 == Color::O) {
            q.add((state.0.turn(Move::L), Color::O, state.1));
            q.add((state.0.turn(Move::Li), Color::O, state.1));
            q.add((state.0.turn(Move::L2), Color::O, state.1));
        }
        if state.1 != Color::Y && !(state.1 == Color::W && state.2 == Color::Y) {
            q.add((state.0.turn(Move::D), Color::Y, state.1));
            q.add((state.0.turn(Move::Di), Color::Y, state.1));
            q.add((state.0.turn(Move::D2), Color::Y, state.1));
        }
    }
}
