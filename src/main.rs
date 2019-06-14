use cubers::cube;
use cubers::solve::Phase;

fn main() {
    println!("Hello, world!");

    use cubers::cube::Move::*;
    let mut cl = cube::SOLVED;
    let v: Vec<cube::Move> = vec![
        L2, U1, F2, D1, L2, F2, U1, D2, F2, U2, F2, U3, D1, L2, U3, L2, D1, U3, D2, L2, R2, B2,
    ];
    let v: Vec<cube::Move> = v[..].to_vec();
    println!("{:?}", v);

    for m in v.iter() {
        cl = *m * cl;
    }

    let c = cube::RubikCube(cl);
    println!("{:?}", c);

    let p2 = cubers::solve::phase2::Phase2::new();
    let solve = p2.solve(&c);
    println!("{:?}", solve);

    if let Ok(moves) = solve {
        for m in moves.iter() {
            cl = *m * cl;
        }
        let c = cube::RubikCube(cl);
        println!("{:?}", c);
    }
}
