use cubers::cube;

fn main() {
    println!("Hello, world!");
    println!("{:?}", cube::RubikCube::new(cube::Move::F1));
}
