use crate::cube;
use cube::{Move, Sym16, MOVE_COUNT, SYM16_COUNT};
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
pub struct Flip(pub u16); // Edge Permutation Coordinate

#[derive(Debug, Copy, Clone)]
pub struct FlipIterator(u16);
pub const FLIP_COUNT: usize = 2048; // 2**11

impl Flip {
    pub const fn iter() -> FlipIterator {
        FlipIterator(0)
    }
}

impl From<cube::CubieLevel> for Flip {
    fn from(cl: cube::CubieLevel) -> Flip {
        let res = cl.1.iter().take(11).fold(0, |s, e| s * 2 + e.o as u16);
        Flip(res)
    }
}
impl From<Flip> for cube::CubieLevel {
    // return a representation
    fn from(fl: Flip) -> cube::CubieLevel {
        let mut res = cube::SOLVED;
        let (sum, _) = (0..11).into_iter().rev().fold((0, fl.0), |(sum, s), i| {
            res.1[i].o = (s % 2) as u8;
            (sum + (s % 2) as u8, s / 2)
        });
        res.1[11].o = (200 - sum) % 2;
        res
    }
}

#[test]
fn flip() {
    use super::*;
    for m in Move::iter() {
        let cube = m * cube::SOLVED;
        let fl: Flip = cube.into();
        assert_eq!(
            cube.1.iter().map(|e| e.o).collect::<Vec<_>>(),
            cube::CubieLevel::from(fl)
                .1
                .iter()
                .map(|e| e.o)
                .collect::<Vec<_>>()
        );
    }
}
impl Mul<Flip> for Move {
    type Output = Flip;
    fn mul(self, rhs: Flip) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<Flip> = {
                let mut memo = vec![Flip(!0); FLIP_COUNT * MOVE_COUNT];
                for fl in Flip::iter() {
                    let cube: cube::CubieLevel = fl.into();
                    for m in Move::iter() {
                        let v: Flip = (m * cube).into();
                        memo[fl.0 as usize * MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * MOVE_COUNT + self as usize]
    }
}

impl std::iter::Iterator for FlipIterator {
    type Item = Flip;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < FLIP_COUNT {
            return Some(Flip(i));
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (
            FLIP_COUNT - self.0 as usize,
            Some(FLIP_COUNT - self.0 as usize),
        )
    }
}
impl std::iter::FusedIterator for FlipIterator {}
impl std::iter::ExactSizeIterator for FlipIterator {}
