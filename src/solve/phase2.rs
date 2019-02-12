use super::cube;

pub struct Phase2();

impl Phase2 {
    fn new() -> Self {
        Phase2()
    }
}

impl super::Phase for Phase2 {
    fn solve(&self, src: &cube::RubikCube) -> Vec<cube::Move> {
        vec![]
    }
}
