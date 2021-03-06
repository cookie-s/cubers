use num_traits::cast::FromPrimitive;
use std::ops::Mul;
use strum::IntoEnumIterator;

use CornerCube as CC;
use EdgeCube as EC;

#[rustfmt::skip]
pub const SOLVED: CubieLevel = CubieLevel(
    [
        CC { c: CornerCubePos::URF, o: 0 },
        CC { c: CornerCubePos::UFL, o: 0 },
        CC { c: CornerCubePos::ULB, o: 0 },
        CC { c: CornerCubePos::UBR, o: 0 },
        CC { c: CornerCubePos::DFR, o: 0 },
        CC { c: CornerCubePos::DLF, o: 0 },
        CC { c: CornerCubePos::DBL, o: 0 },
        CC { c: CornerCubePos::DRB, o: 0 },
    ],
    [
        EC { e: EdgeCubePos::UR, o: 0 },
        EC { e: EdgeCubePos::UF, o: 0 },
        EC { e: EdgeCubePos::UL, o: 0 },
        EC { e: EdgeCubePos::UB, o: 0 },
        EC { e: EdgeCubePos::DR, o: 0 },
        EC { e: EdgeCubePos::DF, o: 0 },
        EC { e: EdgeCubePos::DL, o: 0 },
        EC { e: EdgeCubePos::DB, o: 0 },
        EC { e: EdgeCubePos::FR, o: 0 },
        EC { e: EdgeCubePos::FL, o: 0 },
        EC { e: EdgeCubePos::BL, o: 0 },
        EC { e: EdgeCubePos::BR, o: 0 },
    ],
);

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CornerCubePos {
    URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
}

#[rustfmt::skip]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum EdgeCubePos {
    UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CornerCube {
    pub c: CornerCubePos,
    pub o: u8, // [0, 3)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct EdgeCube {
    pub e: EdgeCubePos,
    pub o: u8, // [0, 2)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumCount, EnumIter)]
#[rustfmt::skip]
pub enum Move {
    U1, U2, U3,
    D1, D2, D3,
    F1, F2, F3,
    B1, B2, B3,
    L1, L2, L3,
    R1, R2, R3,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CubieLevel(pub [CornerCube; 8], pub [EdgeCube; 12]);

impl<'a> Mul<&'a CubieLevel> for &'a CubieLevel {
    type Output = CubieLevel;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Self) -> Self::Output {
        let lhs = self;

        let mut res = SOLVED;

        for i in 0..8 {
            res.0[i] = rhs.0[lhs.0[i].c as usize];
            res.0[i].o += lhs.0[i].o;
            res.0[i].o %= 3;
        }

        for i in 0..12 {
            res.1[i] = rhs.1[lhs.1[i].e as usize];
            res.1[i].o += lhs.1[i].o;
            res.1[i].o %= 2;
        }

        res
    }
}

impl Mul<CubieLevel> for Move {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        #[allow(clippy::suspicious_arithmetic_impl)]
        fn subst(m: Move) -> &'static CubieLevel {
            use self::CornerCubePos::*;
            use self::EdgeCubePos::*;
            use self::Move::*;

            const U1_CUBE: CubieLevel = CubieLevel(
                [
                    CC { c: UBR, o: 0 },
                    CC { c: URF, o: 0 },
                    CC { c: UFL, o: 0 },
                    CC { c: ULB, o: 0 },
                    CC { c: DFR, o: 0 },
                    CC { c: DLF, o: 0 },
                    CC { c: DBL, o: 0 },
                    CC { c: DRB, o: 0 },
                ],
                [
                    EC { e: UB, o: 0 },
                    EC { e: UR, o: 0 },
                    EC { e: UF, o: 0 },
                    EC { e: UL, o: 0 },
                    EC { e: DR, o: 0 },
                    EC { e: DF, o: 0 },
                    EC { e: DL, o: 0 },
                    EC { e: DB, o: 0 },
                    EC { e: FR, o: 0 },
                    EC { e: FL, o: 0 },
                    EC { e: BL, o: 0 },
                    EC { e: BR, o: 0 },
                ],
            );
            const R1_CUBE: CubieLevel = CubieLevel(
                [
                    CC { c: DFR, o: 2 },
                    CC { c: UFL, o: 0 },
                    CC { c: ULB, o: 0 },
                    CC { c: URF, o: 1 },
                    CC { c: DRB, o: 1 },
                    CC { c: DLF, o: 0 },
                    CC { c: DBL, o: 0 },
                    CC { c: UBR, o: 2 },
                ],
                [
                    EC { e: FR, o: 0 },
                    EC { e: UF, o: 0 },
                    EC { e: UL, o: 0 },
                    EC { e: UB, o: 0 },
                    EC { e: BR, o: 0 },
                    EC { e: DF, o: 0 },
                    EC { e: DL, o: 0 },
                    EC { e: DB, o: 0 },
                    EC { e: DR, o: 0 },
                    EC { e: FL, o: 0 },
                    EC { e: BL, o: 0 },
                    EC { e: UR, o: 0 },
                ],
            );

            const F1_CUBE: CubieLevel = CubieLevel(
                [
                    CC { c: UFL, o: 1 },
                    CC { c: DLF, o: 2 },
                    CC { c: ULB, o: 0 },
                    CC { c: UBR, o: 0 },
                    CC { c: URF, o: 2 },
                    CC { c: DFR, o: 1 },
                    CC { c: DBL, o: 0 },
                    CC { c: DRB, o: 0 },
                ],
                [
                    EC { e: UR, o: 0 },
                    EC { e: FL, o: 1 },
                    EC { e: UL, o: 0 },
                    EC { e: UB, o: 0 },
                    EC { e: DR, o: 0 },
                    EC { e: FR, o: 1 },
                    EC { e: DL, o: 0 },
                    EC { e: DB, o: 0 },
                    EC { e: UF, o: 1 },
                    EC { e: DF, o: 1 },
                    EC { e: BL, o: 0 },
                    EC { e: BR, o: 0 },
                ],
            );
            const D1_CUBE: CubieLevel = CubieLevel(
                [
                    CC { c: URF, o: 0 },
                    CC { c: UFL, o: 0 },
                    CC { c: ULB, o: 0 },
                    CC { c: UBR, o: 0 },
                    CC { c: DLF, o: 0 },
                    CC { c: DBL, o: 0 },
                    CC { c: DRB, o: 0 },
                    CC { c: DFR, o: 0 },
                ],
                [
                    EC { e: UR, o: 0 },
                    EC { e: UF, o: 0 },
                    EC { e: UL, o: 0 },
                    EC { e: UB, o: 0 },
                    EC { e: DF, o: 0 },
                    EC { e: DL, o: 0 },
                    EC { e: DB, o: 0 },
                    EC { e: DR, o: 0 },
                    EC { e: FR, o: 0 },
                    EC { e: FL, o: 0 },
                    EC { e: BL, o: 0 },
                    EC { e: BR, o: 0 },
                ],
            );
            const L1_CUBE: CubieLevel = CubieLevel(
                [
                    CC { c: URF, o: 0 },
                    CC { c: ULB, o: 1 },
                    CC { c: DBL, o: 2 },
                    CC { c: UBR, o: 0 },
                    CC { c: DFR, o: 0 },
                    CC { c: UFL, o: 2 },
                    CC { c: DLF, o: 1 },
                    CC { c: DRB, o: 0 },
                ],
                [
                    EC { e: UR, o: 0 },
                    EC { e: UF, o: 0 },
                    EC { e: BL, o: 0 },
                    EC { e: UB, o: 0 },
                    EC { e: DR, o: 0 },
                    EC { e: DF, o: 0 },
                    EC { e: FL, o: 0 },
                    EC { e: DB, o: 0 },
                    EC { e: FR, o: 0 },
                    EC { e: UL, o: 0 },
                    EC { e: DL, o: 0 },
                    EC { e: BR, o: 0 },
                ],
            );
            const B1_CUBE: CubieLevel = CubieLevel(
                [
                    CC { c: URF, o: 0 },
                    CC { c: UFL, o: 0 },
                    CC { c: UBR, o: 1 },
                    CC { c: DRB, o: 2 },
                    CC { c: DFR, o: 0 },
                    CC { c: DLF, o: 0 },
                    CC { c: ULB, o: 2 },
                    CC { c: DBL, o: 1 },
                ],
                [
                    EC { e: UR, o: 0 },
                    EC { e: UF, o: 0 },
                    EC { e: UL, o: 0 },
                    EC { e: BR, o: 1 },
                    EC { e: DR, o: 0 },
                    EC { e: DF, o: 0 },
                    EC { e: DL, o: 0 },
                    EC { e: BL, o: 1 },
                    EC { e: FR, o: 0 },
                    EC { e: FL, o: 0 },
                    EC { e: UB, o: 1 },
                    EC { e: DB, o: 1 },
                ],
            );

            lazy_static! {
                static ref U2_CUBE: CubieLevel = &U1_CUBE * &U1_CUBE;
                static ref U3_CUBE: CubieLevel = &U1_CUBE * &U2_CUBE;
                static ref R2_CUBE: CubieLevel = &R1_CUBE * &R1_CUBE;
                static ref R3_CUBE: CubieLevel = &R1_CUBE * &R2_CUBE;
                static ref F2_CUBE: CubieLevel = &F1_CUBE * &F1_CUBE;
                static ref F3_CUBE: CubieLevel = &F1_CUBE * &F2_CUBE;
                static ref D2_CUBE: CubieLevel = &D1_CUBE * &D1_CUBE;
                static ref D3_CUBE: CubieLevel = &D1_CUBE * &D2_CUBE;
                static ref L2_CUBE: CubieLevel = &L1_CUBE * &L1_CUBE;
                static ref L3_CUBE: CubieLevel = &L1_CUBE * &L2_CUBE;
                static ref B2_CUBE: CubieLevel = &B1_CUBE * &B1_CUBE;
                static ref B3_CUBE: CubieLevel = &B1_CUBE * &B2_CUBE;
            };

            match m {
                U1 => &U1_CUBE,
                U2 => &U2_CUBE,
                U3 => &U3_CUBE,

                R1 => &R1_CUBE,
                R2 => &R2_CUBE,
                R3 => &R3_CUBE,

                F1 => &F1_CUBE,
                F2 => &F2_CUBE,
                F3 => &F3_CUBE,

                D1 => &D1_CUBE,
                D2 => &D2_CUBE,
                D3 => &D3_CUBE,

                L1 => &L1_CUBE,
                L2 => &L2_CUBE,
                L3 => &L3_CUBE,

                B1 => &B1_CUBE,
                B2 => &B2_CUBE,
                B3 => &B3_CUBE,
            }
        }

        subst(self) * &rhs
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn jimei() {
        use super::Move::*;
        use super::*;
        use crate::RubikCube;
        for &(m1, m2, m3) in [
            (U1, U2, U3),
            (D1, D2, D3),
            (F1, F2, F3),
            (B1, B2, B3),
            (L1, L2, L3),
            (R1, R2, R3),
        ]
        .iter()
        {
            assert_eq!(RubikCube(m1 * (m1 * SOLVED)), RubikCube(m2 * SOLVED));
            assert_eq!(RubikCube(m2 * (m1 * SOLVED)), RubikCube(m3 * SOLVED));
            assert_eq!(RubikCube(m3 * (m3 * (m3 * SOLVED))), RubikCube(m1 * SOLVED));
            assert_eq!(RubikCube(m1 * (m2 * (m2 * SOLVED))), RubikCube(m1 * SOLVED));
            assert_eq!(RubikCube(m2 * (m1 * (m2 * SOLVED))), RubikCube(m1 * SOLVED));
            assert_eq!(RubikCube(m2 * (m2 * (m1 * SOLVED))), RubikCube(m1 * SOLVED));
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
#[rustfmt::skip]
enum SymF { F0, F1 }

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
#[rustfmt::skip]
enum SymU { U0, U1, U2, U3 }

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
#[rustfmt::skip]
enum SymLR { LR0, LR1 }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Sym16Vec(SymF, SymU, SymLR);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Sym16(pub u8);
pub const SYM16_COUNT: usize = 16;

impl std::fmt::Debug for Sym16 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Sym16Vec::fmt(&Sym16Vec::from(*self), f)
    }
}

impl From<SymF> for &'static CubieLevel {
    fn from(src: SymF) -> &'static CubieLevel {
        use CornerCubePos::*;
        use EdgeCubePos::*;

        const F1_CUBE: CubieLevel = CubieLevel(
            [
                CC { c: DLF, o: 0 },
                CC { c: DFR, o: 0 },
                CC { c: DRB, o: 0 },
                CC { c: DBL, o: 0 },
                CC { c: UFL, o: 0 },
                CC { c: URF, o: 0 },
                CC { c: UBR, o: 0 },
                CC { c: ULB, o: 0 },
            ],
            [
                EC { e: DL, o: 0 },
                EC { e: DF, o: 0 },
                EC { e: DR, o: 0 },
                EC { e: DB, o: 0 },
                EC { e: UL, o: 0 },
                EC { e: UF, o: 0 },
                EC { e: UR, o: 0 },
                EC { e: UB, o: 0 },
                EC { e: FL, o: 0 },
                EC { e: FR, o: 0 },
                EC { e: BR, o: 0 },
                EC { e: BL, o: 0 },
            ],
        );

        match src {
            SymF::F0 => &SOLVED,
            SymF::F1 => &F1_CUBE,
        }
    }
}
impl From<SymU> for &'static CubieLevel {
    fn from(src: SymU) -> &'static CubieLevel {
        use CornerCubePos::*;
        use EdgeCubePos::*;

        const U1_CUBE: CubieLevel = CubieLevel(
            [
                CC { c: UBR, o: 0 },
                CC { c: URF, o: 0 },
                CC { c: UFL, o: 0 },
                CC { c: ULB, o: 0 },
                CC { c: DRB, o: 0 },
                CC { c: DFR, o: 0 },
                CC { c: DLF, o: 0 },
                CC { c: DBL, o: 0 },
            ],
            [
                EC { e: UB, o: 1 },
                EC { e: UR, o: 1 },
                EC { e: UF, o: 1 },
                EC { e: UL, o: 1 },
                EC { e: DB, o: 1 },
                EC { e: DR, o: 1 },
                EC { e: DF, o: 1 },
                EC { e: DL, o: 1 },
                EC { e: BR, o: 1 },
                EC { e: FR, o: 1 },
                EC { e: FL, o: 1 },
                EC { e: BL, o: 1 },
            ],
        );
        lazy_static! {
            static ref U2_CUBE: CubieLevel = &U1_CUBE * &U1_CUBE;
            static ref U3_CUBE: CubieLevel = &U1_CUBE * &U2_CUBE;
        };

        match src {
            SymU::U0 => &SOLVED,
            SymU::U1 => &U1_CUBE,
            SymU::U2 => &U2_CUBE,
            SymU::U3 => &U3_CUBE,
        }
    }
}
impl From<SymLR> for &'static CubieLevel {
    fn from(src: SymLR) -> &'static CubieLevel {
        use CornerCubePos::*;
        use EdgeCubePos::*;

        const LR1_CUBE: CubieLevel = CubieLevel(
            [
                CC { c: UFL, o: 3 },
                CC { c: URF, o: 3 },
                CC { c: UBR, o: 3 },
                CC { c: ULB, o: 3 },
                CC { c: DLF, o: 3 },
                CC { c: DFR, o: 3 },
                CC { c: DRB, o: 3 },
                CC { c: DBL, o: 3 },
            ],
            [
                EC { e: UL, o: 0 },
                EC { e: UF, o: 0 },
                EC { e: UR, o: 0 },
                EC { e: UB, o: 0 },
                EC { e: DL, o: 0 },
                EC { e: DF, o: 0 },
                EC { e: DR, o: 0 },
                EC { e: DB, o: 0 },
                EC { e: FL, o: 0 },
                EC { e: FR, o: 0 },
                EC { e: BR, o: 0 },
                EC { e: BL, o: 0 },
            ],
        );

        match src {
            SymLR::LR0 => &SOLVED,
            SymLR::LR1 => &LR1_CUBE,
        }
    }
}

trait Inv {
    fn inv(self) -> Self;
}

impl Inv for SymF {
    fn inv(self) -> Self {
        use SymF::*;
        match self {
            F0 => F0,
            F1 => F1,
        }
    }
}
impl Mul<SymF> for SymF {
    type Output = SymF;

    fn mul(self, rhs: SymF) -> Self::Output {
        match self {
            Self::F0 => rhs,
            Self::F1 => match rhs {
                Self::F0 => Self::F1,
                Self::F1 => Self::F0,
            },
        }
    }
}
impl Mul<CubieLevel> for SymF {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        match self {
            Self::F0 => rhs, //
            _ => {
                let res = &rhs * <&CubieLevel>::from(self);
                <&CubieLevel>::from(self.inv()) * &res
            }
        }
    }
}

impl Inv for SymU {
    fn inv(self) -> Self {
        use SymU::*;
        match self {
            U0 => U0,
            U1 => U3,
            U2 => U2,
            U3 => U1,
        }
    }
}
impl Mul<SymU> for SymU {
    type Output = SymU;

    fn mul(self, rhs: SymU) -> Self::Output {
        Self::from_usize((self as usize + rhs as usize) % SYMU_COUNT).unwrap()
    }
}
impl Mul<CubieLevel> for SymU {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        match self {
            Self::U0 => rhs,
            _ => {
                let res = &rhs * <&CubieLevel>::from(self);
                <&CubieLevel>::from(self.inv()) * &res
            }
        }
    }
}

impl Inv for SymLR {
    fn inv(self) -> Self {
        use SymLR::*;
        match self {
            Self::LR0 => LR0,
            Self::LR1 => LR1,
        }
    }
}
impl Mul<SymLR> for SymLR {
    type Output = SymLR;

    fn mul(self, rhs: SymLR) -> Self::Output {
        match self {
            Self::LR0 => rhs,
            Self::LR1 => match rhs {
                Self::LR0 => Self::LR1,
                Self::LR1 => Self::LR0,
            },
        }
    }
}
impl Mul<CubieLevel> for SymLR {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        match self {
            Self::LR0 => rhs,
            _ => {
                let res = &rhs * <&CubieLevel>::from(self);
                <&CubieLevel>::from(self.inv()) * &res
            }
        }
    }
}

impl From<Sym16> for CubieLevel {
    fn from(src: Sym16) -> Self {
        let Sym16Vec(f, u, lr) = src.into();
        let res = <&CubieLevel>::from(u) * <&CubieLevel>::from(lr);
        <&CubieLevel>::from(f) * &res
    }
}

impl From<Sym16Vec> for Sym16 {
    fn from(src: Sym16Vec) -> Self {
        let Sym16Vec(f, u, lr) = src;
        let (f, u, lr) = (f as u8, u as u8, lr as u8);
        Sym16((f * SYMU_COUNT as u8 + u) * SYMLR_COUNT as u8 + lr)
    }
}
impl From<Sym16> for Sym16Vec {
    fn from(src: Sym16) -> Self {
        let src = src.0;

        let lr = src % SYMLR_COUNT as u8;
        let t = src / SYMLR_COUNT as u8;

        let u = t % SYMU_COUNT as u8;
        let f = t / SYMU_COUNT as u8;
        let (f, u, lr) = (
            SymF::from_u8(f).unwrap(),
            SymU::from_u8(u).unwrap(),
            SymLR::from_u8(lr).unwrap(),
        );
        Sym16Vec(f, u, lr)
    }
}

impl Inv for Sym16Vec {
    fn inv(self) -> Self {
        let Sym16Vec(f, u, lr) = self;
        Sym16Vec(f.inv(), u.inv(), lr.inv())
    }
}

impl Inv for Sym16 {
    fn inv(self) -> Self {
        lazy_static! {
            static ref MEMO: Vec<Sym16> = {
                let mut memo = vec![Sym16(!0); SYM16_COUNT];

                for (i, s) in Sym16::iter().enumerate() {
                    let s: CubieLevel = s.into();
                    memo[i] = Sym16::iter()
                        .find(|&t| {
                            let t: CubieLevel = t.into();
                            &t * &s == SOLVED
                        })
                        .unwrap();
                }
                memo
            };
        }
        MEMO[self.0 as usize]
    }
}

#[test]
fn inv_sym16() {
    for s in Sym16::iter() {
        assert_eq!(s * s.inv(), Sym16(0))
    }
}

#[test]
fn sym16() {
    // TODO These must be P2Move
    let cube = Move::U1 * (Move::F2 * (Move::U3 * (Move::D1 * SOLVED)));

    for s in Sym16::iter() {
        let c1 = s * cube;
        let c2 = s.inv() * c1;

        assert_eq!(crate::RubikCube(cube), crate::RubikCube(c2));
    }
}

impl Mul<Sym16Vec> for Sym16Vec {
    type Output = Sym16Vec;

    fn mul(self, rhs: Sym16Vec) -> Self::Output {
        (Sym16::from(self) * Sym16::from(rhs)).into()
    }
}

impl Mul<Sym16> for Sym16 {
    type Output = Sym16;

    fn mul(self, rhs: Sym16) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<Sym16> = {
                let mut memo = vec![Sym16(!0); SYM16_COUNT * SYM16_COUNT];

                for s1 in Sym16::iter() {
                    let r1: CubieLevel = s1.into();

                    for s2 in Sym16::iter() {
                        let r2: CubieLevel = s2.into();
                        let r2 = &r2 * &r1;

                        memo[s2.0 as usize * SYM16_COUNT + s1.0 as usize] = Sym16::iter()
                            .find(|&s| {
                                let r3: CubieLevel = s.into();

                                r2 == r3
                            })
                            .unwrap();
                    }
                }
                memo
            };
        }
        MEMO[self.0 as usize * SYM16_COUNT + rhs.0 as usize]
    }
}
#[test]
fn mul_sym() {
    // must be P2Move
    let cube = Move::U1 * (Move::F2 * (Move::U3 * (Move::D1 * SOLVED)));

    for s1 in Sym16::iter() {
        for s2 in Sym16::iter() {
            assert_eq!(
                crate::RubikCube(s1 * (s2 * cube)),
                crate::RubikCube((s1 * s2) * cube),
                "{:?} {:?}",
                Sym16Vec::from(s1),
                Sym16Vec::from(s2),
            );
        }
    }
}

impl Mul<CubieLevel> for Sym16Vec {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        let Sym16Vec(f, u, lr) = self;
        f * (u * (lr * rhs))
    }
}

impl Mul<CubieLevel> for Sym16 {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        let v: Sym16Vec = self.into();
        v * rhs
    }
}

pub struct Sym16Iterator(u8);
impl Sym16 {
    pub fn iter() -> Sym16Iterator {
        Sym16Iterator(0)
    }
}
impl std::iter::Iterator for Sym16Iterator {
    type Item = Sym16;

    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.0;
        if cur < SYM16_COUNT as u8 {
            self.0 += 1;
            return Some(Sym16(cur));
        }
        None
    }
}

impl Mul<Move> for SymF {
    type Output = Option<Move>;

    fn mul(self, rhs: Move) -> Self::Output {
        match self {
            Self::F0 => Some(rhs), //
            _ => {
                let res = rhs * SOLVED;
                let res = &res * <&CubieLevel>::from(self.inv());
                let res = <&CubieLevel>::from(self) * &res;
                for m in Move::iter() {
                    if res == m * SOLVED {
                        return Some(m);
                    }
                }
                None
            }
        }
    }
}
impl Mul<Move> for SymU {
    type Output = Option<Move>;

    fn mul(self, rhs: Move) -> Self::Output {
        match self {
            Self::U0 => Some(rhs), //
            _ => {
                let res = rhs * SOLVED;
                let res = &res * <&CubieLevel>::from(self.inv());
                let res = <&CubieLevel>::from(self) * &res;
                for m in Move::iter() {
                    if res == m * SOLVED {
                        return Some(m);
                    }
                }
                None
            }
        }
    }
}
impl Mul<Move> for SymLR {
    type Output = Option<Move>;

    fn mul(self, rhs: Move) -> Self::Output {
        match self {
            Self::LR0 => Some(rhs), //
            _ => {
                let res = rhs * SOLVED;
                let res = &res * <&CubieLevel>::from(self.inv());
                let res = <&CubieLevel>::from(self) * &res;
                for m in Move::iter() {
                    if res == m * SOLVED {
                        return Some(m);
                    }
                }
                None
            }
        }
    }
}

impl Mul<Move> for Sym16Vec {
    type Output = Option<Move>;

    fn mul(self, rhs: Move) -> Self::Output {
        let Sym16Vec(f, u, lr) = self;
        f * (u * (lr * rhs)?)?
    }
}

impl Mul<Move> for Sym16 {
    type Output = Option<Move>;

    fn mul(self, rhs: Move) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<Option<Move>> = {
                let mut res = vec![None; SYM16_COUNT * MOVE_COUNT];
                for s in Sym16::iter() {
                    for m in Move::iter() {
                        let sv: Sym16Vec = s.into();
                        res[s.0 as usize * MOVE_COUNT + m as usize] = sv * m;
                    }
                }
                res
            };
        }
        MEMO[self.0 as usize * MOVE_COUNT + rhs as usize]
    }
}
