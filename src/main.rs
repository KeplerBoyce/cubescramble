mod cube;
use cube::*;

fn main() {
    let mut cube3x3: Cube3x3 = Cube3x3::new();
    cube3x3 = cube3x3.turn(Move::F2);
    cube3x3 = cube3x3.turn(Move::F2);
    println!("{}", cube3x3.check());
}
