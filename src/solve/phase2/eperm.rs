use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct EPerm(pub u16); // Edge Permutation Coordinate

#[derive(Debug, Copy, Clone)]
pub struct EPermIterator(u16);
pub const EPERM_COUNT: usize = FACT8;

impl EPerm {
    pub fn iter() -> EPermIterator {
        EPermIterator(0)
    }
}

impl From<cube::CubieLevel> for EPerm {
    fn from(cl: cube::CubieLevel) -> EPerm {
        use crate::solve::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array: Vec<_> = cl.1.iter().map(|e| e.e as u16).take(8).collect();
        let res = shuffle.array_to_num(&array);
        EPerm(res as u16)
    }
}
impl From<EPerm> for cube::CubieLevel {
    // return a representation
    fn from(ep: EPerm) -> cube::CubieLevel {
        use crate::solve::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array = shuffle.num_to_array(ep.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..8 {
            res.1[i as usize].e = cube::SOLVED.1[array[i]].e;
        }
        res
    }
}

#[test]
fn eperm() {
    use super::*;
    for m in P2Move::iter() {
        let cube = m * cube::SOLVED;
        let ep: EPerm = cube.into();
        assert_eq!(
            cube.1
                .iter()
                .map(|e| e.e)
                .take(8)
                .collect::<Vec<cube::EdgeCubePos>>(),
            cube::CubieLevel::from(ep)
                .1
                .iter()
                .map(|e| e.e)
                .take(8)
                .collect::<Vec<cube::EdgeCubePos>>()
        );
    }
}
impl Mul<EPerm> for P2Move {
    type Output = EPerm;
    fn mul(self, rhs: EPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<EPerm> = {
                let mut memo = vec![EPerm(!0); EPERM_COUNT * P2MOVE_COUNT];
                for ep in EPerm::iter() {
                    let cube: cube::CubieLevel = ep.into();
                    for m in P2Move::iter() {
                        let v: EPerm = (m * cube).into();
                        memo[ep.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}
impl Mul<EPerm> for Sym16 {
    type Output = EPerm;
    fn mul(self, rhs: EPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<EPerm> = {
                let mut memo = vec![EPerm(!0); EPERM_COUNT * SYM16_COUNT];
                for ep in EPerm::iter() {
                    let cube: cube::CubieLevel = ep.into();
                    for s in Sym16::iter() {
                        let v: EPerm = (s * cube).into();
                        memo[ep.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}

impl std::iter::Iterator for EPermIterator {
    type Item = EPerm;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < EPERM_COUNT {
            return Some(EPerm(i));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            EPERM_COUNT - self.0 as usize,
            Some(EPERM_COUNT - self.0 as usize),
        )
    }
}
impl std::iter::FusedIterator for EPermIterator {}
impl std::iter::ExactSizeIterator for EPermIterator {}
