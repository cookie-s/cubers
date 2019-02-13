use super::cube;

pub mod phase1;
pub mod phase2;

pub trait Phase {
    type Error;

    fn solve(&self, cube: &cube::RubikCube) -> Result<Vec<cube::Move>, Self::Error>;
}
