use crate::cube;
use cube::{Move, Sym16};
use std::ops::Mul;

#[derive(Debug, Copy, Clone, EnumCount, EnumIter, FromPrimitive)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum P2Move {
    U1, U2, U3,
    D1, D2, D3,
    F2,
    B2,
    L2,
    R2,
}

impl From<Move> for P2Move {
    fn from(m: Move) -> P2Move {
        match m {
            Move::U1 => P2Move::U1,
            Move::U2 => P2Move::U2,
            Move::U3 => P2Move::U3,
            Move::D1 => P2Move::D1,
            Move::D2 => P2Move::D2,
            Move::D3 => P2Move::D3,
            Move::F2 => P2Move::F2,
            Move::B2 => P2Move::B2,
            Move::L2 => P2Move::L2,
            Move::R2 => P2Move::R2,
            _ => panic!("invalid argument"),
        }
    }
}
impl From<P2Move> for Move {
    fn from(m: P2Move) -> Move {
        match m {
            P2Move::U1 => Move::U1,
            P2Move::U2 => Move::U2,
            P2Move::U3 => Move::U3,
            P2Move::D1 => Move::D1,
            P2Move::D2 => Move::D2,
            P2Move::D3 => Move::D3,
            P2Move::F2 => Move::F2,
            P2Move::B2 => Move::B2,
            P2Move::L2 => Move::L2,
            P2Move::R2 => Move::R2,
        }
    }
}

impl std::ops::Mul<cube::CubieLevel> for P2Move {
    type Output = cube::CubieLevel;
    fn mul(self, rhs: cube::CubieLevel) -> Self::Output {
        let m: Move = self.into();
        m * rhs
    }
}

impl Mul<P2Move> for Sym16 {
    type Output = P2Move;
    fn mul(self, rhs: P2Move) -> Self::Output {
        (self * Move::from(rhs)).unwrap().into()
    }
}
