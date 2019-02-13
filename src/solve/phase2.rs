use super::cube;

const P2_MOVES: [cube::Move; 10] = [
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
        assert!(idx < 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1);

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

const P2_EDGES: [u16; 8] = ([
    cube::EdgeCubePos::UR as u16,
    cube::EdgeCubePos::UF as u16,
    cube::EdgeCubePos::UL as u16,
    cube::EdgeCubePos::UB as u16,
    cube::EdgeCubePos::DR as u16,
    cube::EdgeCubePos::DF as u16,
    cube::EdgeCubePos::DL as u16,
    cube::EdgeCubePos::DB as u16,
]);

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
        assert!(idx < 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1);

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
        assert!(idx < 4 * 3 * 2 * 1);

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
    cperm_movetable: [CPerm; 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1],
    eperm_movetable: [EPerm; 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1],
    udslice_movetable: [UDSlice; 4 * 3 * 2 * 1],
}

impl Phase2 {
    fn new() -> Self {
        let mut p2 = Phase2 {
            cperm_movetable: [CPerm(!0); 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1],
            eperm_movetable: [EPerm(!0); 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1],
            udslice_movetable: [UDSlice(!0); 4 * 3 * 2 * 1],
        };

        p2
    }
}

impl super::Phase for Phase2 {
    fn solve(&self, src: &cube::RubikCube) -> Vec<cube::Move> {
        vec![]
    }
}
