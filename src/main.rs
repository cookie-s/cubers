use cubers::cube;
use cubers::solve;

fn main() {
    println!("Hello, world!");

    use cubers::cube::Move::*;
    let mut cl = cube::SOLVED;
    let v: Vec<cube::Move> = vec![
        D1, L2, U3, L2, D3, B2, L2, U1, F2, D1, L2, F2, U1, D2, U2, F2, U3, D1, L2, U3, L2, D1, U3,
        D2, L2, R2, B2,
    ];
    let v: Vec<cube::Move> = v[..].to_vec();
    println!("{:?}", v);

    for m in v.iter() {
        cl = *m * cl;
    }

    let c = cubers::RubikCube(cl);
    println!("{:?}", c);

    use solve::solver::Solver;
    let p2 = std::fs::File::open("phase2.db")
        .map(|file| solve::solver::Phase2Solver::new_from_cache(file).unwrap())
        .unwrap_or_else(|_| {
            let res = solve::solver::Phase2Solver::new();
            let file = std::io::BufWriter::new(std::fs::File::create("phase2.db").unwrap());
            let _ = bincode::serialize_into(file, &res);
            res
        });

    let solve = p2.solve(c.0);
    println!("{:?}", solve);

    if let Ok(moves) = solve {
        for m in moves {
            cl = m * cl;
        }
        let c = cubers::RubikCube(cl);
        println!("{:?}", c);
    }
}
