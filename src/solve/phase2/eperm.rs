use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct EPerm(pub u16); // Edge Permutation Coordinate
use EPerm as S;

pub const COUNT: usize = FACT8;

impl From<cube::CubieLevel> for S {
    fn from(cl: cube::CubieLevel) -> S {
        use crate::solve::util::FisherShuffle8;
        let shuffle = FisherShuffle8::new();

        let array: Vec<_> = cl.1.iter().map(|e| e.e as u16).take(8).collect();
        let res = shuffle.array_to_num(&array);
        S(res as u16)
    }
}
impl From<S> for cube::CubieLevel {
    // return a representation
    fn from(ep: S) -> cube::CubieLevel {
        use crate::solve::util::FisherShuffle8;
        let shuffle = FisherShuffle8::new();

        let array = shuffle.num_to_array(ep.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..8 {
            res.1[i].e = cube::SOLVED.1[array[i]].e;
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

impl Mul<S> for P2Move {
    type Output = S;
    fn mul(self, rhs: S) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<S> = {
                let mut memo = vec![S(!0); COUNT * P2MOVE_COUNT];
                for ep in S::iter() {
                    let cube: cube::CubieLevel = ep.into();
                    for m in P2Move::iter() {
                        let v: S = (m * cube).into();
                        memo[ep.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
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
