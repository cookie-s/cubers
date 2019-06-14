use super::cube;

const FACT4: usize = 4 * 3 * 2 * 1;
const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum P2Move {
    U1,
    U2,
    U3,
    D1,
    D2,
    D3,
    F2,
    B2,
    L2,
    R2,
}
impl P2Move {
    fn from_usize(n: usize) -> Self {
        match n {
            0 => P2Move::U1,
            1 => P2Move::U2,
            2 => P2Move::U3,
            3 => P2Move::D1,
            4 => P2Move::D2,
            5 => P2Move::D3,
            6 => P2Move::F2,
            7 => P2Move::B2,
            8 => P2Move::L2,
            9 => P2Move::R2,
            _ => panic!("invalid argument"),
        }
    }
    fn to_usize(&self) -> usize {
        match self {
            P2Move::U1 => 0,
            P2Move::U2 => 1,
            P2Move::U3 => 2,
            P2Move::D1 => 3,
            P2Move::D2 => 4,
            P2Move::D3 => 5,
            P2Move::F2 => 6,
            P2Move::B2 => 7,
            P2Move::L2 => 8,
            P2Move::R2 => 9,
        }
    }
}

const P2_MOVES_SIZE: usize = 10;
const P2_MOVES: [P2Move; P2_MOVES_SIZE] = [
    P2Move::U1,
    P2Move::U2,
    P2Move::U3,
    P2Move::D1,
    P2Move::D2,
    P2Move::D3,
    P2Move::F2,
    P2Move::B2,
    P2Move::L2,
    P2Move::R2,
];

impl From<P2Move> for cube::Move {
    fn from(m: P2Move) -> cube::Move {
        match m {
            P2Move::U1 => cube::Move::U1,
            P2Move::U2 => cube::Move::U2,
            P2Move::U3 => cube::Move::U3,
            P2Move::D1 => cube::Move::D1,
            P2Move::D2 => cube::Move::D2,
            P2Move::D3 => cube::Move::D3,
            P2Move::F2 => cube::Move::F2,
            P2Move::B2 => cube::Move::B2,
            P2Move::L2 => cube::Move::L2,
            P2Move::R2 => cube::Move::R2,
        }
    }
}

impl std::ops::Mul<cube::CubieLevel> for P2Move {
    type Output = cube::CubieLevel;
    fn mul(self, rhs: cube::CubieLevel) -> cube::CubieLevel {
        let m: cube::Move = self.into();
        m * rhs
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct CPerm(u16); // Corner Permutation Coordinate
impl From<cube::CubieLevel> for CPerm {
    fn from(cl: cube::CubieLevel) -> CPerm {
        use super::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array: Vec<_> = cl.0.iter().map(|c| c.c as u16).collect();
        let res = shuffle.array_to_num(&array);
        CPerm(res as u16)
    }
}
impl From<CPerm> for cube::CubieLevel {
    // return a representation
    fn from(cp: CPerm) -> cube::CubieLevel {
        use super::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array = shuffle.num_to_array(cp.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..8 {
            res.0[i as usize].c = cube::SOLVED.0[array[i]].c;
        }
        return res;
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
        use super::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array: Vec<_> = cl.1.iter().map(|e| e.e as u16).take(8).collect();
        let res = shuffle.array_to_num(&array);
        EPerm(res as u16)
    }
}
impl From<EPerm> for cube::CubieLevel {
    // return a representation
    fn from(ep: EPerm) -> cube::CubieLevel {
        use super::util::FisherShuffle;
        let shuffle = FisherShuffle::new(8);

        let array = shuffle.num_to_array(ep.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..8 {
            res.1[i as usize].e = cube::SOLVED.1[array[i]].e;
        }
        res
    }
}

#[test]
fn eperm() {
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
        use super::util::FisherShuffle;
        let shuffle = FisherShuffle::new(4);

        let array: Vec<_> = cl.1.iter().skip(8).map(|e| e.e as u16 - 8).collect();
        let res = shuffle.array_to_num(&array);
        UDSlice(res as u8)
    }
}
impl From<UDSlice> for cube::CubieLevel {
    // return a representation
    fn from(uds: UDSlice) -> cube::CubieLevel {
        use super::util::FisherShuffle;
        let shuffle = FisherShuffle::new(4);

        let array = shuffle.num_to_array(uds.0 as usize);
        let mut res = cube::SOLVED;
        for i in 0..4 {
            res.1[i as usize + 8].e = cube::SOLVED.1[array[i] + 8].e;
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
    cperm_movetable: Vec<CPerm>,     // FACT8 * P2_MOVES_SIZE
    eperm_movetable: Vec<EPerm>,     // FACT8 * P2_MOVES_SIZE
    udslice_movetable: Vec<UDSlice>, // FACT4 * P2_MOVES_SIZE
    prunetable: Vec<u8>,             // FACT8 // TODO: this should be more efficient
}

impl Phase2 {
    pub fn new() -> Self {
        let mut p2 = Phase2 {
            cperm_movetable: vec![CPerm(!0); FACT8 * P2_MOVES_SIZE],
            eperm_movetable: vec![EPerm(!0); FACT8 * P2_MOVES_SIZE],
            udslice_movetable: vec![UDSlice(!0); FACT4 * P2_MOVES_SIZE],
            prunetable: vec![!0; FACT8],
        };

        // cperm
        for i in 0..FACT8 {
            let cube: cube::CubieLevel = CPerm(i as u16).into();
            for m in P2_MOVES.iter() {
                let m = *m;
                let v: CPerm = (m * cube).into();
                p2.cperm_movetable[i * P2_MOVES_SIZE + (m as usize)] = v;
            }
        }

        // eperm
        for i in 0..FACT8 {
            let cube: cube::CubieLevel = EPerm(i as u16).into();
            for m in P2_MOVES.iter() {
                let m = *m;
                let v: EPerm = (m * cube).into();
                p2.eperm_movetable[i * P2_MOVES_SIZE + (m as usize)] = v;
            }
        }

        // udslice
        for i in 0..FACT4 {
            let cube: cube::CubieLevel = UDSlice(i as u8).into();
            for m in P2_MOVES.iter() {
                let m = *m;
                let v: UDSlice = (m * cube).into();
                p2.udslice_movetable[i * P2_MOVES_SIZE + (m as usize)] = v;
            }
        }

        {
            let mut cnt = 1;
            let mut queue = std::collections::VecDeque::new();

            let solved: Phase2Cube = cube::RubikCube(cube::SOLVED).try_into().unwrap();
            let solved: Phase2Vec = solved.into();
            let eperm = solved.split().1;
            p2.prunetable[eperm.0 as usize] = 0;

            queue.push_back((0, solved));

            while cnt < FACT8 {
                let (dis, state) = queue.pop_front().unwrap();
                let eperm: EPerm = state.split().1;

                for m in P2_MOVES.iter() {
                    let m = *m;
                    let nextstate = state.rotate(&p2, m);
                    let nexteperm = nextstate.split().1;
                    if p2.prunetable[nexteperm.0 as usize] == !0 {
                        p2.prunetable[nexteperm.0 as usize] = dis + 1;
                        queue.push_back((dis + 1, nextstate));
                        cnt += 1;
                    }
                }
            }
        }

        p2
    }
}

use std::convert::{TryFrom, TryInto};
struct Phase2Cube(cube::RubikCube);
impl TryFrom<cube::RubikCube> for Phase2Cube {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Phase2Vec(u64);
impl<T: Into<Phase2Cube>> From<T> for Phase2Vec {
    fn from(src: T) -> Phase2Vec {
        let src: Phase2Cube = src.into();
        let (v1, v2, v3): (CPerm, EPerm, UDSlice) =
            ((src.0).0.into(), (src.0).0.into(), (src.0).0.into());
        Self::combine(v1, v2, v3)
    }
}

impl Phase2Vec {
    fn split(self) -> (CPerm, EPerm, UDSlice) {
        let t = self.0;
        let v3 = t % FACT4 as u64;

        let t = t / FACT4 as u64;
        let v2 = t % FACT8 as u64;

        let v1 = t / FACT8 as u64;

        (CPerm(v1 as u16), EPerm(v2 as u16), UDSlice(v3 as u8))
    }

    fn combine(cp: CPerm, ep: EPerm, uds: UDSlice) -> Self {
        Phase2Vec(((cp.0 as u64 * FACT8 as u64) + ep.0 as u64) * FACT4 as u64 + uds.0 as u64)
    }

    fn rotate(self, p2: &Phase2, m: P2Move) -> Self {
        let (cp, ep, uds) = self.split();
        let (cp, ep, uds) = (cp.0 as usize, ep.0 as usize, uds.0 as usize);

        let v1 = p2.cperm_movetable[cp * P2_MOVES_SIZE + m as usize];
        let v2 = p2.eperm_movetable[ep * P2_MOVES_SIZE + m as usize];
        let v3 = p2.udslice_movetable[uds * P2_MOVES_SIZE + m as usize];

        Self::combine(v1, v2, v3)
    }
}

#[test]
fn rotate_test() {
    use super::*;
    let p2 = Phase2::new();

    let solved: Phase2Cube = cube::RubikCube(cube::SOLVED).try_into().unwrap();
    let solved: Phase2Vec = solved.into();

    for m in P2_MOVES.iter() {
        let m = *m;

        let cube = m * cube::SOLVED;
        let cube: Phase2Cube = cube::RubikCube(cube).try_into().unwrap();
        let v1: Phase2Vec = cube.into();

        let v2 = solved.rotate(&p2, m);
        assert_eq!(v1.split(), v2.split(), "move {:?}", m);
    }

    use P2Move::*;
    let cube = cube::RubikCube(F2 * (U1 * (L2 * cube::SOLVED)));
    let cube: Phase2Cube = cube.try_into().unwrap();
    let cube: Phase2Vec = cube.into();
    let cube = cube.rotate(&p2, F2).rotate(&p2, U3).rotate(&p2, L2);
    assert_eq!(cube.split(), solved.split());
}

impl super::Phase for Phase2 {
    type Error = ();

    fn solve(&self, src: &cube::RubikCube) -> Result<Vec<cube::Move>, Self::Error> {
        use std::collections::{BinaryHeap, HashSet};

        fn recover_rotates(dist: usize, rotates: u64) -> Vec<cube::Move> {
            let mut rotates = rotates;
            let mut res = vec![cube::Move::U1; dist];

            for i in 0..dist {
                let p2move = P2Move::from_usize(rotates as usize % 10);
                res[dist - 1 - i] = p2move.into();
                rotates /= 10;
            }
            res
        }

        let solved: Phase2Cube = cube::RubikCube(cube::SOLVED).try_into().unwrap();
        let solved: Phase2Vec = solved.into();

        let src: Phase2Cube = (*src).try_into()?;
        let src: Phase2Vec = src.into();

        const MAX_STEPS: isize = 14; // TODO: 18
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();
        heap.push((-0, src, 0));
        set.insert(src);

        while let Some((dist, state, rotates)) = heap.pop() {
            let dist = -dist;
            let pruneval = self.prunetable[(state.split().1).0 as usize];
            //println!("{:?}", state);

            if state == solved {
                println!("{}", dist);
                return Ok(recover_rotates(dist as usize, rotates));
            }
            if dist + pruneval as isize > MAX_STEPS {
                continue;
            }
            for m in P2_MOVES.iter() {
                let m = *m;
                let newstate = state.rotate(self, m);
                if set.contains(&newstate) {
                    continue;
                }

                set.insert(newstate); // TODO: ayashii
                heap.push((
                    -(dist + 1),
                    newstate,
                    rotates * P2_MOVES_SIZE as u64 + m.to_usize() as u64,
                ));
            }
        }
        Err(())
    }
}
