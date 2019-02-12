use super::cube;

pub mod phase1;
pub mod phase2;

trait Phase {
    fn solve(&self, cube: &cube::RubikCube) -> Vec<cube::Move>;
}
