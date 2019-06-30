mod cperm;
mod eperm;
mod udslice;

use self::cperm::*;
use self::eperm::*;
use self::udslice::*;

mod cpermcoset;
use self::cpermcoset::*;

mod p2move;
use self::p2move::*;

use crate::cube;
use crate::solve::util::VecU2;
use cube::{Move, Sym16};
use num_traits::cast::FromPrimitive;
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Deserialize, Serialize)]
pub struct Phase2 {
    prunetable: VecU2, // 2768 * EPERM_COUNT
}

impl Phase2 {
    pub fn new_from_cache<R>(src: R) -> bincode::Result<Self>
    where
        R: std::io::Read,
    {
        bincode::deserialize_from(src)
    }

    pub fn new() -> Self {
        let mut rawtable = vec![!0u8; CPERMCOSET_COUNT * EPERM_COUNT];
        {
            let mut queue = std::collections::VecDeque::with_capacity(37144996);

            let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
            let solved: Phase2Coord = solved.into();
            let solved: PruneCoord = solved.into();

            queue.push_back(solved);
            rawtable[solved.coord()] = 0;

            while let Some(pc) = queue.pop_front() {
                let dis = rawtable[pc.coord()];

                if dis >= 18 {
                    break;
                }

                let cur: Phase2Coord = pc.into();

                for s in Sym16::iter() {
                    let cur = s * cur;

                    let t: PruneCoord = cur.into();
                    if rawtable[t.coord()] < dis {
                        continue;
                    }

                    for m in P2Move::iter() {
                        let cur = m * cur;

                        let t: PruneCoord = cur.into();
                        let coord = t.coord();

                        if rawtable[coord] > dis + 1 {
                            rawtable[coord] = dis + 1;
                            queue.push_back(t);
                        }
                    }
                }
            }

            let mut t = [0; 18 + 1];
            for &v in rawtable.iter() {
                if v == !0 {
                    continue;
                }
                t[v as usize] += 1;
            }
            println!("{:?}", t);
        }

        let mut p2 = Phase2 {
            prunetable: VecU2::new(3, CPERMCOSET_COUNT * EPERM_COUNT),
        };

        for (i, &v) in rawtable.iter().enumerate() {
            if v != !0 {
                p2.prunetable.set(i, v as u8 % 3);
            }
        }

        p2
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Phase2Vec {
    cp: CPerm,
    ep: EPerm,
    uds: UDSlice,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Phase2Coord(u32, u16);

impl From<Phase2Coord> for Phase2Vec {
    fn from(src: Phase2Coord) -> Self {
        let (cp, uds) = (src.0 / UDSLICE_COUNT as u32, src.0 % UDSLICE_COUNT as u32);
        Phase2Vec {
            cp: CPerm(cp as u16),
            ep: EPerm(src.1),
            uds: UDSlice(uds as u8),
        }
    }
}
impl From<Phase2Vec> for Phase2Coord {
    fn from(src: Phase2Vec) -> Self {
        Phase2Coord(
            src.cp.0 as u32 * UDSLICE_COUNT as u32 + src.uds.0 as u32,
            src.ep.0 as u16,
        )
    }
}

impl<T: Into<Phase2Cube>> From<T> for Phase2Coord {
    fn from(src: T) -> Self {
        let src: Phase2Cube = src.into();
        let (v1, v2, v3): (CPerm, EPerm, UDSlice) =
            ((src).0.into(), (src).0.into(), (src).0.into());
        Phase2Vec {
            cp: v1,
            ep: v2,
            uds: v3,
        }
        .into()
    }
}

impl Mul<Phase2Vec> for P2Move {
    type Output = Phase2Vec;
    fn mul(self, rhs: Phase2Vec) -> Self::Output {
        Phase2Vec {
            cp: self * rhs.cp,
            ep: self * rhs.ep,
            uds: self * rhs.uds,
        }
    }
}
impl Mul<Phase2Coord> for P2Move {
    type Output = Phase2Coord;
    fn mul(self, rhs: Phase2Coord) -> Self::Output {
        let v: Phase2Vec = rhs.into();
        (self * v).into()
    }
}

impl Mul<Phase2Vec> for Sym16 {
    type Output = Phase2Vec;
    fn mul(self, rhs: Phase2Vec) -> Self::Output {
        Phase2Vec {
            cp: self * rhs.cp,
            ep: self * rhs.ep,
            uds: self * rhs.uds,
        }
    }
}
impl Mul<Phase2Coord> for Sym16 {
    type Output = Phase2Coord;
    fn mul(self, rhs: Phase2Coord) -> Self::Output {
        let v: Phase2Vec = rhs.into();
        (self * v).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct PruneVec {
    coset: CPermCoset,
    ep: EPerm,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct PruneCoord(u32);

impl PruneCoord {
    fn coord(&self) -> usize {
        self.0 as usize
    }
}

impl From<PruneVec> for PruneCoord {
    fn from(src: PruneVec) -> Self {
        PruneCoord((src.coset.0 as usize * EPERM_COUNT + src.ep.0 as usize) as u32)
    }
}
impl From<PruneCoord> for PruneVec {
    fn from(src: PruneCoord) -> Self {
        let (coset, ep) = (src.0 as usize / EPERM_COUNT, src.0 as usize % EPERM_COUNT);
        PruneVec {
            coset: CPermCoset(coset as u16),
            ep: EPerm(ep as u16),
        }
    }
}

impl Mul<PruneVec> for P2Move {
    type Output = PruneVec;
    fn mul(self, rhs: PruneVec) -> Self::Output {
        let cp = self * CPerm::from(rhs.coset);
        let coset: CPermCoset = cp.into();
        let s = Sym16::from(cp);

        PruneVec {
            coset: coset,
            ep: s * (self * rhs.ep),
        }
    }
}

impl Mul<PruneCoord> for P2Move {
    type Output = PruneCoord;
    fn mul(self, rhs: PruneCoord) -> Self::Output {
        (self * PruneVec::from(rhs)).into()
    }
}

impl From<CPerm> for Sym16 {
    //FIXME
    fn from(src: CPerm) -> Self {
        lazy_static! {
            static ref MEMO: Vec<Sym16> = {
                let mut memo = vec![Sym16(!0); CPERM_COUNT];
                for cp in CPerm::iter() {
                    let coset = CPerm::from(CPermCoset::from(cp));
                    memo[cp.0 as usize] = Sym16::iter().find(|&s| coset == s * cp).unwrap();
                }
                memo
            };
        }
        MEMO[src.0 as usize]
    }
}

impl From<Phase2Coord> for PruneVec {
    fn from(src: Phase2Coord) -> Self {
        let src: Phase2Vec = src.into();
        let coset: CPermCoset = src.cp.into();
        let s = Sym16::from(src.cp);

        PruneVec {
            coset: coset,
            ep: s * src.ep,
        }
    }
}
impl From<PruneCoord> for Phase2Coord {
    // representation
    fn from(src: PruneCoord) -> Self {
        let src: PruneVec = src.into();
        Phase2Vec {
            cp: src.coset.into(),
            ep: src.ep.into(),
            uds: UDSlice(0),
        }
        .into()
    }
}

#[test]
fn sym_move_one() {
    use P2Move::*;

    let solved = PruneVec::from(Phase2Coord::from(
        Phase2Cube::try_from(cube::SOLVED).unwrap(),
    ));

    for &m in &[U1, U3, D1, D3] {
        assert_eq!(m * solved, U1 * solved, "{:?}", m);
    }
    for &m in &[U2, D2] {
        assert_eq!(m * solved, U2 * solved, "{:?}", m);
    }
    for &m in &[F2, B2, L2, R2] {
        assert_eq!(m * solved, F2 * solved, "{:?}", m);
    }
}

impl From<Phase2Coord> for PruneCoord {
    fn from(src: Phase2Coord) -> Self {
        PruneVec::from(src).into()
    }
}

#[test]
fn rotate_test() {
    use super::*;

    let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
    let solved: Phase2Coord = solved.into();

    for m in P2Move::iter() {
        let cube = m * cube::SOLVED;
        let cube: Phase2Cube = cube.try_into().unwrap();
        let v1: Phase2Coord = cube.into();

        let v2 = m * solved;
        assert_eq!(v1, v2, "move {:?}", m);
    }

    use P2Move::*;
    let cube = F2 * (U1 * (L2 * cube::SOLVED));
    let cube: Phase2Cube = cube.try_into().unwrap();
    let cube: Phase2Coord = cube.into();
    let cube = L2 * (U3 * (F2 * cube));
    assert_eq!(cube, solved);
}

impl super::Phase for Phase2 {
    type Error = ();

    fn solve(&self, src: &crate::RubikCube) -> Result<Vec<Move>, Self::Error> {
        use std::collections::{BinaryHeap, HashSet};

        fn recover_rotates(dist: usize, rotates: u64) -> Vec<Move> {
            let mut rotates = rotates;
            let mut res = vec![Move::U1; dist];

            for i in 0..dist {
                let p2move = P2Move::from_usize(rotates as usize % P2MOVE_COUNT).unwrap();
                res[dist - 1 - i] = p2move.into();
                rotates /= P2MOVE_COUNT as u64;
            }
            res
        }

        let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
        let solved: Phase2Coord = solved.into();

        let src = (*src).0;
        let src: Phase2Cube = src.try_into()?;
        let src: Phase2Coord = src.into();

        fn cur_lowerbound(p2: &Phase2, src: Phase2Coord) -> u8 {
            let src: PruneCoord = src.into();

            let mut set = HashSet::new();
            let mut heap = BinaryHeap::new();
            heap.push((-0, src));
            set.insert(src);

            let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
            let solved: Phase2Coord = solved.into();
            let goalpc = PruneCoord::from(solved);

            loop {
                let (dist, pc) = heap.pop().unwrap();
                let dist = -dist;
                if pc == goalpc {
                    return dist as u8;
                }

                let cur: Phase2Coord = pc.into();

                for s in Sym16::iter() {
                    let cur = s * cur;

                    for m in P2Move::iter() {
                        let npc: PruneCoord = (m * cur).into();

                        if set.contains(&npc) {
                            continue;
                        }

                        if (3 + p2.prunetable.get(npc.coord()) - p2.prunetable.get(pc.coord())) % 3
                            == 3 - 1
                        {
                            heap.push((-(dist + 1), npc));
                            set.insert(npc);
                        }
                    }
                }
            }
        }

        let lb = cur_lowerbound(&self, src);
        println!("lb: {}", lb);

        const MAX_STEPS: u8 = 18;

        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();

        heap.push((-0i8, src, lb, 0));
        set.insert(src);

        while let Some((dist, state, lb, rotates)) = heap.pop() {
            let dist = -dist;
            let dist = dist as u8;

            if state == solved {
                println!("dist: {}", dist);
                return Ok(recover_rotates(dist as usize, rotates));
            }

            if dist as u8 + lb >= MAX_STEPS {
                continue;
            }

            for m in P2Move::iter() {
                let nstate = m * state;
                if set.contains(&nstate) {
                    continue;
                }

                set.insert(nstate);

                let nlb = match self.prunetable.get(PruneCoord::from(nstate).coord()) {
                    i if i == lb % 3 => lb,
                    i if i == (lb + 1) % 3 => lb + 1,
                    i if i == (lb + 2) % 3 => lb - 1,
                    _ => unreachable!(),
                };

                heap.push((
                    -(dist as i8 + 1),
                    nstate,
                    nlb,
                    rotates * P2MOVE_COUNT as u64 + m as u64,
                ));
            }
        }

        Err(())
    }
}
