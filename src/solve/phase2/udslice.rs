use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

const FACT4: usize = 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct UDSlice(pub u8); // phase2 UDSlice Coordinate
use UDSlice as S;

pub const COUNT: usize = FACT4;

impl From<cube::CubieLevel> for S {
    fn from(cl: cube::CubieLevel) -> Self {
        use crate::solve::util::FisherShuffle;
        let shuffle = FisherShuffle::new(4);

        let array: Vec<_> = cl.1.iter().skip(8).map(|e| e.e as u16 - 8).collect();
        let res = shuffle.array_to_num(&array);
        S(res as u8)
    }
}
impl From<S> for cube::CubieLevel {
    // return a representation
    fn from(uds: S) -> Self {
        use crate::solve::util::FisherShuffle;
        let shuffle = FisherShuffle::new(4);

        let array = shuffle.num_to_array(uds.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..4 {
            res.1[i as usize + 8].e = cube::SOLVED.1[array[i] + 8].e;
        }
        res
    }
}

#[test]
fn udslice() {
    use super::*;
    for m in P2Move::iter() {
        let cube = m * cube::SOLVED;
        let ep: UDSlice = cube.into();
        assert_eq!(
            cube.1
                .iter()
                .map(|e| e.e)
                .skip(8)
                .collect::<Vec<cube::EdgeCubePos>>(),
            cube::CubieLevel::from(ep)
                .1
                .iter()
                .map(|e| e.e)
                .skip(8)
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
                for uds in S::iter() {
                    let cube: cube::CubieLevel = uds.into();
                    for m in P2Move::iter() {
                        let v: S = (m * cube).into();
                        memo[uds.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
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
pub struct Iter(u8);
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
