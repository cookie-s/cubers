use super::cperm::*;
use super::p2move::{P2Move, P2MOVE_COUNT};
use crate::cube;
use cube::Sym16;
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct CPermCoset(pub u16);
pub const CPERMCOSET_COUNT: usize = 2768;

impl From<CPerm> for CPermCoset {
    fn from(src: CPerm) -> CPermCoset {
        lazy_static! {
            static ref MEMO: Vec<CPermCoset> = {
                let mut memo = vec![CPermCoset(!0); CPERM_COUNT];
                let mut cnt = 0;

                for i in 0..CPERM_COUNT {
                    let cp = CPerm(i as u16);
                    let cube: cube::CubieLevel = cp.into();
                    let mut found = None;

                    for s in Sym16::iter() {
                        let cube = s * cube;
                        let v: CPerm = cube.into();

                        if memo[v.0 as usize] != CPermCoset(!0) {
                            found = Some(memo[v.0 as usize]);
                            break;
                        }
                    }
                    if found == None {
                        found = Some(CPermCoset(cnt as u16));
                        cnt += 1;
                    }
                    memo[i] = found.unwrap()
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

                for i in 0..CPERM_COUNT {
                    let cp = CPerm(i as u16);
                    if memo[CPermCoset::from(cp).0 as usize] == CPerm(!0) {
                        memo[CPermCoset::from(cp).0 as usize] = cp;
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
                for i in 0..CPERMCOSET_COUNT {
                    let cp: CPerm = CPermCoset(i as u16).into();

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
