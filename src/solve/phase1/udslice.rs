use crate::cube;
use crate::cube::{Move, MOVE_COUNT};
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct UDSlice(pub u16); // phase2 UDSlice Coordinate
use UDSlice as S;

pub const COUNT: usize = (12 * 11 * 10 * 9) / (4 * 3 * 2 * 1); // C(12, 4)

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

impl From<cube::CubieLevel> for S {
    // https://math.stackexchange.com/questions/1363239/fast-way-to-get-a-position-of-combination-without-repetitions
    fn from(cl: cube::CubieLevel) -> S {
        let mut array: Vec<_> =
            cl.1.iter()
                .enumerate()
                .filter(|(_, &e)| e.e as u16 >= 8)
                .map(|(i, _)| i as u16)
                .collect();
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
impl From<S> for cube::CubieLevel {
    // return a representation
    fn from(uds: S) -> cube::CubieLevel {
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
        // FIXME 違うと思う テストは通る
        for i in 0..4 {
            res.1.swap(ary[i], i + 8);
        }
        res
    }
}

#[test]
fn udslice() {
    for m1 in Move::iter() {
        for m2 in Move::iter() {
            let cube = m2 * (m1 * cube::SOLVED);
            let ep: UDSlice = cube.into();
            assert_eq!(
                {
                    let mut c = cube
                        .1
                        .iter()
                        .map(|e| e.e as u16)
                        .enumerate()
                        .filter(|(_, e)| *e >= 8)
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>();
                    c.sort();
                    c
                },
                {
                    let mut c = cube::CubieLevel::from(ep)
                        .1
                        .iter()
                        .map(|e| e.e as u16)
                        .enumerate()
                        .filter(|(_, e)| *e >= 8)
                        .map(|(i, _)| i)
                        .collect::<Vec<usize>>();
                    c.sort();
                    c
                },
            );
        }
    }
}

impl Mul<S> for Move {
    type Output = S;
    fn mul(self, rhs: S) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<S> = {
                let mut memo = vec![S(!0); COUNT * MOVE_COUNT];
                for uds in S::iter() {
                    let cube: cube::CubieLevel = uds.into();
                    for m in Move::iter() {
                        let v: S = (m * cube).into();
                        memo[uds.0 as usize * MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * MOVE_COUNT + self as usize]
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
