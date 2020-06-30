mod ctwist;
mod etwist;
mod udslice;

use self::ctwist::{CTwist, COUNT as CTWIST_COUNT};
use self::etwist::{ETwist, COUNT as ETWIST_COUNT};
use self::udslice::{UDSlice, COUNT as UDSLICE_COUNT};

mod flipud;
use self::flipud::*;

use crate::cube;
use crate::solve::util::VecU2;
use cube::{Move, Sym16};

use num_traits::cast::FromPrimitive;
use std::convert::TryInto;
use std::ops::Mul;
use strum::IntoEnumIterator;

#[derive(Deserialize, Serialize)]
pub struct Phase1 {
    prunetable: VecU2, // FLIPUD_COUNT * CTWIST_COUNT
}

pub const MAX_STEPS: usize = 12;

impl Phase1 {
    pub fn new_from_cache<R>(src: R) -> bincode::Result<Self>
    where
        R: std::io::Read,
    {
        bincode::deserialize_from(src)
    }

    pub fn new() -> Self {
        let mut rawtable = vec![!0u8; FLIPUD_COUNT * CTWIST_COUNT];
        {
            let mut queue = std::collections::VecDeque::with_capacity(100); // TODO: update cap

            let solved: Phase1Cube = cube::SOLVED.try_into().unwrap();
            let solved: Phase1Coord = solved.into();
            let solved: PruneCoord = solved.into();

            queue.push_back(solved);
            rawtable[solved.coord()] = 0;

            while let Some(pc) = queue.pop_front() {
                let dis = rawtable[pc.coord()];
                if dis >= MAX_STEPS as u8 {
                    break;
                }

                let cur: Phase1Vec = PruneVec::from(pc).into();

                for s in Sym16::iter() {
                    let cur = s * cur;

                    for m in P2Move::iter() {
                        let cur = m * cur;

                        let t: PruneCoord = PruneVec::from(cur).into();
                        let coord = t.coord();

                        if rawtable[coord] == !0 {
                            rawtable[coord] = dis + 1;
                            queue.push_back(t);
                        }
                    }
                }
            }

            let mut t = [0; MAX_STEPS + 1];
            for &v in rawtable.iter() {
                if v == !0 {
                    continue;
                }
                t[v as usize] += 1;
            }
            println!("{:?}", t);
        }

        let mut p1 = Phase1 {
            prunetable: VecU2::new(3, FLIPUD_COUNT * CTWIST_COUNT),
        };

        for (i, &v) in rawtable.iter().enumerate() {
            if v != !0 {
                p1.prunetable.set(i, v as u8 % 3);
            }
        }

        p1
    }
}

#[derive(Clone)]
struct Phase1Cube(cube::CubieLevel);

impl std::convert::TryFrom<cube::CubieLevel> for Phase1Cube {
    type Error = ();
    fn try_from(src: cube::CubieLevel) -> Result<Self, Self::Error> {
        Ok(Phase1Cube(src))
    }
}

impl Phase1Cube {
    fn solve(&self, p1: &Phase1) -> Vec<Move> {
        use std::collections::{BinaryHeap, HashSet};

        fn recover_rotates(dist: usize, rotates: u64) -> Vec<Move> {
            let mut rotates = rotates;
            let mut res = vec![Move::U1; dist];

            for i in 0..dist {
                let mv = Move::from_usize(rotates as usize % MOVE_COUNT).unwrap();
                res[dist - 1 - i] = mv.into();
                rotates /= MOVE_COUNT as u64;
            }
            res
        }

        let solved: Phase1Cube = cube::SOLVED.try_into().unwrap();
        let solved: Phase1Coord = solved.into();

        let src: Phase1Coord = self.clone().into();

        fn cur_lowerbound(p1: &Phase1, src: Phase1Coord) -> u8 {
            let src: PruneCoord = src.into();

            let mut set = HashSet::new();
            let mut heap = BinaryHeap::new();
            heap.push((-0i8, src));
            set.insert(src);

            let solved: Phase1Cube = cube::SOLVED.try_into().unwrap();
            let solved: Phase1Coord = solved.into();
            let goalpc = PruneCoord::from(solved);

            loop {
                let (dist, pc) = heap.pop().unwrap();
                let dist = -dist;
                if pc == goalpc {
                    return dist as u8;
                }

                let cur: Phase1Vec = PruneVec::from(pc).into();
                let dec = (p1.prunetable.get(pc.coord()) + 2) % 3;

                for s in Sym16::iter() {
                    let cur = s * cur;

                    for m in Move::iter() {
                        let cur = m * cur;

                        let npc: PruneCoord = PruneVec::from(cur).into();

                        if set.contains(&npc) {
                            continue;
                        }

                        if p1.prunetable.get(npc.coord()) == dec {
                            heap.push((-(dist + 1), npc));
                            set.insert(npc);
                        }
                    }
                }
            }
        }

        let lb = cur_lowerbound(p1, src);
        println!("lb: {}", lb);

        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();

        heap.push((-0i8, src, lb, 0));
        set.insert(src);

        while let Some((dist, state, lb, rotates)) = heap.pop() {
            let dist = -dist;
            let dist = dist as u8;

            if state == solved {
                println!("dist: {}", dist);
                return recover_rotates(dist as usize, rotates);
            }

            for m in Move::iter() {
                let nstate = m * state;

                let nlb = match p1.prunetable.get(PruneCoord::from(nstate).coord()) {
                    i if i == lb % 3 => lb,
                    i if i == (lb + 1) % 3 => lb + 1,
                    i if i == (lb + 2) % 3 => lb - 1,
                    _ => unreachable!(),
                };

                if dist + 1 + nlb >= MAX_STEPS as u8 {
                    continue;
                }

                if set.contains(&nstate) {
                    continue;
                }
                set.insert(nstate);

                heap.push((
                    -(dist as i8 + 1),
                    nstate,
                    nlb,
                    rotates * MOVE_COUNT as u64 + m as u64,
                ));
            }
        }

        unreachable!();
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Phase1Vec {
    ctwi: CTwist,
    etwi: ETwist,
    uds: UDSlice,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Phase1Coord(u32, u16); // TODO

impl From<Phase1Coord> for Phase1Vec {
    fn from(src: Phase1Coord) -> Self {
        let (ctwi, uds) = (src.0 / UDSLICE_COUNT as u32, src.0 % UDSLICE_COUNT as u32);
        Phase1Vec {
            ctwi: CTwist(ctwi as u16),
            etwi: ETwist(src.1),
            uds: UDSlice(uds as u16),
        }
    }
}
impl From<Phase1Vec> for Phase1Coord {
    fn from(src: Phase1Vec) -> Self {
        Phase1Coord(
            src.cp.0 as u32 * UDSLICE_COUNT as u32 + src.uds.0 as u32,
            src.ep.0 as u32,
        )
    }
}

impl<T: Into<Phase1Cube>> From<T> for Phase1Coord {
    fn from(src: T) -> Self {
        let src: Phase1Cube = src.into();
        let (v1, v2, v3): (CPerm, EPerm, UDSlice) =
            ((src).0.into(), (src).0.into(), (src).0.into());
        Phase2Vec {
            ctwi: v1,
            etwi: v2,
            uds: v3,
        }
        .into()
    }
}

impl Mul<Phase1Vec> for Move {
    type Output = Phase1Vec;
    fn mul(self, rhs: Phase1Vec) -> Self::Output {
        Phase1Vec {
            ctwi: self * rhs.ctwi,
            etwi: self * rhs.etwi,
            uds: self * rhs.uds,
        }
    }
}
impl Mul<Phase1Coord> for Move {
    type Output = Phase1Coord;
    fn mul(self, rhs: Phase1Coord) -> Self::Output {
        let v: Phase1Vec = rhs.into();
        (self * v).into()
    }
}

impl Mul<Phase1Vec> for Sym16 {
    type Output = Phase1Vec;
    fn mul(self, rhs: Phase1Vec) -> Self::Output {
        Phase1Vec {
            ctwi: self * rhs.ctwi,
            etwi: self * rhs.etwi,
            uds: self * rhs.uds,
        }
    }
}
impl Mul<Phase1Coord> for Sym16 {
    type Output = Phase1Coord;
    fn mul(self, rhs: Phase1Coord) -> Self::Output {
        let v: Phase1Vec = rhs.into();
        (self * v).into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct PruneVec {
    flipud: FlipUD,
    ctwi: CTwist,
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
        PruneCoord((src.coset.0 as usize * CTWIST_COUNT + src.ep.0 as usize) as u32)
    }
}
impl From<PruneCoord> for PruneVec {
    fn from(src: PruneCoord) -> Self {
        let (coset, ep) = (src.0 as usize / CTWIST_COUNT, src.0 as usize % CTWIST_COUNT);
        PruneVec {
            coset: CPermCoset(coset as u16),
            ep: EPerm(ep as u16),
        }
    }
}
impl From<Phase1Vec> for PruneCoord {
    fn from(src: Phase1Vec) -> Self {
        PruneVec::from(src).into()
    }
}

impl From<Phase1Vec> for Sym16 {
    //FIXME
    fn from(src: Phase1Vec) -> Self {
        lazy_static! {
            static ref MEMO: Vec<Sym16> = {
                let mut memo = vec![Sym16(!0); ETWIST_COUNT * UDSLICE_COUNT];
                for et in ETwist::iter() {
                    for uds in UDSlice::iter() {
                        let coset = CPerm::from(CPermCoset::from(cp));
                        memo[cp.0 as usize] = Sym16::iter().find(|&s| coset == s * cp).unwrap();
                    }
                }
                memo
            };
        }
        MEMO[src.0 as usize]
    }
}

impl From<Phase1Vec> for PruneVec {
    fn from(src: Phase1Vec) -> Self {
        let coset: FlipUD = src.into();
        let s = Sym16::from(src);

        PruneVec {
            coset: coset,
            ctwi: s * src.ctwi,
        }
    }
}

impl From<Phase1Coord> for PruneVec {
    fn from(src: Phase1Coord) -> Self {
        Phase1Vec::from(src).into()
    }
}

impl From<PruneVec> for Phase1Vec {
    // representation
    fn from(src: PruneVec) -> Self {
        Phase1Vec {
            ctwi: CTwi(0),
            etwi: src.coset.into(),
            uds: src.coset.into(),
        }
    }
}

impl From<PruneCoord> for Phase1Coord {
    // representation
    fn from(src: PruneCoord) -> Self {
        Phase1Vec::from(PruneVec::from(src)).into()
    }
}

impl From<Phase1Coord> for PruneCoord {
    fn from(src: Phase1Coord) -> Self {
        PruneVec::from(src).into()
    }
}

#[test]
fn rotate_test() {
    use super::*;

    let solved: Phase1Cube = cube::SOLVED.try_into().unwrap();
    let solved: Phase1Coord = solved.into();

    for m in Move::iter() {
        let cube = m * cube::SOLVED;
        let cube: Phase1Cube = cube.try_into().unwrap();
        let v1: Phase1Coord = cube.into();

        let v2 = m * solved;
        assert_eq!(v1, v2, "move {:?}", m);
    }

    use Move::*;
    let cube = F2 * (U1 * (L2 * cube::SOLVED));
    let cube: Phase1Cube = cube.try_into().unwrap();
    let cube: Phase1Coord = cube.into();
    let cube = L2 * (U3 * (F2 * cube));
    assert_eq!(cube, solved);
}

impl super::Phase for Phase1 {
    type Error = ();

    fn solve(&self, src: &crate::RubikCube) -> Result<Vec<Move>, Self::Error> {
        let src = (*src).0;
        let src: Phase1Cube = src.try_into()?;

        Ok(src.solve(&self))
    }
}
