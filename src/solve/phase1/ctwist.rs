use crate::cube;
use crate::cube::{Move, MOVE_COUNT};
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct CTwist(pub u16);
use CTwist as S;

pub const COUNT: usize = 3 * 3 * 3 * 3 * 3 * 3 * 3; // 3 ** 7

impl From<cube::CubieLevel> for S {
    fn from(cl: cube::CubieLevel) -> Self {
        let res = cl.0.iter().take(7).fold(0, |s, c| s * 3 + c.o as u16);
        S(res)
    }
}
impl From<S> for cube::CubieLevel {
    // return a representation
    fn from(tw: S) -> Self {
        let mut res = cube::SOLVED;
        let (sum, _) = (0..7).rev().fold((0, tw.0), |(sum, s), i| {
            res.0[i].o = (s % 3) as u8;
            (sum + (s % 3) as u8, s / 3)
        });
        res.0[7].o = (30 - sum) % 3;
        res
    }
}

#[test]
fn ctwi() {
    for m in Move::iter() {
        let cube = m * cube::SOLVED;
        let cp: S = cube.into();
        assert_eq!(
            cube.0.iter().map(|c| c.o).collect::<Vec<u8>>(),
            cube::CubieLevel::from(cp)
                .0
                .iter()
                .map(|c| c.o)
                .collect::<Vec<u8>>()
        );
    }
}

impl Mul<S> for Move {
    type Output = S;
    fn mul(self, rhs: S) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<S> = {
                let mut memo = vec![S(!0); COUNT * MOVE_COUNT];
                for cp in S::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for m in Move::iter() {
                        let v: S = (m * cube).into();
                        memo[cp.0 as usize * MOVE_COUNT + (m as usize)] = v;
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
