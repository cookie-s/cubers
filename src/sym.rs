use super::cube::*;
use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(rustfmt, rustfmt_skip)]
pub enum SymMove {
    ID,
    URF1, URF2,
    F1,
    U1, U2, U3,
    LR1,
}

pub const S_URF: [SymMove; 3] = [SymMove::ID, SymMove::URF1, SymMove::URF2];
pub const S_F: [SymMove; 2] = [SymMove::ID, SymMove::F1];
pub const S_U: [SymMove; 4] = [SymMove::ID, SymMove::U1, SymMove::U2, SymMove::U3];
pub const S_LR: [SymMove; 2] = [SymMove::ID, SymMove::LR1];

impl SymMove {
    fn inv(self) -> Self {
        use self::SymMove::*;
        match self {
            ID => ID,

            URF1 => URF2,
            URF2 => URF1,

            F1 => F1,

            U1 => U3,
            U2 => U2,
            U3 => U1,

            LR1 => LR1,
        }
    }
}

impl Mul<CubieLevel> for SymMove {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        fn subst(m: SymMove) -> CubieLevel {
            use self::CornerCubePos::*;
            use self::EdgeCubePos::*;
            use self::SymMove::*;
            match m {
                ID => SOLVED,

                URF1 => unimplemented!(),
                URF2 => URF1 * subst(URF1),

                F1 => CubieLevel(
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
                ),

                U1 => CubieLevel(
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
                ),
                U2 => U1 * subst(U1),
                U3 => U1 * subst(U2),

                LR1 => CubieLevel(
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
                ),
                _ => panic!("panic"),
            }
        }

        let lhs = subst(self);

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
}
