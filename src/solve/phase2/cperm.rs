use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct CPerm(pub u16); // Corner Permutation Coordinate

#[derive(Debug, Copy, Clone)]
pub struct CPermIterator(u16);
pub const CPERM_COUNT: usize = FACT8;

impl CPerm {
    pub fn iter() -> CPermIterator {
        CPermIterator(0)
    }
}

impl From<cube::CubieLevel> for CPerm {
    fn from(cl: cube::CubieLevel) -> CPerm {
        use crate::solve::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array: Vec<_> = cl.0.iter().map(|c| c.c as u16).collect();
        let res = shuffle.array_to_num(&array);
        CPerm(res as u16)
    }
}
impl From<CPerm> for cube::CubieLevel {
    // return a representation
    fn from(cp: CPerm) -> cube::CubieLevel {
        use crate::solve::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array = shuffle.num_to_array(cp.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..8 {
            res.0[i as usize].c = cube::SOLVED.0[array[i]].c;
        }
        return res;
    }
}

#[test]
fn cperm() {
    use super::*;
    for m in P2Move::iter() {
        let cube = m * cube::SOLVED;
        let cp: CPerm = cube.into();
        assert_eq!(
            cube.0
                .iter()
                .map(|c| c.c)
                .collect::<Vec<cube::CornerCubePos>>(),
            cube::CubieLevel::from(cp)
                .0
                .iter()
                .map(|c| c.c)
                .collect::<Vec<cube::CornerCubePos>>()
        );
    }
}

impl Mul<CPerm> for P2Move {
    type Output = CPerm;
    fn mul(self, rhs: CPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<CPerm> = {
                let mut memo = vec![CPerm(!0); CPERM_COUNT * P2MOVE_COUNT];
                for cp in CPerm::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for m in P2Move::iter() {
                        let v: CPerm = (m * cube).into();
                        memo[cp.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}
impl Mul<CPerm> for Sym16 {
    type Output = CPerm;
    fn mul(self, rhs: CPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<CPerm> = {
                let mut memo = vec![CPerm(!0); CPERM_COUNT * SYM16_COUNT];
                for cp in CPerm::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for s in Sym16::iter() {
                        let v: CPerm = (s * cube).into();
                        memo[cp.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}

impl std::iter::Iterator for CPermIterator {
    type Item = CPerm;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < CPERM_COUNT {
            return Some(CPerm(i));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            CPERM_COUNT - self.0 as usize,
            Some(CPERM_COUNT - self.0 as usize),
        )
    }
}
impl std::iter::FusedIterator for CPermIterator {}
impl std::iter::ExactSizeIterator for CPermIterator {}
