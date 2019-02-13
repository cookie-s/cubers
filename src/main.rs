use cubers::cube;
use cubers::solve::Phase;

fn main() {
    println!("Hello, world!");

    use cubers::cube::Move::*;
    let mut c = cube::SOLVED;
    let v: Vec<cube::Move> = vec![
        L2, U1, F2, D1, L2, F2, U1, B2, F2, U2, F2, U3, D1, D2, U3, L2, D1,
    ];

    for m in v.iter() {
        c = *m * c;
    }

    let c = cube::RubikCube(c);
    println!("{:?}", c);

    let p2 = cubers::solve::phase2::Phase2::new();
    println!("{:?}", p2.solve(&c));
}
