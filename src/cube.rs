#[allow(non_camel_case_types)]
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
    edges: [i8; 12], //order: UL clockwise, DL clockwise, FL, FR, BL, BR, +12 if flipped
    corners: [i8; 8], //order: UFL clockwise, DFL clockwise, +8 for CW twist, +16 for CCW twist
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
    pub fn turn(mut self, m: Move) -> Cube3x3 {
        let e = self.edges;
        let c = self.corners;
        match m {
            Move::F => {self.edges = [e[0], e[1], e[2], (e[8] + 12) % 24, e[4], (e[9] + 12) % 24, e[6], e[7], (e[5] + 12) % 24, (e[3] + 12) % 24, e[10], e[11]];
                        self.corners = [(c[4] + 8) % 24, c[1], c[2], (c[0] + 16) % 24, (c[5] + 16) % 24, (c[3] + 8) % 24, c[6], c[7]]},
            Move::R => {self.edges = [e[0], e[1], e[9], e[3], e[4], e[5], e[11], e[7], e[8], e[6], e[10], e[2]];
                        self.corners = [c[0], c[1], (c[3] + 16) % 24, (c[5] + 8) % 24, c[4], (c[6] + 16) % 24, (c[2] + 8) % 24, c[7]]},
            Move::U => {self.edges = [e[3], e[0], e[1], e[2], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                        self.corners = [c[3], c[0], c[1], c[2], c[4], c[5], c[6], c[7]]},
            Move::B => {self.edges = [e[0], (e[11] + 12) % 24, e[2], e[3], e[4], e[5], e[6], (e[10] + 12) % 24, e[8], e[9], (e[1] + 12) % 24, (e[7] + 12) % 24];
                        self.corners = [c[0], (c[2] + 16) % 24, (c[6] + 8) % 24, c[3], c[4], c[5], (c[7] + 16) % 24, (c[1] + 8) % 24]},
            Move::L => {self.edges = [e[10], e[1], e[2], e[3], e[8], e[5], e[6], e[7], e[0], e[9], e[4], e[11]];
                        self.corners = [(c[1] + 16) % 24, (c[7] + 8) % 24, c[2], c[3], (c[0] + 8) % 24, c[5], c[6], (c[4] + 16) % 24]},
            Move::D => {self.edges = [e[0], e[1], e[2], e[3], e[7], e[4], e[5], e[6], e[8], e[9], e[10], e[11]];
                         self.corners = [c[0], c[1], c[2], c[3], c[7], c[4], c[5], c[6]]},
            Move::Fi => {self.edges = [e[0], e[1], e[2], (e[9] + 12) % 24, e[4], (e[8] + 12) % 24, e[6], e[7], (e[3] + 12) % 24, (e[5] + 12) % 24, e[10], e[11]];
                         self.corners = [(c[3] + 8) % 24, c[1], c[2], (c[5] + 16) % 24, (c[0] + 16) % 24, (c[4] + 8) % 24, c[6], c[7]]},
            Move::Ri => {self.edges = [e[0], e[1], e[11], e[3], e[4], e[5], e[9], e[7], e[8], e[2], e[10], e[6]];
                         self.corners = [c[0], c[1], (c[6] + 16) % 24, (c[2] + 8) % 24, c[4], (c[3] + 16) % 24, (c[5] + 8) % 24, c[7]]},
            Move::Ui => {self.edges = [e[1], e[2], e[3], e[0], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                         self.corners = [c[1], c[2], c[3], c[0], c[4], c[5], c[6], c[7]]},
            Move::Bi => {self.edges = [e[0], (e[10] + 12) % 24, e[2], e[3], e[4], e[5], e[6], (e[11] + 12) % 24, e[8], e[9], (e[7] + 12) % 24, (e[1] + 12) % 24];
                         self.corners = [c[0], (c[7] + 16) % 24, (c[1] + 8) % 24, c[3], c[4], c[5], (c[2] + 16) % 24, (c[6] + 8) % 24]},
            Move::Li => {self.edges = [e[8], e[1], e[2], e[3], e[10], e[5], e[6], e[7], e[4], e[9], e[0], e[11]];
                         self.corners = [(c[4] + 16) % 24, (c[0] + 8) % 24, c[2], c[3], (c[7] + 8) % 24, c[5], c[6], (c[1] + 16) % 24]},
            Move::Di => {self.edges = [e[0], e[1], e[2], e[3], e[5], e[6], e[7], e[4], e[8], e[9], e[10], e[11]];
                         self.corners = [c[0], c[1], c[2], c[3], c[5], c[6], c[7], c[4]]},
            Move::F2 => {self.edges = [e[0], e[1], e[2], e[5], e[4], e[3], e[6], e[7], e[9], e[8], e[10], e[11]];
                         self.corners = [c[5], c[1], c[2], c[4], c[3], c[0], c[6], c[7]]},
            Move::R2 => {self.edges = [e[0], e[1], e[6], e[3], e[4], e[5], e[2], e[7], e[8], e[11], e[10], e[9]];
                         self.corners = [c[0], c[1], c[5], c[6], c[4], c[2], c[3], c[7]]},
            Move::U2 => {self.edges = [e[2], e[3], e[0], e[1], e[4], e[5], e[6], e[7], e[8], e[9], e[10], e[11]];
                         self.corners = [c[2], c[3], c[0], c[1], c[4], c[5], c[6], c[7]]},
            Move::B2 => {self.edges = [e[0], e[7], e[2], e[3], e[4], e[5], e[6], e[1], e[8], e[9], e[11], e[10]];
                         self.corners = [c[0], c[6], c[7], c[3], c[4], c[5], c[1], c[2]]},
            Move::L2 => {self.edges = [e[4], e[1], e[2], e[3], e[0], e[5], e[6], e[7], e[10], e[9], e[8], e[11]];
                         self.corners = [c[7], c[4], c[2], c[3], c[1], c[5], c[6], c[0]]},
            Move::D2 => {self.edges = [e[0], e[1], e[2], e[3], e[6], e[7], e[4], e[5], e[8], e[9], e[10], e[11]];
                         self.corners = [c[0], c[1], c[2], c[3], c[6], c[7], c[4], c[5]]},
        }
        return self;
    }

    //check if the cube is solved
    pub fn check(self) -> bool {
        return self == Self::new();
    }

    //check if the cube is in G1 (domino) state
    pub fn check_g1(self) -> bool {
        //make sure U and D faces are only white and yellow
        for x in vec![18, 19, 20, 21, 22, 23, 24, 25, 26, 45, 46, 47, 48, 49, 50, 51, 52, 53].into_iter() {
            if self.stickers[x] != Color::W && self.stickers[x] != Color::Y {
                return false;
            }
        }
        //if the above is true, the E slice edges are already in the proper state; no need to check
        return true;
    }

    //generate a random cube state
    pub fn scramble(mut self) -> Cube3x3 {
        let mut rng = rand::thread_rng();
        //locations of corner stickers and edge stickers; faces of corner in clockwise order
        let corner_locs: [[usize; 3]; 8] = [[0, 51, 38], [2, 9, 53], [6, 44, 18], [8, 20, 15], [27, 47, 11], [29, 36, 45], [33, 17, 26], [35, 24, 42]];
        let edge_locs: [[usize; 2]; 12] = [[1, 52], [3, 41], [7, 19], [5, 12], [23, 16], [50, 10], [48, 37], [21, 43], [28, 46], [30, 14], [34, 25], [32, 39]];
        let mut swaps: usize = 0; //make sure number of swaps is even, as otherwise cube is unsolvable
        
        //perform fisher-yates shuffle on corners
        for i in (1..8).rev() {
            let target: usize = rng.gen_range(0..(i + 1));
            let rotation: usize = rng.gen_range(0..3); //swap with a random sticker of the target
            //make sure not swapping with self, as this either does nothing or twists corner
            if target != i {
                self = self.swap([corner_locs[i][0], corner_locs[target][rotation]])
                    .swap([corner_locs[i][1], corner_locs[target][(1 + rotation) % 3]])
                    .swap([corner_locs[i][2], corner_locs[target][(2 + rotation) % 3]]);
                swaps += 1;
            }
        }
        //perform fisher-yates shuffle on edges
        for i in (1..12).rev() {
            let target: usize = rng.gen_range(0..(i + 1));
            let rotation: usize = rng.gen_range(0..2); //swap with a random sticker of the target
            //make sure not swapping with self, as this either does nothing or flips edge
            //skip final swap if it would make total number of swaps odd
            if target != i && !(i == 1 && swaps % 2 == 0) {
                self = self.swap([edge_locs[i][0], edge_locs[target][rotation]])
                    .swap([edge_locs[i][1], edge_locs[target][(1 + rotation) % 2]]);
                swaps += 1;
            }
        }
        return self;
    }
}
