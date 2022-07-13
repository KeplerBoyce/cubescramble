mod cube;
use cube::*;

fn main() {
    let mut cube3x3: Cube3x3 = Cube3x3::new();
    cube3x3 = cube3x3.turn(Move::R);
    cube3x3 = cube3x3.turn(Move::U);
    cube3x3 = cube3x3.turn(Move::Ri);
    cube3x3 = cube3x3.turn(Move::Fi);
    cube3x3 = cube3x3.turn(Move::R);
    cube3x3 = cube3x3.turn(Move::U);
    cube3x3 = cube3x3.turn(Move::Ri);
    cube3x3 = cube3x3.turn(Move::Ui);
    cube3x3 = cube3x3.turn(Move::Ri);
    cube3x3 = cube3x3.turn(Move::F);
    cube3x3 = cube3x3.turn(Move::R2);
    cube3x3 = cube3x3.turn(Move::Ui);
    cube3x3 = cube3x3.turn(Move::Ri);
    cube3x3 = cube3x3.turn(Move::Ui);
    println!("{}", cube3x3.check());
}
