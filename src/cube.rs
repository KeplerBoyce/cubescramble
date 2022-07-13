//color possibilities of a sticker
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Color {
    G, //green
    R, //red
    W, //white
    B, //blue
    O, //orange
    Y, //yellow
}
//move possibilities
#[derive(Copy, Clone)]
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
    stickers: [Color; 54], //F, R, U, B, L, D; left to right, bottom to top on each face
}
//functions for making a cube, turning, and checking if it is solved
impl Cube3x3 {
    //returns a new, solved Cube3x3
    pub fn new() -> Cube3x3 {
        return Cube3x3 {
            stickers: [
                Color::G, Color::G, Color::G, Color::G, Color::G, Color::G, Color::G, Color::G, Color::G,
                Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R, Color::R,
                Color::W, Color::W, Color::W, Color::W, Color::W, Color::W, Color::W, Color::W, Color::W,
                Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B, Color::B,
                Color::O, Color::O, Color::O, Color::O, Color::O, Color::O, Color::O, Color::O, Color::O,
                Color::Y, Color::Y, Color::Y, Color::Y, Color::Y, Color::Y, Color::Y, Color::Y, Color::Y,
            ]
        };
    }
    //print color of every sticker in order (for debugging)
    pub fn print_colors(self) {
        for (i, x) in self.stickers.iter().enumerate() {
            println!("{} {:?}", i, x);
        }
    }
    //cycles 4 stickers and returns a new Cube3x3
    fn cycle(mut self, points: [usize; 4]) -> Cube3x3 {
        let temp: Color = self.stickers[points[3]];
        self.stickers[points[3]] = self.stickers[points[2]];
        self.stickers[points[2]] = self.stickers[points[1]];
        self.stickers[points[1]] = self.stickers[points[0]];
        self.stickers[points[0]] = temp;
        return self;
    }
    //swaps 2 stickers and returns a new Cube3x3
    fn swap(mut self, points: [usize; 2]) -> Cube3x3 {
        let temp: Color = self.stickers[points[1]];
        self.stickers[points[1]] = self.stickers[points[0]];
        self.stickers[points[0]] = temp;
        return self;
    }
    //applies a move and returns a new Cube3x3
    pub fn turn(mut self, m: Move) -> Cube3x3 {
        match m {
            Move::F => {self = self.cycle([0, 6, 8, 2]).cycle([18, 15, 53, 38]).cycle([20, 9, 51, 44]).cycle([1, 3, 7, 5]).cycle([19, 12, 52, 41])},
            Move::R => {self = self.cycle([9, 15, 17, 11]).cycle([20, 33, 47, 2]).cycle([26, 27, 53, 8]).cycle([10, 12, 16, 14]).cycle([23, 30, 50, 5])},
            Move::U => {self = self.cycle([18, 24, 26, 20]).cycle([35, 17, 8, 44]).cycle([33, 15, 6, 42]).cycle([19, 21, 25, 23]).cycle([34, 16, 7, 43])},
            Move::B => {self = self.cycle([27, 33, 35, 29]).cycle([26, 42, 45, 11]).cycle([24, 36, 47, 17]).cycle([28, 30, 34, 32]).cycle([25, 39, 46, 14])},
            Move::L => {self = self.cycle([36, 42, 44, 38]).cycle([24, 6, 15, 29]).cycle([18, 0, 45, 35]).cycle([37, 39, 43, 41]).cycle([21, 3, 48, 32])},
            Move::D => {self = self.cycle([45, 51, 53, 47]).cycle([0, 9, 27, 36]).cycle([2, 11, 29, 38]).cycle([46, 48, 52, 50]).cycle([1, 10, 28, 37])},
            Move::Fi => {self = self.cycle([2, 8, 6, 0]).cycle([38, 53, 15, 18]).cycle([44, 51, 9, 20]).cycle([5, 7, 3, 1]).cycle([41, 52, 12, 19])},
            Move::Ri => {self = self.cycle([11, 17, 15, 9]).cycle([2, 47, 33, 20]).cycle([8, 53, 27, 26]).cycle([14, 16, 12, 10]).cycle([5, 50, 30, 23])},
            Move::Ui => {self = self.cycle([20, 26, 24, 18]).cycle([44, 8, 17, 35]).cycle([42, 6, 15, 33]).cycle([23, 25, 21, 19]).cycle([43, 7, 16, 34])},
            Move::Bi => {self = self.cycle([29, 35, 33, 27]).cycle([11, 45, 42, 26]).cycle([17, 47, 36, 24]).cycle([32, 34, 30, 28]).cycle([14, 46, 39, 25])},
            Move::Li => {self = self.cycle([38, 44, 42, 36]).cycle([29, 15, 6, 24]).cycle([35, 45, 0, 18]).cycle([41, 43, 39, 37]).cycle([32, 48, 3, 21])},
            Move::Di => {self = self.cycle([47, 53, 51, 45]).cycle([36, 27, 9, 0]).cycle([38, 29, 11, 2]).cycle([50, 52, 48, 46]).cycle([37, 28, 10, 1])},
            Move::F2 => {self = self.swap([0, 8]).swap([6, 2]).swap([18, 53]).swap([15, 38]).swap([20, 51]).swap([9, 44]).swap([1, 7]).swap([3, 5]).swap([19, 52]).swap([12, 41])},
            Move::R2 => {self = self.swap([9, 17]).swap([15, 11]).swap([20, 47]).swap([33, 2]).swap([26, 53]).swap([27, 8]).swap([10, 16]).swap([12, 14]).swap([23, 50]).swap([30, 5])},
            Move::U2 => {self = self.swap([18, 26]).swap([24, 20]).swap([35, 8]).swap([17, 44]).swap([33, 6]).swap([15, 42]).swap([19, 25]).swap([21, 23]).swap([34, 7]).swap([16, 43])},
            Move::B2 => {self = self.swap([27, 35]).swap([33, 29]).swap([26, 45]).swap([42, 11]).swap([24, 47]).swap([36, 17]).swap([28, 34]).swap([30, 32]).swap([25, 46]).swap([39, 14])},
            Move::L2 => {self = self.swap([36, 44]).swap([42, 38]).swap([24, 15]).swap([6, 29]).swap([18, 45]).swap([0, 35]).swap([37, 43]).swap([39, 41]).swap([21, 48]).swap([3, 32])},
            Move::D2 => {self = self.swap([45, 53]).swap([51, 47]).swap([0, 27]).swap([9, 36]).swap([2, 29]).swap([11, 38]).swap([46, 52]).swap([48, 50]).swap([1, 28]).swap([10, 37])},
        }
        return self;
    }
    //function to check if the cube is solved
    pub fn check(self) -> bool {
        // let colors: [Color; 6] = [Color::G, Color::R, Color::W, Color::B, Color::O, Color::Y];
        // for i in 0..6 {
        //     for j in 0..9 {
        //         if self.stickers[i * 9 + j] != colors[i] {
        //             return false;
        //         }
        //     }
        // }
        // return true;
        return self == Self::new();
    }
}
