use std::fmt;

use super::cube::CornerCubePos;
use super::cube::EdgeCubePos;

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RubikCube(pub super::cube::CubieLevel);

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

        use CornerCubePos::*;
        use EdgeCubePos::*;
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
