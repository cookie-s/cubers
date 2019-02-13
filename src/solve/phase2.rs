use super::cube;

const FACT4: usize = 4 * 3 * 2 * 1;
const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;

const P2_MOVES_SIZE: usize = 10;
const P2_MOVES: [cube::Move; P2_MOVES_SIZE] = [
    cube::Move::U1,
    cube::Move::U2,
    cube::Move::U3,
    cube::Move::D1,
    cube::Move::D2,
    cube::Move::D3,
    cube::Move::F2,
    cube::Move::B2,
    cube::Move::L2,
    cube::Move::R2,
];

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CPerm(u16); // Corner Permutation Coordinate
impl From<cube::CubieLevel> for CPerm {
    fn from(cl: cube::CubieLevel) -> CPerm {
        let mut aa = [0; 8];
        for i in 0..8 {
            aa[i] = i as u16;
        }

        let mut res = 0;
        let mut b = 1;
        for (i, c) in cl.0.iter().map(|c| c.c as u16).enumerate().rev() {
            let k = aa.iter().position(|&x| c == x).unwrap();
            res += k * b;
            b *= i + 1;

            let t = aa[i];
            aa[i] = aa[k];
            aa[k] = t;
        }

        CPerm(res as u16)
    }
}
impl From<CPerm> for cube::CubieLevel {
    // return a representation
    fn from(cp: CPerm) -> cube::CubieLevel {
        let mut idx = cp.0;
        assert!(idx < FACT8 as u16);

        let mut res = cube::SOLVED;
        for i in (0..8).rev() {
            let j = idx % (i + 1);
            let t = res.0[i as usize].c;
            res.0[i as usize].c = res.0[j as usize].c;
            res.0[j as usize].c = t;
            idx /= i + 1;
        }
        res
    }
}

#[test]
fn cperm() {
    use super::*;
    for m in P2_MOVES.iter() {
        let m = *m;
        let cube = m * cube::SOLVED;
        let cp: CPerm = cube.into();
        assert_eq!(
            cube.0
                .iter()
                .map(|c| c.c)
                .collect::<Vec<cube::CornerCubePos>>(),
            cube::CubieLevel::from(cp)
                .0
                .iter()
                .map(|c| c.c)
                .collect::<Vec<cube::CornerCubePos>>()
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct EPerm(u16); // Edge Permutation Coordinate
impl From<cube::CubieLevel> for EPerm {
    fn from(cl: cube::CubieLevel) -> EPerm {
        let mut aa = [0; 8];
        for i in 0..8 {
            aa[i] = i as u16;
        }

        let mut res = 0;
        let mut b = 1;
        for (i, e) in cl.1.iter().map(|e| e.e as u16).enumerate().rev() {
            // FIXME: Take does not implement DoubleEndedIterator...
            if i >= 8 {
                continue;
            }

            let k = aa.iter().position(|&x| e == x).unwrap();
            res += k * b;
            b *= i + 1;

            let t = aa[i];
            aa[i] = aa[k];
            aa[k] = t;
        }

        EPerm(res as u16)
    }
}
impl From<EPerm> for cube::CubieLevel {
    // return a representation
    fn from(ep: EPerm) -> cube::CubieLevel {
        let mut idx = ep.0;
        assert!(idx < FACT8 as u16);

        let mut res = cube::SOLVED;
        for i in (0..8).rev() {
            let j = idx % (i + 1);
            let t = res.1[i as usize].e;
            res.1[i as usize].e = res.1[j as usize].e;
            res.1[j as usize].e = t;
            idx /= i + 1;
        }
        res
    }
}

#[test]
fn eperm() {
    use super::cube::Move::*;
    use super::*;
    for m in P2_MOVES.iter() {
        let m = *m;
        let cube = m * cube::SOLVED;
        let ep: EPerm = cube.into();
        assert_eq!(
            cube.1
                .iter()
                .map(|e| e.e)
                .take(8)
                .collect::<Vec<cube::EdgeCubePos>>(),
            cube::CubieLevel::from(ep)
                .1
                .iter()
                .map(|e| e.e)
                .take(8)
                .collect::<Vec<cube::EdgeCubePos>>()
        );
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct UDSlice(u8); // phase2 UDSlice Coordinate
impl From<cube::CubieLevel> for UDSlice {
    fn from(cl: cube::CubieLevel) -> UDSlice {
        let mut aa = [0; 4];
        for i in 0..4 {
            aa[i] = i as u16 + 8;
        }

        let mut res = 0;
        let mut b = 1;
        for (i, c) in cl.1.iter().map(|e| e.e as u16).enumerate().rev() {
            if i < 8 {
                continue;
            }
            let i = i - 8;

            let k = aa.iter().position(|&x| c == x).unwrap();
            res += k * b;
            b *= i + 1;

            let t = aa[i];
            aa[i] = aa[k];
            aa[k] = t;
        }

        UDSlice(res as u8)
    }
}
impl From<UDSlice> for cube::CubieLevel {
    // return a representation
    fn from(uds: UDSlice) -> cube::CubieLevel {
        let mut idx = uds.0;
        assert!(idx < FACT4 as u8);

        let mut res = cube::SOLVED;
        for i in (0..4).rev() {
            let j = idx % (i + 1);
            let t = res.1[i as usize + 8].e;
            res.1[i as usize + 8].e = res.1[j as usize + 8].e;
            res.1[j as usize + 8].e = t;
            idx /= i + 1;
        }
        res
    }
}

#[test]
fn udslice() {
    use super::*;
    for m in P2_MOVES.iter() {
        let m = *m;
        let cube = m * cube::SOLVED;
        let ep: UDSlice = cube.into();
        assert_eq!(
            cube.1
                .iter()
                .map(|e| e.e)
                .skip(8)
                .collect::<Vec<cube::EdgeCubePos>>(),
            cube::CubieLevel::from(ep)
                .1
                .iter()
                .map(|e| e.e)
                .skip(8)
                .collect::<Vec<cube::EdgeCubePos>>()
        );
    }
}

pub struct Phase2 {
    cperm_movetable: [CPerm; FACT8 * P2_MOVES_SIZE],
    eperm_movetable: [EPerm; FACT8 * P2_MOVES_SIZE],
    udslice_movetable: [UDSlice; FACT4 * P2_MOVES_SIZE],
}

impl Phase2 {
    fn new() -> Self {
        let mut p2 = Phase2 {
            cperm_movetable: [CPerm(!0); FACT8 * P2_MOVES_SIZE],
            eperm_movetable: [EPerm(!0); FACT8 * P2_MOVES_SIZE],
            udslice_movetable: [UDSlice(!0); FACT4 * P2_MOVES_SIZE],
        };

        // cperm
        for i in 0..FACT8 {
            let mut cube: cube::CubieLevel = CPerm(i as u16).into();
            for m in P2_MOVES.iter() {
                let m = *m;
                let v: CPerm = (m * cube).into();
                p2.cperm_movetable[i * P2_MOVES_SIZE + (m as usize)] = v;
            }
        }

        // eperm
        for i in 0..FACT8 {
            let mut cube: cube::CubieLevel = CPerm(i as u16).into();
            for m in P2_MOVES.iter() {
                let m = *m;
                let v: EPerm = (m * cube).into();
                p2.eperm_movetable[i * P2_MOVES_SIZE + (m as usize)] = v;
            }
        }

        // udslice
        for i in 0..FACT8 {
            let mut cube: cube::CubieLevel = CPerm(i as u16).into();
            for m in P2_MOVES.iter() {
                let m = *m;
                let v: UDSlice = (m * cube).into();
                p2.udslice_movetable[i * P2_MOVES_SIZE + (m as usize)] = v;
            }
        }
        p2
    }
}

struct Phase2Cube(cube::RubikCube);
impl core::convert::TryFrom<cube::RubikCube> for Phase2Cube {
    type Error = ();
    fn try_from(src: cube::RubikCube) -> Result<Self, Self::Error> {
        fn is_phase2(cube: cube::RubikCube) -> bool {
            (cube.0).0.iter().all(|c| c.o == 0)
                && (cube.0).1.iter().all(|e| e.o == 0)
                && (cube.0)
                    .1
                    .iter()
                    .enumerate()
                    .all(|(i, e)| i < 8 || (e.e as u16 >= 8))
        }
        if !is_phase2(src) {
            return Err(());
        }
        Ok(Phase2Cube(src))
    }
}

impl super::Phase for Phase2 {
    type Error = ();
    fn solve(&self, src: &cube::RubikCube) -> Result<Vec<cube::Move>, Self::Error> {
        Ok(vec![])
    }
}
