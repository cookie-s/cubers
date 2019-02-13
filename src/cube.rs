use std::fmt;
use std::ops::Mul;

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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RubikCube(pub CubieLevel);

impl fmt::Debug for RubikCube {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[derive(Clone,Copy)]
        #[cfg_attr(rustfmt, rustfmt_skip)]
        enum Color { B, W, R, Y, O, G };
        impl fmt::Display for Color {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    B => write!(f, "\x1b[44m  \x1b[0m"),
                    W => write!(f, "\x1b[47m  \x1b[0m"),
                    R => write!(f, "\x1b[41m  \x1b[0m"),
                    Y => write!(f, "\x1b[43m  \x1b[0m"),
                    O => write!(f, "\x1b[46m  \x1b[0m"),
                    G => write!(f, "\x1b[42m  \x1b[0m"),
                }
            }
        }

        // TODO: 全体的に無意味にここにハードコードしない
        use Color::*;
        const CCOLS: [[Color; 3]; 8] = [
            [B, Y, R],
            [B, R, W],
            [B, W, O],
            [B, O, Y],
            [G, R, Y],
            [G, W, R],
            [G, O, W],
            [G, Y, O],
        ];
        const ECOLS: [[Color; 2]; 12] = [
            [B, Y],
            [B, R],
            [B, W],
            [B, O],
            [G, Y],
            [G, R],
            [G, W],
            [G, O],
            [R, Y],
            [R, W],
            [O, W],
            [O, Y],
        ];
        fn c(cb: &RubikCube, x: CornerCubePos, y: u8) -> Color {
            let c = &cb.0;
            CCOLS[c.0[x as usize].c as usize][((6 - c.0[x as usize].o + y) % 3) as usize]
        };
        fn e(cb: &RubikCube, x: EdgeCubePos, y: u8) -> Color {
            let c = &cb.0;
            ECOLS[c.1[x as usize].e as usize][((4 - c.1[x as usize].o + y) % 2) as usize]
        };

        use self::CornerCubePos::*;
        use self::EdgeCubePos::*;

        write!(f, "RubikCube {{\n")?;
        write!(
            f,
            "......{}{}{}............\n",
            c(self, ULB, 0),
            e(self, UB, 0),
            c(self, UBR, 0)
        )?;
        write!(
            f,
            "......{}{}{}............\n",
            e(self, UL, 0),
            B,
            e(self, UR, 0)
        )?;
        write!(
            f,
            "......{}{}{}............\n",
            c(self, UFL, 0),
            e(self, UF, 0),
            c(self, URF, 0)
        )?;
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}\n",
            c(self, ULB, 1),
            e(self, UL, 1),
            c(self, UFL, 2),
            c(self, UFL, 1),
            e(self, UF, 1),
            c(self, URF, 2),
            c(self, URF, 1),
            e(self, UR, 1),
            c(self, UBR, 2),
            c(self, UBR, 1),
            e(self, UB, 1),
            c(self, ULB, 2),
        )?;
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}\n",
            e(self, BL, 1),
            W,
            e(self, FL, 1),
            e(self, FL, 0),
            R,
            e(self, FR, 0),
            e(self, FR, 1),
            Y,
            e(self, BR, 1),
            e(self, BR, 0),
            O,
            e(self, BL, 0),
        )?;
        write!(
            f,
            "{}{}{}{}{}{}{}{}{}{}{}{}\n",
            c(self, DBL, 2),
            e(self, DL, 1),
            c(self, DLF, 1),
            c(self, DLF, 2),
            e(self, DF, 1),
            c(self, DFR, 1),
            c(self, DFR, 2),
            e(self, DR, 1),
            c(self, DRB, 1),
            c(self, DRB, 2),
            e(self, DB, 1),
            c(self, DBL, 1),
        )?;
        write!(
            f,
            "......{}{}{}............\n",
            c(self, DLF, 0),
            e(self, DF, 0),
            c(self, DFR, 0)
        )?;
        write!(
            f,
            "......{}{}{}............\n",
            e(self, DL, 0),
            G,
            e(self, DR, 0)
        )?;
        write!(
            f,
            "......{}{}{}............\n",
            c(self, DBL, 0),
            e(self, DB, 0),
            c(self, DRB, 0)
        )?;
        write!(f, "}}\n")?;
        Ok(())
    }
}

impl Mul<CubieLevel> for Move {
    type Output = CubieLevel;

    fn mul(self, rhs: CubieLevel) -> Self::Output {
        fn subst(m: Move) -> CubieLevel {
            use self::CornerCubePos::*;
            use self::EdgeCubePos::*;
            use self::Move::*;
            match m {
                U1 => CubieLevel(
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
                ),
                U2 => U1 * subst(U1),
                U3 => U1 * subst(U2),

                R1 => CubieLevel(
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
                ),
                R2 => R1 * subst(R1),
                R3 => R1 * subst(R2),

                F1 => CubieLevel(
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
                ),
                F2 => F1 * subst(F1),
                F3 => F1 * subst(F2),

                D1 => CubieLevel(
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
                ),
                D2 => D1 * subst(D1),
                D3 => D1 * subst(D2),

                L1 => CubieLevel(
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
                ),
                L2 => L1 * subst(L1),
                L3 => L1 * subst(L2),

                B1 => CubieLevel(
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
                ),
                B2 => B1 * subst(B1),
                B3 => B1 * subst(B2),
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

#[cfg(test)]
mod tests {
    #[test]
    fn jimei() {
        use super::Move::*;
        use super::*;
        for (m1, m2, m3) in [
            (U1, U2, U3),
            (D1, D2, D3),
            (F1, F2, F3),
            (B1, B2, B3),
            (L1, L2, L3),
            (R1, R2, R3),
        ]
        .iter()
        {
            let (m1, m2, m3) = (*m1, *m2, *m3);
            assert_eq!(RubikCube(m1 * (m1 * SOLVED)), RubikCube(m2 * SOLVED));
            assert_eq!(RubikCube(m2 * (m1 * SOLVED)), RubikCube(m3 * SOLVED));
            assert_eq!(RubikCube(m3 * (m3 * (m3 * SOLVED))), RubikCube(m1 * SOLVED));
            assert_eq!(RubikCube(m1 * (m2 * (m2 * SOLVED))), RubikCube(m1 * SOLVED));
            assert_eq!(RubikCube(m2 * (m1 * (m2 * SOLVED))), RubikCube(m1 * SOLVED));
            assert_eq!(RubikCube(m2 * (m2 * (m1 * SOLVED))), RubikCube(m1 * SOLVED));
        }
    }
}
