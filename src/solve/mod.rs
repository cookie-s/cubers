use super::cube;

// pub mod phase1;
pub mod phase2;
pub mod util;

pub trait Phase {
    type Error;

    fn solve(&self, cube: &crate::RubikCube) -> Result<Vec<cube::Move>, Self::Error>;
}
