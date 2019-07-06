use crate::cube;
use cube::{Move, Sym16, MOVE_COUNT, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct Twist(pub u16); // Corner Permutation Coordinate

#[derive(Debug, Copy, Clone)]
pub struct TwistIterator(u16);
pub const TWIST_COUNT: usize = 3 * 3 * 3 * 3 * 3 * 3 * 3;

impl Twist {
    pub const fn iter() -> TwistIterator {
        TwistIterator(0)
    }
}

impl From<cube::CubieLevel> for Twist {
    fn from(cl: cube::CubieLevel) -> Self {
        let res = cl.0.iter().take(7).fold(0, |s, c| s * 3 + c.o as u16);
        Twist(res)
    }
}
impl From<Twist> for cube::CubieLevel {
    // return a representation
    fn from(tw: Twist) -> Self {
        let mut res = cube::SOLVED;
        let (sum, _) = (0..7).into_iter().rev().fold((0, tw.0), |(sum, s), i| {
            res.0[i].o = (s % 3) as u8;
            (sum + (s % 3) as u8, s / 3)
        });
        res.0[7].o = (30 - sum) % 3;
        res
    }
}

#[test]
fn twist() {
    use super::*;
    for m in Move::iter() {
        let cube = m * cube::SOLVED;
        let tw: Twist = cube.into();
        assert_eq!(
            cube.0.iter().map(|c| c.o).collect::<Vec<_>>(),
            cube::CubieLevel::from(tw)
                .0
                .iter()
                .map(|c| c.o)
                .collect::<Vec<_>>()
        );
    }
}

impl Mul<Twist> for Move {
    type Output = Twist;
    fn mul(self, rhs: Twist) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<Twist> = {
                let mut memo = vec![Twist(!0); TWIST_COUNT * MOVE_COUNT];
                for cp in Twist::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for m in Move::iter() {
                        let v: Twist = (m * cube).into();
                        memo[cp.0 as usize * MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * MOVE_COUNT + self as usize]
    }
}
impl Mul<Twist> for Sym16 {
    type Output = Twist;
    fn mul(self, rhs: Twist) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<Twist> = {
                let mut memo = vec![Twist(!0); TWIST_COUNT * SYM16_COUNT];
                for cp in Twist::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for s in Sym16::iter() {
                        let v: Twist = (s * cube).into();
                        memo[cp.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}

impl std::iter::Iterator for TwistIterator {
    type Item = Twist;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < TWIST_COUNT {
            return Some(Twist(i));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            TWIST_COUNT - self.0 as usize,
            Some(TWIST_COUNT - self.0 as usize),
        )
    }
}
impl std::iter::FusedIterator for TwistIterator {}
impl std::iter::ExactSizeIterator for TwistIterator {}
