use cubers::cube;

fn main() {
    println!("Hello, world!");

    use cubers::cube::Move::*;
    let mut c = cube::SOLVED;
    let v: Vec<cube::Move> = vec![
        L2, U1, F2, D1, L2, F2, U1, B2, F2, U2, F2, U3, R3, F3, D1, L3, D2, U3, F1, L2, R1, D1,
    ];

    for m in v.iter() {
        c = *m * c;
    }

    println!("{:?}", cube::RubikCube(c),);
}
