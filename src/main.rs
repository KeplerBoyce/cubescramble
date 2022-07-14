mod cube;
mod search;
use cube::*;
use search::*;

fn main() {
    let mut cube3x3: Cube3x3 = Cube3x3::new();
    cube3x3 = cube3x3.scramble();
    search3x3(cube3x3);
}
