use super::cperm::{CPerm, COUNT as CPERM_COUNT};
use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::Sym16;
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct CPermCoset(pub u16);

#[derive(Debug, Copy, Clone)]
pub struct Iter(u16);
pub const CPERMCOSET_COUNT: usize = 2768;

impl From<CPerm> for CPermCoset {
    fn from(src: CPerm) -> CPermCoset {
        lazy_static! {
            static ref MEMO: Vec<CPermCoset> = {
                let mut memo = vec![CPermCoset(!0); CPERM_COUNT];
                let mut cnt = 0;

                for (i, cp) in CPerm::iter().enumerate() {
                    let cube: cube::CubieLevel = cp.into();
                    memo[i] = Sym16::iter()
                        .find_map(|s| {
                            let cube = s * cube;
                            let v: CPerm = cube.into();

                            if memo[v.0 as usize] != CPermCoset(!0) {
                                return Some(memo[v.0 as usize]);
                            }
                            None
                        })
                        .unwrap_or_else(|| {
                            cnt += 1;
                            CPermCoset((cnt - 1) as u16)
                        });
                }
                assert_eq!(cnt, CPERMCOSET_COUNT);

                memo
            };
        }
        MEMO[src.0 as usize]
    }
}
impl From<CPermCoset> for CPerm {
    // return a representation
    fn from(src: CPermCoset) -> CPerm {
        lazy_static! {
            static ref MEMO: Vec<CPerm> = {
                let mut memo = vec![CPerm(!0); CPERMCOSET_COUNT];

                for cp in CPerm::iter() {
                    let idx = CPermCoset::from(cp).0 as usize;
                    if memo[idx] == CPerm(!0) {
                        memo[idx] = cp;
                    }
                }
                memo
            };
        }
        MEMO[src.0 as usize]
    }
}

impl Mul<CPermCoset> for P2Move {
    type Output = CPermCoset;

    fn mul(self, rhs: CPermCoset) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<CPermCoset> = {
                let mut memo = vec![CPermCoset(!0); CPERMCOSET_COUNT * P2MOVE_COUNT];
                for (i, co) in CPermCoset::iter().enumerate() {
                    let cp: CPerm = co.into();

                    for m in P2Move::iter() {
                        let v: CPermCoset = (m * cp).into();
                        memo[i * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }

                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}

impl CPermCoset {
    pub const fn iter() -> Iter {
        Iter(0)
    }
}

impl std::iter::Iterator for Iter {
    type Item = CPermCoset;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < CPERMCOSET_COUNT {
            return Some(CPermCoset(i));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            CPERMCOSET_COUNT - self.0 as usize,
            Some(CPERMCOSET_COUNT - self.0 as usize),
        )
    }
}
impl std::iter::FusedIterator for Iter {}
impl std::iter::ExactSizeIterator for Iter {}
