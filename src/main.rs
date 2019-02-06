use cubers::cube;
use cubers::cube::Move::*;

fn main() {
    println!("Hello, world!");
    let f1 = cube::Subst::from(cube::Move::F1);
    let r1 = cube::Subst::from(cube::Move::R1);
    let d1 = cube::Subst::from(cube::Move::D1);
    let l1 = cube::Subst::from(cube::Move::L1);

    println!(
        "{:?}",
        cube::RubikCube::new(d1 * f1 * f1 * r1 * l1 * f1 * r1)
    );
}
