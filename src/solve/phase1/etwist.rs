use crate::cube;
use crate::cube::{Move, MOVE_COUNT};
use cube::{Sym16, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct ETwist(pub u16);
use ETwist as S;

pub const COUNT: usize = 2 * 2 * 2 * 2 * 2 * 2 * 2 * 2 * 2 * 2 * 2;

impl From<cube::CubieLevel> for S {
    fn from(cl: cube::CubieLevel) -> Self {
        let res = cl.1.iter().take(11).fold(0, |s, c| s * 2 + c.o as u16);
        S(res)
    }
}
impl From<S> for cube::CubieLevel {
    // return a representation
    fn from(tw: S) -> Self {
        let mut res = cube::SOLVED;
        let (sum, _) = (0..11).into_iter().rev().fold((0, tw.0), |(sum, s), i| {
            res.1[i].o = (s % 2) as u8;
            (sum + (s % 2) as u8, s / 2)
        });
        res.1[11].o = (30 - sum) % 2;
        res
    }
}

#[test]
fn etwi() {
    for m in Move::iter() {
        let cube = m * cube::SOLVED;
        let ep: S = cube.into();
        assert_eq!(
            cube.1.iter().map(|e| e.o).collect::<Vec<u8>>(),
            cube::CubieLevel::from(ep)
                .1
                .iter()
                .map(|e| e.o)
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
                for ep in S::iter() {
                    let cube: cube::CubieLevel = ep.into();
                    for m in Move::iter() {
                        let v: S = (m * cube).into();
                        memo[ep.0 as usize * MOVE_COUNT + (m as usize)] = v;
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
