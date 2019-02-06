use std::ops::Mul;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
enum CornerCubePos {
    URF, UFL, ULB, UBR, DFR, DLF, DBL, DRB,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
enum EdgeCubePos {
    UR, UF, UL, UB, DR, DF, DL, DB, FR, FL, BL, BR,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CornerCube {
    c: CornerCubePos,
    o: u8, // [0, 3)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct EdgeCube {
    e: EdgeCubePos,
    o: u8, // [0, 2)
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Subst([CornerCube; 8], [EdgeCube; 12]);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[cfg_attr(rustfmt, rustfmt_skip)]
enum Move {
    U1, U2, U3,
    D1, D2, D3,
    F1, F2, F3,
    B1, B2, B3,
    L1, L2, L3,
    R1, R2, R3,
}

impl Mul for Subst {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let mut res = Subst(
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
            res.0[i] = self.0[rhs.0[i].c as usize];
            res.0[i].o += rhs.0[i].o;
            res.0[i].o %= 3;
        }

        for i in 0..12 {
            res.1[i] = self.1[rhs.1[i].e as usize];
            res.1[i].o += rhs.1[i].o;
            res.1[i].o %= 2;
        }

        res
    }
}

impl From<Move> for Subst {
    fn from(p: Move) -> Subst {
        use self::CornerCubePos::*;
        use self::EdgeCubePos::*;
        use self::Move::*;
        match p {
            U1 => Subst(
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
            U2 => Self::from(U1) * Self::from(U1),
            U3 => Self::from(U1) * Self::from(U2),

            R1 => Subst(
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
            R2 => Self::from(R1) * Self::from(R1),
            R3 => Self::from(R1) * Self::from(R2),

            F1 => Subst(
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
            F2 => Self::from(F1) * Self::from(F1),
            F3 => Self::from(F1) * Self::from(F2),

            D1 => Subst(
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
            D2 => Self::from(D1) * Self::from(D1),
            D3 => Self::from(D1) * Self::from(D2),

            L1 => Subst(
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
            L2 => Self::from(L1) * Self::from(L1),
            L3 => Self::from(L1) * Self::from(L2),

            B1 => Subst(
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
            B2 => Self::from(B1) * Self::from(B1),
            B3 => Self::from(B1) * Self::from(B2),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        use super::Subst;
        use super::Move::*;
        for (m1, m2, m3) in &[
            (U1, U2, U3),
            (D1, D2, D3),
            (F1, F2, F3),
            (B1, B2, B3),
            (L1, L2, L3),
            (R1, R2, R3),
        ] {
            let m1 = Subst::from(*m1);
            let m2 = Subst::from(*m2);
            let m3 = Subst::from(*m3);

            assert!(m1 != m2);
            assert!(m2 != m3);
            assert!(m3 != m1);

            assert_eq!(m1 * m1, m2);
            assert_eq!(m2 * m1, m3);
            assert_eq!(m3 * m3 * m3, m1);
            assert_eq!(m1 * m2 * m2, m1);
            assert_eq!(m2 * m1 * m2, m1);
            assert_eq!(m2 * m2 * m1, m1);
        }
    }
}
