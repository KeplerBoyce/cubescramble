//color possibilities of a sticker
#[derive(Copy, Clone, PartialEq)]
pub enum Color {
    G, //green
    R, //red
    W, //white
    B, //blue
    O, //orange
    Y, //yellow
}
//possibilities for the face a sticker is on
#[derive(Copy, Clone)]
pub enum Face {
    F, //front
    R, //right
    U, //up
    B, //back
    L, //left
    D, //down
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
#[derive(PartialEq)]
pub struct Cube3x3 {
    stickers: [Color; 54], //F, R, U, B, L, D; left to right, bottom to top on each face
}
//functions for making a cube, turning, and checking if it is solved
impl Cube3x3 {
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
    fn cycle(mut self, points: [usize; 4]) -> Cube3x3 {
        let temp: Color = self.stickers[points[3]];
        self.stickers[points[1]] = self.stickers[points[0]];
        self.stickers[points[2]] = self.stickers[points[1]];
        self.stickers[points[3]] = self.stickers[points[2]];
        self.stickers[points[0]] = temp;
        return self;
    }
    fn swap(mut self, points: [usize; 2]) -> Cube3x3 {
        let temp: Color = self.stickers[points[1]];
        self.stickers[points[1]] = self.stickers[points[0]];
        self.stickers[points[0]] = temp;
        return self;
    }
    pub fn turn(mut self, m: Move) -> Cube3x3 {
        match m {
            F => {self = self.cycle([0, 6, 8, 2]).cycle([18, 15, 53, 38]).cycle([20, 9, 51, 44]).cycle([1, 3, 7, 5]).cycle([19, 12, 52, 41])},
            R => {},
            U => {},
            B => {},
            L => {},
            D => {},
            Fi => {self = self.cycle([2, 8, 6, 0]).cycle([38, 53, 15, 18]).cycle([44, 51, 9, 20]).cycle([5, 7, 3, 1]).cycle([41, 52, 12, 19])},
            Ri => {},
            Ui => {},
            Bi => {},
            Li => {},
            Di => {},
            F2 => {self = self.swap([0, 8]).swap([6, 2]).swap([18, 53]).swap([15, 38]).swap([20, 51]).swap([9, 44]).swap([1, 7]).swap([3, 5]).swap([19, 52]).swap([12, 41])},
            R2 => {},
            U2 => {},
            B2 => {},
            L2 => {},
            D2 => {},
        }
        return self;
    }
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
