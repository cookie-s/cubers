use crate::cube;
use cube::{Move, Sym16, MOVE_COUNT, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

const MAGIC: [[u16; 5]; 12] = [
    [1, 0, 0, 0, 0],
    [1, 1, 0, 0, 0],
    [1, 2, 1, 0, 0],
    [1, 3, 3, 1, 0],
    [1, 4, 6, 4, 1],
    [1, 5, 10, 10, 5],
    [1, 6, 15, 20, 15],
    [1, 7, 21, 35, 35],
    [1, 8, 28, 56, 70],
    [1, 9, 36, 84, 126],
    [1, 10, 45, 120, 210],
    [1, 11, 55, 165, 330],
];

#[derive(Debug, Copy, Clone)]
pub struct UDSliceIterator(u16);
pub const UDSLICE_COUNT: usize = 495; // 12 C 4
impl UDSlice {
    pub const fn iter() -> UDSliceIterator {
        UDSliceIterator(0)
    }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct UDSlice(pub u16); // phase2 UDSlice Coordinate
impl From<cube::CubieLevel> for UDSlice {
    // https://math.stackexchange.com/questions/1363239/fast-way-to-get-a-position-of-combination-without-repetitions
    fn from(cl: cube::CubieLevel) -> UDSlice {
        let mut array: Vec<_> = cl.1.iter().skip(8).map(|e| e.e as u16).collect();
        array.sort();

        let mut res = 0;
        let mut j = 0;
        for i in 0..12 {
            if j >= 4 {
                break;
            }
            if array[j] == i as u16 {
                j += 1;
                res += MAGIC[i][j];
            }
        }

        UDSlice(res)
    }
}
impl From<UDSlice> for cube::CubieLevel {
    // return a representation
    fn from(uds: UDSlice) -> cube::CubieLevel {
        let mut ary = [0; 4];
        let mut uds = uds.0;

        let mut j = 4;
        for i in (0..12).rev() {
            if uds >= MAGIC[i][j] {
                uds -= MAGIC[i][j];
                j -= 1;
                ary[j] = i;
                if j == 0 {
                    break;
                }
            }
        }

        let mut res = cube::SOLVED;
        for i in 0..4 {
            res.1[i as usize + 8].e = cube::SOLVED.1[ary[i]].e;
        }
        res
    }
}

#[test]
fn udslice() {
    use super::*;
    for m in Move::iter() {
        let cube = m * cube::SOLVED;

        let mut uds1 = cube.1.iter().map(|e| e.e).skip(8).collect::<Vec<_>>();
        uds1.sort();

        let uds: UDSlice = cube.into();
        let mut uds2 = cube::CubieLevel::from(uds)
            .1
            .iter()
            .map(|e| e.e)
            .skip(8)
            .collect::<Vec<_>>();
        uds2.sort();

        assert_eq!(uds1, uds2, "{:?} {:?}", uds, m);
    }
}

impl Mul<UDSlice> for Move {
    type Output = UDSlice;
    fn mul(self, rhs: UDSlice) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<UDSlice> = {
                let mut memo = vec![UDSlice(!0); UDSLICE_COUNT * MOVE_COUNT];
                for uds in UDSlice::iter() {
                    let cube: cube::CubieLevel = uds.into();
                    for m in Move::iter() {
                        let v: UDSlice = (m * cube).into();
                        memo[uds.0 as usize * MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * MOVE_COUNT + self as usize]
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
