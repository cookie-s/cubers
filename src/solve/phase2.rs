use super::cube;
use num_traits::cast::FromPrimitive;
use strum::IntoEnumIterator;

const FACT4: usize = 4 * 3 * 2 * 1;
const FACT8: usize = 8 * 7 * 6 * 5 * 4 * 3 * 2 * 1;

#[derive(Debug, Copy, Clone, EnumCount, EnumIter, FromPrimitive)]
#[cfg_attr(rustfmt, rustfmt_skip)]
enum P2Move {
    U1, U2, U3,
    D1, D2, D3,
    F2,
    B2,
    L2,
    R2,
}

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

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
    for m in P2Move::iter() {
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
    for m in P2Move::iter() {
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
    for m in P2Move::iter() {
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
    cperm_movetable: Vec<CPerm>,     // FACT8 * P2MOVE_COUNT
    eperm_movetable: Vec<EPerm>,     // FACT8 * P2MOVE_COUNT
    udslice_movetable: Vec<UDSlice>, // FACT4 * P2MOVE_COUNT
    sym_movetable: Vec<u16>,         // 2768 * P2MOVE_COUNT
    cperm_to_sym: Vec<u16>,          // FACT8
    prunetable: Vec<u8>,             // 2768 * FACT8
}

impl Phase2 {
    pub fn new() -> &'static Self {
        lazy_static! {
                static ref P2: Phase2 = {
                    let mut p2 = Phase2 {
                        cperm_movetable: vec![CPerm(!0); FACT8 * P2MOVE_COUNT],
                        eperm_movetable: vec![EPerm(!0); FACT8 * P2MOVE_COUNT],
                        udslice_movetable: vec![UDSlice(!0); FACT4 * P2MOVE_COUNT],
                        sym_movetable: vec![!0; 2768 * P2MOVE_COUNT],
                        cperm_to_sym: vec![!0; FACT8],
                        prunetable: vec![!0; 2768 * FACT8],
                    };

                    // cperm
                    for i in 0..FACT8 {
                        let cube: cube::CubieLevel = CPerm(i as u16).into();
        for m in P2Move::iter() {
                            let v: CPerm = (m * cube).into();
                            p2.cperm_movetable[i * P2MOVE_COUNT + (m as usize)] = v;
                        }
                    }

                    // eperm
                    for i in 0..FACT8 {
                        let cube: cube::CubieLevel = EPerm(i as u16).into();
        for m in P2Move::iter() {
                            let v: EPerm = (m * cube).into();
                            p2.eperm_movetable[i * P2MOVE_COUNT + (m as usize)] = v;
                        }
                    }

                    // udslice
                    for i in 0..FACT4 {
                        let cube: cube::CubieLevel = UDSlice(i as u8).into();
        for m in P2Move::iter() {
                            let v: UDSlice = (m * cube).into();
                            p2.udslice_movetable[i * P2MOVE_COUNT + (m as usize)] = v;
                        }
                    }

                    let mut sym_to_cperm = [CPerm(!0); 2768];
                    {
                        let mut cnt = 0;
                        for i in 0..FACT8 {
                            let cp = CPerm(i as u16);
                            let cube: cube::CubieLevel = cp.into();
                            let mut found = None;

                            for s in cube::Sym16::iter() {
                                let cube = s * cube;
                                let v: CPerm = cube.into();

                                if p2.cperm_to_sym[v.0 as usize] != !0 {
                                    found = Some(p2.cperm_to_sym[v.0 as usize]);
                                    break;
                                }
                            }
                            if found == None {
                                sym_to_cperm[cnt] = cp;
                                found = Some(cnt as u16);
                                cnt += 1;
                            }
                            p2.cperm_to_sym[i] = found.unwrap() as u16;
                        }
                    }

        for m in P2Move::iter() {
                        let cp: CPerm = (m * cube::SOLVED).into();
                        let ep: EPerm = (m * cube::SOLVED).into();
                        println!(
                            "{:?} {:?} {:?} {:?}",
                            m, cp, p2.cperm_to_sym[cp.0 as usize], ep
                        );
                    }

                    {
                        for (i, &v) in sym_to_cperm.iter().enumerate() {
                            let cp = v.0 as usize;

                            for m in 0..P2MOVE_COUNT {
                                let ncp = p2.cperm_movetable[cp * P2MOVE_COUNT + m].0 as usize;
                                let j = p2.cperm_to_sym[ncp];

                                p2.sym_movetable[i * P2MOVE_COUNT + m] = j;
                            }
                        }
                    }

                    {
                        let mut queue = std::collections::VecDeque::new();

                        let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
                        let solved: Phase2Vec = solved.into();

                        queue.push_back((0, solved));

                        while let Some((dis, state)) = queue.pop_front() {
                            let pc = state.prune_coord(&p2);
                            if p2.prunetable[pc] <= dis {
                                continue;
                            }
                            p2.prunetable[pc] = dis;

        for m in P2Move::iter() {
                                let nextstate = state.rotate(&p2, m);
                                let nextpc = nextstate.prune_coord(&p2);
                                if p2.prunetable[nextpc] > dis + 1 {
                                    queue.push_back(((dis + 1), nextstate));
                                }
                            }
                        }

                        let mut t = [0; 25];
                        for &v in p2.prunetable.iter() {
                            if v == !0 {
                                continue;
                            }
                            t[v as usize] += 1;
                        }
                        println!("{:?}", t);
                    }

                    println!("init done");
                    p2
                };
            };

        &P2
    }
}

use std::convert::{TryFrom, TryInto};
struct Phase2Cube(cube::CubieLevel);
impl TryFrom<cube::CubieLevel> for Phase2Cube {
    type Error = ();
    fn try_from(src: cube::CubieLevel) -> Result<Self, Self::Error> {
        fn is_phase2(cube: cube::CubieLevel) -> bool {
            cube.0.iter().all(|c| c.o == 0)
                && (cube).1.iter().all(|e| e.o == 0)
                && (cube)
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
struct Phase2Vec(u32, u16);
impl<T: Into<Phase2Cube>> From<T> for Phase2Vec {
    fn from(src: T) -> Phase2Vec {
        let src: Phase2Cube = src.into();
        let (v1, v2, v3): (CPerm, EPerm, UDSlice) =
            ((src).0.into(), (src).0.into(), (src).0.into());
        Self::combine(v1, v2, v3)
    }
}

impl Phase2Vec {
    fn split(self) -> (CPerm, EPerm, UDSlice) {
        let (cp, uds) = (self.0 / FACT4 as u32, self.0 % FACT4 as u32);
        (CPerm(cp as u16), EPerm(self.1), UDSlice(uds as u8))
    }

    fn combine(cp: CPerm, ep: EPerm, uds: UDSlice) -> Self {
        Phase2Vec(cp.0 as u32 * FACT4 as u32 + uds.0 as u32, ep.0 as u16)
    }

    fn rotate(self, p2: &Phase2, m: P2Move) -> Self {
        let (cp, ep, uds) = self.split();
        let (cp, ep, uds) = (cp.0 as usize, ep.0 as usize, uds.0 as usize);

        let v1 = p2.cperm_movetable[cp * P2MOVE_COUNT + m as usize];
        let v2 = p2.eperm_movetable[ep * P2MOVE_COUNT + m as usize];
        let v3 = p2.udslice_movetable[uds * P2MOVE_COUNT + m as usize];

        Self::combine(v1, v2, v3)
    }

    fn prune_coord(self, p2: &Phase2) -> usize {
        let cp = self.split().0;
        let idx = p2.cperm_to_sym[cp.0 as usize];
        (idx as usize * FACT8) + self.1 as usize
    }
}

#[test]
fn rotate_test() {
    use super::*;
    let p2 = Phase2::new();

    let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
    let solved: Phase2Vec = solved.into();

    for m in P2Move::iter() {
        let cube = m * cube::SOLVED;
        let cube: Phase2Cube = cube.try_into().unwrap();
        let v1: Phase2Vec = cube.into();

        let v2 = solved.rotate(&p2, m);
        assert_eq!(v1.split(), v2.split(), "move {:?}", m);
    }

    use P2Move::*;
    let cube = F2 * (U1 * (L2 * cube::SOLVED));
    let cube: Phase2Cube = cube.try_into().unwrap();
    let cube: Phase2Vec = cube.into();
    let cube = cube.rotate(&p2, F2).rotate(&p2, U3).rotate(&p2, L2);
    assert_eq!(cube.split(), solved.split());
}

impl super::Phase for Phase2 {
    type Error = ();

    fn solve(&self, src: &crate::RubikCube) -> Result<Vec<cube::Move>, Self::Error> {
        use std::collections::{BinaryHeap, HashSet};

        fn recover_rotates(dist: usize, rotates: u64) -> Vec<cube::Move> {
            let mut rotates = rotates;
            let mut res = vec![cube::Move::U1; dist];

            for i in 0..dist {
                let p2move = P2Move::from_usize(rotates as usize % 10).unwrap();
                res[dist - 1 - i] = p2move.into();
                rotates /= 10;
            }
            res
        }

        let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
        let solved: Phase2Vec = solved.into();

        let src = (*src).0;
        let src: Phase2Cube = src.try_into()?;
        let src: Phase2Vec = src.into();

        fn cur_lowerbound(p2: &Phase2, src: Phase2Vec) -> u8 {
            let mut heap = BinaryHeap::new();
            heap.push((-0, src.prune_coord(p2)));
            let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
            let solved: Phase2Vec = solved.into();
            let goalpc = solved.prune_coord(p2);
            loop {
                let (dist, pc) = heap.pop().unwrap();
                let dist = -dist;
                if pc == goalpc {
                    return dist as u8;
                }

                // FIXME
                let ep = pc % FACT8;
                let i = pc / FACT8;

                for m in P2Move::iter() {
                    // FIXME
                    let nep = p2.eperm_movetable[ep * P2MOVE_COUNT + (m as usize)].0 as usize;
                    let j = p2.sym_movetable[i * P2MOVE_COUNT + (m as usize)] as usize;

                    let npc = j * FACT8 + nep;
                    if p2.prunetable[npc] == p2.prunetable[pc] - 1 {
                        println!("{}", p2.prunetable[npc]);
                        heap.push((-(dist + 1), npc));
                    }
                }
            }
        }

        let lb = cur_lowerbound(&self, src);
        println!("{}", lb);

        const MAX_STEPS: i8 = 18;
        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();
        heap.push((-0i8, src, lb, 0));
        set.insert(src);

        while let Some((dist, state, lb, rotates)) = heap.pop() {
            let dist = -dist;

            if state == solved {
                println!("{}", dist);
                return Ok(recover_rotates(dist as usize, rotates));
            }
            if dist + lb as i8 > MAX_STEPS {
                continue;
            }
            for m in P2Move::iter() {
                let nstate = state.rotate(self, m);
                if set.contains(&nstate) {
                    continue;
                }
                set.insert(nstate); // TODO: ayashii

                let nlb = self.prunetable[nstate.prune_coord(&self)];

                heap.push((
                    -(dist + 1) as i8,
                    nstate,
                    nlb,
                    rotates * P2MOVE_COUNT as u64 + m as u64,
                ));
            }
        }
        Err(())
    }
}
