use num_traits::cast::FromPrimitive;
use std::ops::Mul;
use strum::IntoEnumIterator;

#[cfg_attr(rustfmt, rustfmt_skip)]
pub const SOLVED: CubieLevel = CubieLevel(
    [
        CornerCube { c: CornerCubePos::URF, o: 0 },
        CornerCube { c: CornerCubePos::UFL, o: 0 },
        CornerCube { c: CornerCubePos::ULB, o: 0 },
        CornerCube { c: CornerCubePos::UBR, o: 0 },
        CornerCube { c: CornerCubePos::DFR, o: 0 },
        CornerCube { c: CornerCubePos::DLF, o: 0 },
        CornerCube { c: CornerCubePos::DBL, o: 0 },
        CornerCube { c: CornerCubePos::DRB, o: 0 },
    ],
    [
        EdgeCube { e: EdgeCubePos::UR, o: 0 },
        EdgeCube { e: EdgeCubePos::UF, o: 0 },
        EdgeCube { e: EdgeCubePos::UL, o: 0 },
        EdgeCube { e: EdgeCubePos::UB, o: 0 },
        EdgeCube { e: EdgeCubePos::DR, o: 0 },
        EdgeCube { e: EdgeCubePos::DF, o: 0 },
        EdgeCube { e: EdgeCubePos::DL, o: 0 },
        EdgeCube { e: EdgeCubePos::DB, o: 0 },
        EdgeCube { e: EdgeCubePos::FR, o: 0 },
        EdgeCube { e: EdgeCubePos::FL, o: 0 },
        EdgeCube { e: EdgeCubePos::BL, o: 0 },
        EdgeCube { e: EdgeCubePos::BR, o: 0 },
    ],
);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum CornerCubePos {
    URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
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
#[cfg_attr(rustfmt, rustfmt_skip)]
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

fn inner_mul(lhs: &CubieLevel, rhs: &CubieLevel) -> CubieLevel {
    let mut res = CubieLevel(
        [CornerCube {
            c: CornerCubePos::UBR,
            o: 0,
        }; 8],
        [EdgeCube {
            e: EdgeCubePos::UR,
            o: 0,
        }; 12],
    );

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

impl Mul<CubieLevel> for Move {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        fn subst(m: Move) -> &'static CubieLevel {
            use self::CornerCubePos::*;
            use self::EdgeCubePos::*;
            use self::Move::*;

            const U1_CUBE: CubieLevel = CubieLevel(
                [
                    CornerCube { c: UBR, o: 0 },
                    CornerCube { c: URF, o: 0 },
                    CornerCube { c: UFL, o: 0 },
                    CornerCube { c: ULB, o: 0 },
                    CornerCube { c: DFR, o: 0 },
                    CornerCube { c: DLF, o: 0 },
                    CornerCube { c: DBL, o: 0 },
                    CornerCube { c: DRB, o: 0 },
                ],
                [
                    EdgeCube { e: UB, o: 0 },
                    EdgeCube { e: UR, o: 0 },
                    EdgeCube { e: UF, o: 0 },
                    EdgeCube { e: UL, o: 0 },
                    EdgeCube { e: DR, o: 0 },
                    EdgeCube { e: DF, o: 0 },
                    EdgeCube { e: DL, o: 0 },
                    EdgeCube { e: DB, o: 0 },
                    EdgeCube { e: FR, o: 0 },
                    EdgeCube { e: FL, o: 0 },
                    EdgeCube { e: BL, o: 0 },
                    EdgeCube { e: BR, o: 0 },
                ],
            );
            const R1_CUBE: CubieLevel = CubieLevel(
                [
                    CornerCube { c: DFR, o: 2 },
                    CornerCube { c: UFL, o: 0 },
                    CornerCube { c: ULB, o: 0 },
                    CornerCube { c: URF, o: 1 },
                    CornerCube { c: DRB, o: 1 },
                    CornerCube { c: DLF, o: 0 },
                    CornerCube { c: DBL, o: 0 },
                    CornerCube { c: UBR, o: 2 },
                ],
                [
                    EdgeCube { e: FR, o: 0 },
                    EdgeCube { e: UF, o: 0 },
                    EdgeCube { e: UL, o: 0 },
                    EdgeCube { e: UB, o: 0 },
                    EdgeCube { e: BR, o: 0 },
                    EdgeCube { e: DF, o: 0 },
                    EdgeCube { e: DL, o: 0 },
                    EdgeCube { e: DB, o: 0 },
                    EdgeCube { e: DR, o: 0 },
                    EdgeCube { e: FL, o: 0 },
                    EdgeCube { e: BL, o: 0 },
                    EdgeCube { e: UR, o: 0 },
                ],
            );

            const F1_CUBE: CubieLevel = CubieLevel(
                [
                    CornerCube { c: UFL, o: 1 },
                    CornerCube { c: DLF, o: 2 },
                    CornerCube { c: ULB, o: 0 },
                    CornerCube { c: UBR, o: 0 },
                    CornerCube { c: URF, o: 2 },
                    CornerCube { c: DFR, o: 1 },
                    CornerCube { c: DBL, o: 0 },
                    CornerCube { c: DRB, o: 0 },
                ],
                [
                    EdgeCube { e: UR, o: 0 },
                    EdgeCube { e: FL, o: 1 },
                    EdgeCube { e: UL, o: 0 },
                    EdgeCube { e: UB, o: 0 },
                    EdgeCube { e: DR, o: 0 },
                    EdgeCube { e: FR, o: 1 },
                    EdgeCube { e: DL, o: 0 },
                    EdgeCube { e: DB, o: 0 },
                    EdgeCube { e: UF, o: 1 },
                    EdgeCube { e: DF, o: 1 },
                    EdgeCube { e: BL, o: 0 },
                    EdgeCube { e: BR, o: 0 },
                ],
            );
            const D1_CUBE: CubieLevel = CubieLevel(
                [
                    CornerCube { c: URF, o: 0 },
                    CornerCube { c: UFL, o: 0 },
                    CornerCube { c: ULB, o: 0 },
                    CornerCube { c: UBR, o: 0 },
                    CornerCube { c: DLF, o: 0 },
                    CornerCube { c: DBL, o: 0 },
                    CornerCube { c: DRB, o: 0 },
                    CornerCube { c: DFR, o: 0 },
                ],
                [
                    EdgeCube { e: UR, o: 0 },
                    EdgeCube { e: UF, o: 0 },
                    EdgeCube { e: UL, o: 0 },
                    EdgeCube { e: UB, o: 0 },
                    EdgeCube { e: DF, o: 0 },
                    EdgeCube { e: DL, o: 0 },
                    EdgeCube { e: DB, o: 0 },
                    EdgeCube { e: DR, o: 0 },
                    EdgeCube { e: FR, o: 0 },
                    EdgeCube { e: FL, o: 0 },
                    EdgeCube { e: BL, o: 0 },
                    EdgeCube { e: BR, o: 0 },
                ],
            );
            const L1_CUBE: CubieLevel = CubieLevel(
                [
                    CornerCube { c: URF, o: 0 },
                    CornerCube { c: ULB, o: 1 },
                    CornerCube { c: DBL, o: 2 },
                    CornerCube { c: UBR, o: 0 },
                    CornerCube { c: DFR, o: 0 },
                    CornerCube { c: UFL, o: 2 },
                    CornerCube { c: DLF, o: 1 },
                    CornerCube { c: DRB, o: 0 },
                ],
                [
                    EdgeCube { e: UR, o: 0 },
                    EdgeCube { e: UF, o: 0 },
                    EdgeCube { e: BL, o: 0 },
                    EdgeCube { e: UB, o: 0 },
                    EdgeCube { e: DR, o: 0 },
                    EdgeCube { e: DF, o: 0 },
                    EdgeCube { e: FL, o: 0 },
                    EdgeCube { e: DB, o: 0 },
                    EdgeCube { e: FR, o: 0 },
                    EdgeCube { e: UL, o: 0 },
                    EdgeCube { e: DL, o: 0 },
                    EdgeCube { e: BR, o: 0 },
                ],
            );
            const B1_CUBE: CubieLevel = CubieLevel(
                [
                    CornerCube { c: URF, o: 0 },
                    CornerCube { c: UFL, o: 0 },
                    CornerCube { c: UBR, o: 1 },
                    CornerCube { c: DRB, o: 2 },
                    CornerCube { c: DFR, o: 0 },
                    CornerCube { c: DLF, o: 0 },
                    CornerCube { c: ULB, o: 2 },
                    CornerCube { c: DBL, o: 1 },
                ],
                [
                    EdgeCube { e: UR, o: 0 },
                    EdgeCube { e: UF, o: 0 },
                    EdgeCube { e: UL, o: 0 },
                    EdgeCube { e: BR, o: 1 },
                    EdgeCube { e: DR, o: 0 },
                    EdgeCube { e: DF, o: 0 },
                    EdgeCube { e: DL, o: 0 },
                    EdgeCube { e: BL, o: 1 },
                    EdgeCube { e: FR, o: 0 },
                    EdgeCube { e: FL, o: 0 },
                    EdgeCube { e: UB, o: 1 },
                    EdgeCube { e: DB, o: 1 },
                ],
            );

            lazy_static! {
                static ref U2_CUBE: CubieLevel = inner_mul(&U1_CUBE, &U1_CUBE);
                static ref U3_CUBE: CubieLevel = inner_mul(&U1_CUBE, &U2_CUBE);
                static ref R2_CUBE: CubieLevel = inner_mul(&R1_CUBE, &R1_CUBE);
                static ref R3_CUBE: CubieLevel = inner_mul(&R1_CUBE, &R2_CUBE);
                static ref F2_CUBE: CubieLevel = inner_mul(&F1_CUBE, &F1_CUBE);
                static ref F3_CUBE: CubieLevel = inner_mul(&F1_CUBE, &F2_CUBE);
                static ref D2_CUBE: CubieLevel = inner_mul(&D1_CUBE, &D1_CUBE);
                static ref D3_CUBE: CubieLevel = inner_mul(&D1_CUBE, &D2_CUBE);
                static ref L2_CUBE: CubieLevel = inner_mul(&L1_CUBE, &L1_CUBE);
                static ref L3_CUBE: CubieLevel = inner_mul(&L1_CUBE, &L2_CUBE);
                static ref B2_CUBE: CubieLevel = inner_mul(&B1_CUBE, &B1_CUBE);
                static ref B3_CUBE: CubieLevel = inner_mul(&B1_CUBE, &B2_CUBE);
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

        let lhs = subst(self);

        inner_mul(lhs, &rhs)
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
#[cfg_attr(rustfmt, rustfmt_skip)]
enum SymF { F0, F1 }

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
#[cfg_attr(rustfmt, rustfmt_skip)]
enum SymU { U0, U1, U2, U3 }

#[derive(Debug, Copy, Clone, PartialEq, Eq, EnumCount, EnumIter, FromPrimitive)]
#[cfg_attr(rustfmt, rustfmt_skip)]
enum SymLR { LR0, LR1 }

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Sym16Vec(SymF, SymU, SymLR);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Sym16(u8);
const SYM16_COUNT: usize = 16;

impl From<SymF> for &'static CubieLevel {
    fn from(src: SymF) -> &'static CubieLevel {
        use CornerCubePos::*;
        use EdgeCubePos::*;

        const F1_CUBE: CubieLevel = CubieLevel(
            [
                CornerCube { c: DLF, o: 0 },
                CornerCube { c: DFR, o: 0 },
                CornerCube { c: DRB, o: 0 },
                CornerCube { c: DBL, o: 0 },
                CornerCube { c: UFL, o: 0 },
                CornerCube { c: URF, o: 0 },
                CornerCube { c: UBR, o: 0 },
                CornerCube { c: ULB, o: 0 },
            ],
            [
                EdgeCube { e: DL, o: 0 },
                EdgeCube { e: DF, o: 0 },
                EdgeCube { e: DR, o: 0 },
                EdgeCube { e: DB, o: 0 },
                EdgeCube { e: UL, o: 0 },
                EdgeCube { e: UF, o: 0 },
                EdgeCube { e: UR, o: 0 },
                EdgeCube { e: UB, o: 0 },
                EdgeCube { e: FL, o: 0 },
                EdgeCube { e: FR, o: 0 },
                EdgeCube { e: BR, o: 0 },
                EdgeCube { e: BL, o: 0 },
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
                CornerCube { c: UBR, o: 0 },
                CornerCube { c: URF, o: 0 },
                CornerCube { c: UFL, o: 0 },
                CornerCube { c: ULB, o: 0 },
                CornerCube { c: DRB, o: 0 },
                CornerCube { c: DFR, o: 0 },
                CornerCube { c: DLF, o: 0 },
                CornerCube { c: DBL, o: 0 },
            ],
            [
                EdgeCube { e: UB, o: 1 },
                EdgeCube { e: UR, o: 1 },
                EdgeCube { e: UF, o: 1 },
                EdgeCube { e: UL, o: 1 },
                EdgeCube { e: DB, o: 1 },
                EdgeCube { e: DR, o: 1 },
                EdgeCube { e: DF, o: 1 },
                EdgeCube { e: DL, o: 1 },
                EdgeCube { e: BR, o: 1 },
                EdgeCube { e: FR, o: 1 },
                EdgeCube { e: FL, o: 1 },
                EdgeCube { e: BL, o: 1 },
            ],
        );
        lazy_static! {
            static ref U2_CUBE: CubieLevel = inner_mul(&U1_CUBE, &U1_CUBE);
            static ref U3_CUBE: CubieLevel = inner_mul(&U1_CUBE, &U2_CUBE);
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
                CornerCube { c: UFL, o: 3 },
                CornerCube { c: URF, o: 3 },
                CornerCube { c: UBR, o: 3 },
                CornerCube { c: ULB, o: 3 },
                CornerCube { c: DLF, o: 3 },
                CornerCube { c: DFR, o: 3 },
                CornerCube { c: DRB, o: 3 },
                CornerCube { c: DBL, o: 3 },
            ],
            [
                EdgeCube { e: UL, o: 0 },
                EdgeCube { e: UF, o: 0 },
                EdgeCube { e: UR, o: 0 },
                EdgeCube { e: UB, o: 0 },
                EdgeCube { e: DL, o: 0 },
                EdgeCube { e: DF, o: 0 },
                EdgeCube { e: DR, o: 0 },
                EdgeCube { e: DB, o: 0 },
                EdgeCube { e: FL, o: 0 },
                EdgeCube { e: FR, o: 0 },
                EdgeCube { e: BR, o: 0 },
                EdgeCube { e: BL, o: 0 },
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
            F0 => rhs,
            F1 => rhs.inv(),
        }
    }
}
impl Mul<CubieLevel> for SymF {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        match self {
            Self::F0 => rhs, //
            _ => {
                let res = inner_mul(&rhs, self.into());
                inner_mul(self.inv().into(), &res)
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
                let res = inner_mul(&rhs, self.into());
                inner_mul(self.inv().into(), &res)
            }
        }
    }
}

impl Inv for SymLR {
    fn inv(self) -> Self {
        use SymLR::*;
        match self {
            LR0 => LR0,
            LR1 => LR1,
        }
    }
}
impl Mul<SymLR> for SymLR {
    type Output = SymLR;

    fn mul(self, rhs: SymLR) -> Self::Output {
        match self {
            LR0 => rhs,
            LR1 => rhs.inv(),
        }
    }
}
impl Mul<CubieLevel> for SymLR {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        match self {
            Self::LR0 => rhs,
            _ => {
                let res = inner_mul(&rhs, self.into());
                inner_mul(self.inv().into(), &res)
            }
        }
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
            static ref MEMO: [Sym16; SYM16_COUNT] = {
                let mut res = [Sym16(0); SYM16_COUNT];
                for (i, e) in res.iter_mut().enumerate() {
                    let v: Sym16Vec = Sym16(i as u8).into();
                    *e = v.inv().into();
                }
                res
            };
        }
        MEMO[self.0 as usize]
    }
}

impl Mul<Sym16Vec> for Sym16Vec {
    type Output = Sym16Vec;

    fn mul(self, rhs: Sym16Vec) -> Self::Output {
        let Sym16Vec(f1, u1, lr1) = self;
        let Sym16Vec(f2, u2, lr2) = rhs;
        Sym16Vec(f1 * f2, u1 * u2, lr1 * lr2)
    }
}
impl Mul<Sym16> for Sym16 {
    type Output = Sym16;

    fn mul(self, rhs: Sym16) -> Self::Output {
        let v1: Sym16Vec = self.into();
        let v2: Sym16Vec = rhs.into();
        (v1 * v2).into()
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
                let res = inner_mul(&res, self.inv().into());
                let res = inner_mul(self.into(), &res);
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
                let res = inner_mul(&res, self.inv().into());
                let res = inner_mul(self.into(), &res);
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
                let res = inner_mul(&res, self.inv().into());
                let res = inner_mul(self.into(), &res);
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
            static ref MEMO: [Option<Move>; SYM16_COUNT * MOVE_COUNT] = {
                let mut res = [None; SYM16_COUNT * MOVE_COUNT];
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
