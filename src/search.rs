use std::collections::{VecDeque, HashMap, HashSet};
use crate::cube::*;
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
pub fn search3x3_phase_1(cube3x3: Cube3x3, lookup: &HashMap<Cube3x3Simple, Vec<Move>>, depth: usize) -> Option<Vec<Move>> {
    let mut deque: VecDeque<(Cube3x3Simple, Face, Face, Vec<Move>)> = VecDeque::from(vec!((cube3x3.simplify(), Face::None, Face::None, Vec::new())));
    let mut visited: HashSet<Cube3x3Simple> = HashSet::new();

    //run until stack is empty (all nodes within max depth have been checked)
    while deque.len() > 0 {
        let (cube, last_face, next_last_face, moves) = deque.pop_front().unwrap(); //destructure current state

        if visited.contains(&cube) {
            continue; //kill this branch if this node has already been reached
        } else {
            visited.insert(cube); //otherwise, insert this state to the hashset
        }
        if lookup.contains_key(&cube) {
            let finish_moves = lookup.get(&cube);
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
            deque.push_back((cube.turn(Move::F), Face::F, last_face, moves.append_move(Move::F)));
            deque.push_back((cube.turn(Move::Fi), Face::F, last_face, moves.append_move(Move::Fi)));
            deque.push_back((cube.turn(Move::F2), Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            deque.push_back((cube.turn(Move::R), Face::R, last_face, moves.append_move(Move::R)));
            deque.push_back((cube.turn(Move::Ri), Face::R, last_face, moves.append_move(Move::Ri)));
            deque.push_back((cube.turn(Move::R2), Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            deque.push_back((cube.turn(Move::U), Face::U, last_face, moves.append_move(Move::U)));
            deque.push_back((cube.turn(Move::Ui), Face::U, last_face, moves.append_move(Move::Ui)));
            deque.push_back((cube.turn(Move::U2), Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            deque.push_back((cube.turn(Move::B), Face::B, last_face, moves.append_move(Move::B)));
            deque.push_back((cube.turn(Move::Bi), Face::B, last_face, moves.append_move(Move::Bi)));
            deque.push_back((cube.turn(Move::B2), Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            deque.push_back((cube.turn(Move::L), Face::L, last_face, moves.append_move(Move::L)));
            deque.push_back((cube.turn(Move::Li), Face::L, last_face, moves.append_move(Move::Li)));
            deque.push_back((cube.turn(Move::L2), Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            deque.push_back((cube.turn(Move::D), Face::D, last_face, moves.append_move(Move::D)));
            deque.push_back((cube.turn(Move::Di), Face::D, last_face, moves.append_move(Move::Di)));
            deque.push_back((cube.turn(Move::D2), Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
    return None;
}

//DFS from g1 to solved state
pub fn search3x3_phase_2(cube3x3: Cube3x3, lookup: &HashMap<Cube3x3, Vec<Move>>, depth: usize) -> Option<Vec<Move>> {
    let mut deque: VecDeque<(Cube3x3, Face, Face, Vec<Move>)> = VecDeque::from(vec!((cube3x3, Face::None, Face::None, Vec::new())));
    let mut visited: HashSet<Cube3x3> = HashSet::new();

    //run until stack is empty (all nodes within max depth have been checked)
    while deque.len() > 0 {
        let (cube, last_face, next_last_face, moves) = deque.pop_front().unwrap(); //destructure current state

        if visited.contains(&cube) {
            continue; //kill this branch if this node has already been reached
        } else {
            visited.insert(cube); //otherwise, insert this state to the hashset
        }
        if lookup.contains_key(&cube) {
            let finish_moves = lookup.get(&cube);
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
            deque.push_back((cube.turn(Move::F2), Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            deque.push_back((cube.turn(Move::R2), Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            deque.push_back((cube.turn(Move::U), Face::U, last_face, moves.append_move(Move::U)));
            deque.push_back((cube.turn(Move::Ui), Face::U, last_face, moves.append_move(Move::Ui)));
            deque.push_back((cube.turn(Move::U2), Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            deque.push_back((cube.turn(Move::B2), Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            deque.push_back((cube.turn(Move::L2), Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            deque.push_back((cube.turn(Move::D), Face::D, last_face, moves.append_move(Move::D)));
            deque.push_back((cube.turn(Move::Di), Face::D, last_face, moves.append_move(Move::Di)));
            deque.push_back((cube.turn(Move::D2), Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
    return None;
}

//generate lookup hashmap with specified depth from generic g1 state
pub fn gen_lookups_3x3_phase_1(depth: usize) -> HashMap<Cube3x3Simple, Vec<Move>> {
    let mut hashmap = HashMap::new();
    let mut deque: VecDeque<(Cube3x3Simple, Face, Face, Vec<Move>)> = VecDeque::from(vec!((Cube3x3Simple::new(), Face::None, Face::None, Vec::new())));
    while deque.len() > 0 {
        let (cube, last_face, next_last_face, moves) = deque.pop_front().unwrap(); //destructure current state

        if hashmap.contains_key(&cube) {
            continue;
        } else {
            hashmap.insert(cube, moves.clone());
        }
        if moves.len() == depth {
            continue;
        }
        //push next possible cube states
        if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
            deque.push_back((cube.turn(Move::F), Face::F, last_face, moves.append_move(Move::F)));
            deque.push_back((cube.turn(Move::Fi), Face::F, last_face, moves.append_move(Move::Fi)));
            deque.push_back((cube.turn(Move::F2), Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            deque.push_back((cube.turn(Move::R), Face::R, last_face, moves.append_move(Move::R)));
            deque.push_back((cube.turn(Move::Ri), Face::R, last_face, moves.append_move(Move::Ri)));
            deque.push_back((cube.turn(Move::R2), Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            deque.push_back((cube.turn(Move::U), Face::U, last_face, moves.append_move(Move::U)));
            deque.push_back((cube.turn(Move::Ui), Face::U, last_face, moves.append_move(Move::Ui)));
            deque.push_back((cube.turn(Move::U2), Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            deque.push_back((cube.turn(Move::B), Face::B, last_face, moves.append_move(Move::B)));
            deque.push_back((cube.turn(Move::Bi), Face::B, last_face, moves.append_move(Move::Bi)));
            deque.push_back((cube.turn(Move::B2), Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            deque.push_back((cube.turn(Move::L), Face::L, last_face, moves.append_move(Move::L)));
            deque.push_back((cube.turn(Move::Li), Face::L, last_face, moves.append_move(Move::Li)));
            deque.push_back((cube.turn(Move::L2), Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            deque.push_back((cube.turn(Move::D), Face::D, last_face, moves.append_move(Move::D)));
            deque.push_back((cube.turn(Move::Di), Face::D, last_face, moves.append_move(Move::Di)));
            deque.push_back((cube.turn(Move::D2), Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
    hashmap.insert(Cube3x3Simple::new(), Vec::new()); //include g1 state itself
    return hashmap;
}

//generate lookup hashmap with specified depth from solved state
pub fn gen_lookups_3x3_phase_2(depth: usize) -> HashMap<Cube3x3, Vec<Move>> {
    let mut hashmap = HashMap::new();
    let mut deque: VecDeque<(Cube3x3, Face, Face, Vec<Move>)> = VecDeque::from(vec!((Cube3x3::new(), Face::None, Face::None, Vec::new())));
    while deque.len() > 0 {
        let (cube, last_face, next_last_face, moves) = deque.pop_front().unwrap(); //destructure current state

        if hashmap.contains_key(&cube) {
            continue;
        } else {
            hashmap.insert(cube, moves.clone());
        }
        if moves.len() == depth {
            continue;
        }
        //push next possible cube states
        if last_face != Face::F && !(last_face == Face::B && next_last_face == Face::F) {
            deque.push_back((cube.turn(Move::F2), Face::F, last_face, moves.append_move(Move::F2)));
        }
        if last_face != Face::R && !(last_face == Face::L && next_last_face == Face::R) {
            deque.push_back((cube.turn(Move::R2), Face::R, last_face, moves.append_move(Move::R2)));
        }
        if last_face != Face::U && !(last_face == Face::D && next_last_face == Face::U) {
            deque.push_back((cube.turn(Move::U), Face::U, last_face, moves.append_move(Move::U)));
            deque.push_back((cube.turn(Move::Ui), Face::U, last_face, moves.append_move(Move::Ui)));
            deque.push_back((cube.turn(Move::U2), Face::U, last_face, moves.append_move(Move::U2)));
        }
        if last_face != Face::B && !(last_face == Face::F && next_last_face == Face::B) {
            deque.push_back((cube.turn(Move::B2), Face::B, last_face, moves.append_move(Move::B2)));
        }
        if last_face != Face::L && !(last_face == Face::R && next_last_face == Face::L) {
            deque.push_back((cube.turn(Move::L2), Face::L, last_face, moves.append_move(Move::L2)));
        }
        if last_face != Face::D && !(last_face == Face::U && next_last_face == Face::D) {
            deque.push_back((cube.turn(Move::D), Face::D, last_face, moves.append_move(Move::D)));
            deque.push_back((cube.turn(Move::Di), Face::D, last_face, moves.append_move(Move::Di)));
            deque.push_back((cube.turn(Move::D2), Face::D, last_face, moves.append_move(Move::D2)));
        }
    }
    hashmap.insert(Cube3x3::new(), Vec::new()); //include solved cube
    return hashmap;
}
