#![allow(non_camel_case_types)]
#![allow(dead_code)]
use rand::Rng;

//color possibilities of a sticker
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    G, //green
    R, //red
    W, //white
    B, //blue
    O, //orange
    Y, //yellow
    None,
}

//move possibilities
#[derive(Copy, Clone, Debug)]
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

//type for layers
#[derive(Copy, Clone, PartialEq)]
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

    //print edges and corners for debugging
    pub fn print_pieces(&self) {
        println!("--- Printing pieces ---");
        for (i, e) in self.edges.iter().enumerate() {
            println!("Edge {}: {}", i, e);
        }
        for (i, c) in self.corners.iter().enumerate() {
            println!("Corner {}: {}", i, c);
        }
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

    //check if the cube is solved
    pub fn check(&self) -> bool {
        return *self == Self::new();
    }

    //check if the cube is in G1 (domino) state
    pub fn check_g1(&self) -> bool {
        //make sure all pieces are oriented and U and D faces are only white and yellow
        for x in self.corners {
            if x >= 8 {
                return false;
            }
        }
        for (i, x) in self.edges.iter().enumerate() {
            if x >= &12 {
                return false;
            }
            if i >= 8 && x < &8 { //check that E slice edges are in E slice
                return false;
            }
        }
        //if the above are true, the E slice edges are already in the proper state; no need to check
        return true;
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
                self.corners[i] = (self.corners[i] + 8 * rotation) & 24; //rotate in opposite directions
                self.corners[target] = (self.corners[target] + 16 * rotation) & 24;
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
                self.edges[i] = (self.edges[i] + 12 * rotation) & 24;
                self.edges[target] = (self.edges[target] + 12 * rotation) & 24;
                swaps += 1;
            }
        }
    }
}
