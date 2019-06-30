use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

const FACT4: usize = 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone)]
pub struct UDSliceIterator(u8);
pub const UDSLICE_COUNT: usize = FACT4;
impl UDSlice {
    pub const fn iter() -> UDSliceIterator {
        UDSliceIterator(0)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UDSlice(pub u8); // phase2 UDSlice Coordinate
impl From<cube::CubieLevel> for UDSlice {
    fn from(cl: cube::CubieLevel) -> UDSlice {
        use crate::solve::util::FisherShuffle;
        let shuffle = FisherShuffle::new(4);

        let array: Vec<_> = cl.1.iter().skip(8).map(|e| e.e as u16 - 8).collect();
        let res = shuffle.array_to_num(&array);
        UDSlice(res as u8)
    }
}
impl From<UDSlice> for cube::CubieLevel {
    // return a representation
    fn from(uds: UDSlice) -> cube::CubieLevel {
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

impl Mul<UDSlice> for P2Move {
    type Output = UDSlice;
    fn mul(self, rhs: UDSlice) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<UDSlice> = {
                let mut memo = vec![UDSlice(!0); UDSLICE_COUNT * P2MOVE_COUNT];
                for uds in UDSlice::iter() {
                    let cube: cube::CubieLevel = uds.into();
                    for m in P2Move::iter() {
                        let v: UDSlice = (m * cube).into();
                        memo[uds.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}

impl Mul<UDSlice> for Sym16 {
    type Output = UDSlice;
    fn mul(self, rhs: UDSlice) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<UDSlice> = {
                let mut memo = vec![UDSlice(!0); UDSLICE_COUNT * SYM16_COUNT];
                for uds in UDSlice::iter() {
                    let cube: cube::CubieLevel = uds.into();
                    for s in Sym16::iter() {
                        let v: UDSlice = (s * cube).into();
                        memo[uds.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}

impl std::iter::Iterator for UDSliceIterator {
    type Item = UDSlice;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < UDSLICE_COUNT {
            return Some(UDSlice(i));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            UDSLICE_COUNT - self.0 as usize,
            Some(UDSLICE_COUNT - self.0 as usize),
        )
    }
}
impl std::iter::FusedIterator for UDSliceIterator {}
impl std::iter::ExactSizeIterator for UDSliceIterator {}
