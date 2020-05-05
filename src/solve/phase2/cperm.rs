use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct CPerm(pub u16); // Corner Permutation Coordinate
use CPerm as S;

pub const COUNT: usize = FACT8;

impl From<cube::CubieLevel> for S {
    fn from(cl: cube::CubieLevel) -> S {
        use crate::solve::util::FisherShuffle8;
        let shuffle = FisherShuffle8::new();

        let array: Vec<_> = cl.0.iter().map(|c| c.c as u16).collect();
        let res = shuffle.array_to_num(&array);
        S(res as u16)
    }
}
impl From<S> for cube::CubieLevel {
    // return a representation
    fn from(cp: S) -> cube::CubieLevel {
        use crate::solve::util::FisherShuffle8;
        let shuffle = FisherShuffle8::new();

        let array = shuffle.num_to_array(cp.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..8 {
            res.0[i as usize].c = cube::SOLVED.0[array[i]].c;
        }
        res
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

impl Mul<S> for P2Move {
    type Output = S;
    fn mul(self, rhs: S) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<S> = {
                let mut memo = vec![S(!0); COUNT * P2MOVE_COUNT];
                for cp in S::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for m in P2Move::iter() {
                        let v: S = (m * cube).into();
                        memo[cp.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}
impl Mul<S> for Sym16 {
    type Output = S;
    fn mul(self, rhs: S) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<S> = {
                let mut memo = vec![S(!0); COUNT * SYM16_COUNT];
                for x in S::iter() {
                    let cube: cube::CubieLevel = x.into();
                    for s in Sym16::iter() {
                        let v: S = (s * cube).into();
                        memo[x.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Iter(u16);
impl S {
    pub const fn iter() -> Iter {
        Iter(0)
    }
}
impl std::iter::Iterator for Iter {
    type Item = S;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < COUNT {
            return Some(S(i));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (COUNT - self.0 as usize, Some(COUNT - self.0 as usize))
    }
}
impl std::iter::FusedIterator for Iter {}
impl std::iter::ExactSizeIterator for Iter {}
