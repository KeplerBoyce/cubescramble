#![allow(non_camel_case_types)]
#![allow(dead_code)]
use rand::Rng;
use serde::{Serialize, Deserialize};

//color possibilities of a sticker
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Face {
    F,
    R,
    U,
    B,
    L,
    D,
    None,
}

//move possibilities
#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum Move {
    F, //front CW
    R, //right CW
    U, //up CW
    B, //back CW
    L, //left CW
    D, //down cW
    Fi, //front CCW
    Ri, //right CCW
    Ui, //up CCW
    Bi, //back CCW
    Li, //left CCW
    Di, //down CCW
    F2, //front 180
    R2, //right 180
    U2, //up 180
    B2, //back 180
    L2, //left 180
    D2, //down 180
}

impl Move {
    //fn to invert direction of move
    pub fn invert(&mut self) {
        match self {
            Move::F => {*self = Move::Fi}
            Move::Fi => {*self = Move::F}
            Move::R => {*self = Move::Ri}
            Move::Ri => {*self = Move::R}
            Move::U => {*self = Move::Ui}
            Move::Ui => {*self = Move::U}
            Move::B => {*self = Move::Bi}
            Move::Bi => {*self = Move::B}
            Move::L => {*self = Move::Li}
            Move::Li => {*self = Move::L}
            Move::D => {*self = Move::Di}
            Move::Di => {*self = Move::D}
            _ => {}
        }
    }
}

//struct for 3x3 cube
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cube3x3 {
    edges: [u8; 12], //order: UL clockwise, DL clockwise, FL, FR, BL, BR, +12 if flipped
    corners: [u8; 8], //order: UFL clockwise, DFL clockwise, +8 for CW twist, +16 for CCW twist
}

//functions for making a cube, turning, and checking if it is solved
impl Cube3x3 {
    //returns a new, solved Cube3x3
    pub fn new() -> Cube3x3 {
        return Cube3x3 {
            edges: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            corners: [0, 1, 2, 3, 4, 5, 6, 7],
        };
    }

    //applies a move and returns a new Cube3x3
    pub fn turn(&self, m: Move) -> Cube3x3 {
        let mut e = self.edges;
        let mut c = self.corners;
        match m {
            Move::F => {e = [e[0], e[1], e[2], (e[8] + 12) % 24, e[4], (e[9] + 12) % 24, e[6], e[7], (e[5] + 12) % 24, (e[3] + 12) % 24, e[10], e[11]];
                        c = [(c[4] + 8) % 24, c[1], c[2], (c[0] + 16) % 24, (c[5] + 16) % 24, (c[3] + 8) % 24, c[6], c[7]]},
            Move::R => {e = [e[0], e[1], e[9], e[3], e[4], e[5], e[11], e[7], e[8], e[6], e[10], e[2]];
                        c = [c[0], c[1], (c[3] + 16) % 24, (c[5] + 8) % 24, c[4], (c[6] + 16) % 24, (c[2] + 8) % 24, c[7]]},
            Move::U => {e = [e[3], e[0], e[1], e[2], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                        c = [c[3], c[0], c[1], c[2], c[4], c[5], c[6], c[7]]},
            Move::B => {e = [e[0], (e[11] + 12) % 24, e[2], e[3], e[4], e[5], e[6], (e[10] + 12) % 24, e[8], e[9], (e[1] + 12) % 24, (e[7] + 12) % 24];
                        c = [c[0], (c[2] + 16) % 24, (c[6] + 8) % 24, c[3], c[4], c[5], (c[7] + 16) % 24, (c[1] + 8) % 24]},
            Move::L => {e = [e[10], e[1], e[2], e[3], e[8], e[5], e[6], e[7], e[0], e[9], e[4], e[11]];
                        c = [(c[1] + 16) % 24, (c[7] + 8) % 24, c[2], c[3], (c[0] + 8) % 24, c[5], c[6], (c[4] + 16) % 24]},
            Move::D => {e = [e[0], e[1], e[2], e[3], e[7], e[4], e[5], e[6], e[8], e[9], e[10], e[11]];
                         c = [c[0], c[1], c[2], c[3], c[7], c[4], c[5], c[6]]},
            Move::Fi => {e = [e[0], e[1], e[2], (e[9] + 12) % 24, e[4], (e[8] + 12) % 24, e[6], e[7], (e[3] + 12) % 24, (e[5] + 12) % 24, e[10], e[11]];
                         c = [(c[3] + 8) % 24, c[1], c[2], (c[5] + 16) % 24, (c[0] + 16) % 24, (c[4] + 8) % 24, c[6], c[7]]},
            Move::Ri => {e = [e[0], e[1], e[11], e[3], e[4], e[5], e[9], e[7], e[8], e[2], e[10], e[6]];
                         c = [c[0], c[1], (c[6] + 16) % 24, (c[2] + 8) % 24, c[4], (c[3] + 16) % 24, (c[5] + 8) % 24, c[7]]},
            Move::Ui => {e = [e[1], e[2], e[3], e[0], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                         c = [c[1], c[2], c[3], c[0], c[4], c[5], c[6], c[7]]},
            Move::Bi => {e = [e[0], (e[10] + 12) % 24, e[2], e[3], e[4], e[5], e[6], (e[11] + 12) % 24, e[8], e[9], (e[7] + 12) % 24, (e[1] + 12) % 24];
                         c = [c[0], (c[7] + 16) % 24, (c[1] + 8) % 24, c[3], c[4], c[5], (c[2] + 16) % 24, (c[6] + 8) % 24]},
            Move::Li => {e = [e[8], e[1], e[2], e[3], e[10], e[5], e[6], e[7], e[4], e[9], e[0], e[11]];
                         c = [(c[4] + 16) % 24, (c[0] + 8) % 24, c[2], c[3], (c[7] + 8) % 24, c[5], c[6], (c[1] + 16) % 24]},
            Move::Di => {e = [e[0], e[1], e[2], e[3], e[5], e[6], e[7], e[4], e[8], e[9], e[10], e[11]];
                         c = [c[0], c[1], c[2], c[3], c[5], c[6], c[7], c[4]]},
            Move::F2 => {e = [e[0], e[1], e[2], e[5], e[4], e[3], e[6], e[7], e[9], e[8], e[10], e[11]];
                         c = [c[5], c[1], c[2], c[4], c[3], c[0], c[6], c[7]]},
            Move::R2 => {e = [e[0], e[1], e[6], e[3], e[4], e[5], e[2], e[7], e[8], e[11], e[10], e[9]];
                         c = [c[0], c[1], c[5], c[6], c[4], c[2], c[3], c[7]]},
            Move::U2 => {e = [e[2], e[3], e[0], e[1], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                         c = [c[2], c[3], c[0], c[1], c[4], c[5], c[6], c[7]]},
            Move::B2 => {e = [e[0], e[7], e[2], e[3], e[4], e[5], e[6], e[1], e[8], e[9], e[11], e[10]];
                         c = [c[0], c[6], c[7], c[3], c[4], c[5], c[1], c[2]]},
            Move::L2 => {e = [e[4], e[1], e[2], e[3], e[0], e[5], e[6], e[7], e[10], e[9], e[8], e[11]];
                         c = [c[7], c[4], c[2], c[3], c[1], c[5], c[6], c[0]]},
            Move::D2 => {e = [e[0], e[1], e[2], e[3], e[6], e[7], e[4], e[5], e[8], e[9], e[10], e[11]];
                         c = [c[0], c[1], c[2], c[3], c[6], c[7], c[4], c[5]]},
        }
        return Cube3x3 {
            edges: e,
            corners: c,
        };
    }

    //generate a random cube state
    pub fn scramble(&mut self) {
        let mut rng = rand::thread_rng();
        let mut swaps: usize = 0;
        
        //perform fisher-yates shuffle on corners
        for i in (1..8).rev() {
            let target: usize = rng.gen_range(0..(i + 1));
            let rotation: u8 = rng.gen_range(0..3); //swap with a random sticker of the target
            //make sure not swapping with self, as this either does nothing or twists corner
            if target != i {
                let temp = self.corners[i];
                self.corners[i] = self.corners[target];
                self.corners[target] = temp;
                self.corners[i] = (self.corners[i] + 8 * rotation) % 24; //rotate in opposite directions
                self.corners[target] = (self.corners[target] + 16 * rotation) % 24;
                swaps += 1;
            }
        }
        //repeat for edges
        for i in (1..12).rev() {
            let target: usize = rng.gen_range(0..(i + 1));
            let rotation: u8 = rng.gen_range(0..2); //swap with a random sticker of the target
            //make sure not swapping with self, as this either does nothing or flips edge
            //skip final swap if it would make total number of swaps odd
            if target != i && !(i == 1 && swaps % 2 == 0) {
                let temp = self.edges[i];
                self.edges[i] = self.edges[target];
                self.edges[target] = temp;
                self.edges[i] = (self.edges[i] + 12 * rotation) % 24;
                self.edges[target] = (self.edges[target] + 12 * rotation) % 24;
                swaps += 1;
            }
        }
    }

    //convert to simplified state for g1 search
    pub fn simplify(&self) -> Cube3x3Simple {
        let mut new_edges = self.edges;
        let mut new_corners = self.corners;
        for e in &mut new_edges {
            if [0, 1, 2, 3, 4, 5, 6, 7].contains(e) {
                *e = 0;
            } else if [8, 9, 10, 11].contains(e) {
                *e = 1;
            } else if [12, 13, 14, 15, 16, 17, 18, 19].contains(e) {
                *e = 2;
            } else {
                *e = 3;
            }
        }
        for c in &mut new_corners {
            *c = *c / 8;
        }
        return Cube3x3Simple {
            edges: new_edges,
            corners: new_corners,
        }
    }
}

//struct for simplified state of 3x3 cube for g1 search
#[derive(Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Cube3x3Simple {
    edges: [u8; 12], //permutation of E slice edges and orientation of UD slice edges
    corners: [u8; 8], //orientation of all corners (0=correct, 1=CW twist, 2=CCW twist)
}

//functions for making a cube, turning, and checking if it is solved
impl Cube3x3Simple {
    //returns a new, solved Cube3x3
    pub fn new() -> Cube3x3Simple {
        return Cube3x3Simple {
            edges: [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1],
            corners: [0, 0, 0, 0, 0, 0, 0, 0],
        };
    }

    //applies a move and returns a new Cube3x3
    pub fn turn(&self, m: Move) -> Cube3x3Simple {
        let mut e = self.edges;
        let mut c = self.corners;
        match m {
            Move::F => {e = [e[0], e[1], e[2], (e[8] + 2) % 4, e[4], (e[9] + 2) % 4, e[6], e[7], (e[5] + 2) % 4, (e[3] + 2) % 4, e[10], e[11]];
                        c = [(c[4] + 1) % 3, c[1], c[2], (c[0] + 2) % 3, (c[5] + 2) % 3, (c[3] + 1) % 3, c[6], c[7]]},
            Move::R => {e = [e[0], e[1], e[9], e[3], e[4], e[5], e[11], e[7], e[8], e[6], e[10], e[2]];
                        c = [c[0], c[1], (c[3] + 2) % 3, (c[5] + 1) % 3, c[4], (c[6] + 2) % 3, (c[2] + 1) % 3, c[7]]},
            Move::U => {e = [e[3], e[0], e[1], e[2], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                        c = [c[3], c[0], c[1], c[2], c[4], c[5], c[6], c[7]]},
            Move::B => {e = [e[0], (e[11] + 2) % 4, e[2], e[3], e[4], e[5], e[6], (e[10] + 2) % 4, e[8], e[9], (e[1] + 2) % 4, (e[7] + 2) % 4];
                        c = [c[0], (c[2] + 2) % 3, (c[6] + 1) % 3, c[3], c[4], c[5], (c[7] + 2) % 3, (c[1] + 1) % 3]},
            Move::L => {e = [e[10], e[1], e[2], e[3], e[8], e[5], e[6], e[7], e[0], e[9], e[4], e[11]];
                        c = [(c[1] + 2) % 3, (c[7] + 1) % 3, c[2], c[3], (c[0] + 1) % 3, c[5], c[6], (c[4] + 2) % 3]},
            Move::D => {e = [e[0], e[1], e[2], e[3], e[7], e[4], e[5], e[6], e[8], e[9], e[10], e[11]];
                         c = [c[0], c[1], c[2], c[3], c[7], c[4], c[5], c[6]]},
            Move::Fi => {e = [e[0], e[1], e[2], (e[9] + 2) % 4, e[4], (e[8] + 2) % 4, e[6], e[7], (e[3] + 2) % 4, (e[5] + 2) % 4, e[10], e[11]];
                         c = [(c[3] + 1) % 3, c[1], c[2], (c[5] + 2) % 3, (c[0] + 2) % 3, (c[4] + 1) % 3, c[6], c[7]]},
            Move::Ri => {e = [e[0], e[1], e[11], e[3], e[4], e[5], e[9], e[7], e[8], e[2], e[10], e[6]];
                         c = [c[0], c[1], (c[6] + 2) % 3, (c[2] + 1) % 3, c[4], (c[3] + 2) % 3, (c[5] + 1) % 3, c[7]]},
            Move::Ui => {e = [e[1], e[2], e[3], e[0], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                         c = [c[1], c[2], c[3], c[0], c[4], c[5], c[6], c[7]]},
            Move::Bi => {e = [e[0], (e[10] + 2) % 4, e[2], e[3], e[4], e[5], e[6], (e[11] + 2) % 4, e[8], e[9], (e[7] + 2) % 4, (e[1] + 2) % 4];
                         c = [c[0], (c[7] + 2) % 3, (c[1] + 1) % 3, c[3], c[4], c[5], (c[2] + 2) % 3, (c[6] + 1) % 3]},
            Move::Li => {e = [e[8], e[1], e[2], e[3], e[10], e[5], e[6], e[7], e[4], e[9], e[0], e[11]];
                         c = [(c[4] + 2) % 3, (c[0] + 1) % 3, c[2], c[3], (c[7] + 1) % 3, c[5], c[6], (c[1] + 2) % 3]},
            Move::Di => {e = [e[0], e[1], e[2], e[3], e[5], e[6], e[7], e[4], e[8], e[9], e[10], e[11]];
                         c = [c[0], c[1], c[2], c[3], c[5], c[6], c[7], c[4]]},
            Move::F2 => {e = [e[0], e[1], e[2], e[5], e[4], e[3], e[6], e[7], e[9], e[8], e[10], e[11]];
                         c = [c[5], c[1], c[2], c[4], c[3], c[0], c[6], c[7]]},
            Move::R2 => {e = [e[0], e[1], e[6], e[3], e[4], e[5], e[2], e[7], e[8], e[11], e[10], e[9]];
                         c = [c[0], c[1], c[5], c[6], c[4], c[2], c[3], c[7]]},
            Move::U2 => {e = [e[2], e[3], e[0], e[1], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                         c = [c[2], c[3], c[0], c[1], c[4], c[5], c[6], c[7]]},
            Move::B2 => {e = [e[0], e[7], e[2], e[3], e[4], e[5], e[6], e[1], e[8], e[9], e[11], e[10]];
                         c = [c[0], c[6], c[7], c[3], c[4], c[5], c[1], c[2]]},
            Move::L2 => {e = [e[4], e[1], e[2], e[3], e[0], e[5], e[6], e[7], e[10], e[9], e[8], e[11]];
                         c = [c[7], c[4], c[2], c[3], c[1], c[5], c[6], c[0]]},
            Move::D2 => {e = [e[0], e[1], e[2], e[3], e[6], e[7], e[4], e[5], e[8], e[9], e[10], e[11]];
                         c = [c[0], c[1], c[2], c[3], c[6], c[7], c[4], c[5]]},
        }
        return Cube3x3Simple {
            edges: e,
            corners: c,
        };
    }
}
