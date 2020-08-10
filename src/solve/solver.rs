use crate::cube;
use cube::Move;
use num_traits::{FromPrimitive, ToPrimitive};
use std::convert::{TryFrom, TryInto};

pub trait Solver {
    type Error;
    fn solve(&self, cubie: cube::CubieLevel) -> Result<Vec<Move>, Self::Error>;
}

pub trait Kociemba {
    type S;
    type M;
    type Cube;
    type Coord;
    type PruneCoord;

    fn prunetable_get(&self, idx: usize) -> usize;

    fn SOLVED_COORD() -> Self::Coord; // const SOLVED_COORD: Self::Coord;
    const MAX_STEPS: usize;

    fn solve(
        &self,
        cubie: cube::CubieLevel,
    ) -> Result<Vec<Move>, <Self::Cube as std::convert::TryFrom<cube::CubieLevel>>::Error>
    where
        Self::Cube: std::convert::TryFrom<cube::CubieLevel>,
        Self::Coord: std::convert::From<Self::PruneCoord>
            + std::convert::From<Self::Cube>
            + std::cmp::Eq
            + std::cmp::Ord
            + std::hash::Hash
            + Copy,
        Self::PruneCoord:
            std::convert::From<Self::Coord> + std::cmp::Eq + std::cmp::Ord + std::hash::Hash + Copy,
        usize: std::convert::From<Self::PruneCoord>,
        Self::S: StaticExactSizeIterator + std::ops::Mul<Self::Coord, Output = Self::Coord> + Copy,
        Self::M: StaticExactSizeIterator
            + num_traits::FromPrimitive
            + num_traits::ToPrimitive
            + std::ops::Mul<Self::Coord, Output = Self::Coord>
            + Copy,
        Move: std::convert::From<Self::M>,
    {
        let cube: Self::Cube = cubie.try_into()?;

        use std::collections::{BinaryHeap, HashSet};

        let solved: Self::Coord = Self::SOLVED_COORD();
        let src: Self::Coord = cube.into();

        let lb = (|src: Self::Coord| -> u8 {
            let src: Self::PruneCoord = src.into();

            let mut set = HashSet::new();
            let mut heap = BinaryHeap::new();
            heap.push((-0i8, src));
            set.insert(src);

            let solved: Self::Coord = Self::SOLVED_COORD();
            let goalpc: Self::PruneCoord = solved.into();

            while let Some((dist, pc)) = heap.pop() {
                let dist = -dist;
                if pc == goalpc {
                    return dist as u8;
                }

                let cur: Self::Coord = pc.into();
                let dec = (self.prunetable_get(pc.into()) + 2) % 3;

                for s in Self::S::iter() {
                    let cur = s * cur;

                    for m in Self::M::iter() {
                        let cur = m * cur;

                        let npc: Self::PruneCoord = cur.into();

                        if set.contains(&npc) {
                            continue;
                        }

                        if self.prunetable_get(npc.into()) == dec {
                            heap.push((-(dist + 1), npc));
                            set.insert(npc);
                        }
                    }
                }
            }
            unreachable!("broken prunetable")
        })(src);

        let mut heap = BinaryHeap::new();
        let mut set = HashSet::new();

        heap.push((-0i8, src, lb, 0));
        set.insert(src);

        while let Some((dist, state, lb, rotates)) = heap.pop() {
            let dist = -dist;
            let dist = dist as u8;

            if state == solved {
                let dist = dist as usize;
                let rotates = {
                    let mut rotates = rotates;
                    let mut res = vec![Move::U1; dist];

                    for i in 0..dist {
                        let mv = Self::M::from_usize(rotates as usize % Self::M::COUNT).unwrap();
                        res[dist - 1 - i] = mv.into();
                        rotates /= Self::M::COUNT as u64;
                    }
                    res
                };
                return Ok(rotates);
            }

            for m in Self::M::iter() {
                let nstate = m * state;

                let nlb = match self.prunetable_get(Self::PruneCoord::from(nstate).into()) as u8 {
                    i if i == lb % 3 => lb,
                    i if i == (lb + 1) % 3 => lb + 1,
                    i if i == (lb + 2) % 3 => lb - 1,
                    _ => unreachable!(),
                };

                if dist + 1 + nlb >= Self::MAX_STEPS as u8 {
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
                    rotates * Self::M::COUNT as u64 + m.to_u64().unwrap(),
                ));
            }
        }

        unreachable!();
    }
}

use crate::solve;
use solve::phase2;

use phase2::p2move::*;
use phase2::*;

use crate::solve::util::VecU2;
use cube::Sym16;

#[derive(Deserialize, Serialize)]
pub struct Phase2Solver {
    prunetable: VecU2, // CPERMCOSET_COUNT * EPERM_COUNT
}

impl Kociemba for Phase2Solver {
    type S = Sym16;
    type M = P2Move;
    type Cube = Phase2Cube;
    type Coord = Phase2Coord;
    type PruneCoord = PruneCoord;

    fn prunetable_get(&self, idx: usize) -> usize {
        self.prunetable.get(idx) as usize
    }

    // const SOLVED_COORD: Self::Coord = Self::Cube::try_from(cube::SOLVED).unwrap().into();
    fn SOLVED_COORD() -> Self::Coord {
        lazy_static! {
            static ref RES: Phase2Coord = Phase2Cube::try_from(cube::SOLVED).unwrap().into();
        }
        *RES
    }
    const MAX_STEPS: usize = 18;
}

impl<T: Kociemba> Solver for T
where
    // FIXME duplication
    <T as Kociemba>::Cube: std::convert::TryFrom<cube::CubieLevel>,
    <T as Kociemba>::Cube: std::convert::TryFrom<cube::CubieLevel>,
    <T as Kociemba>::Coord: std::convert::From<<T as Kociemba>::PruneCoord>
        + std::convert::From<<T as Kociemba>::Cube>
        + std::cmp::Eq
        + std::cmp::Ord
        + std::hash::Hash
        + Copy,
    <T as Kociemba>::PruneCoord: std::convert::From<<T as Kociemba>::Coord>
        + std::cmp::Eq
        + std::cmp::Ord
        + std::hash::Hash
        + Copy,
    usize: std::convert::From<<T as Kociemba>::PruneCoord>,
    <T as Kociemba>::S: StaticExactSizeIterator
        + std::ops::Mul<<T as Kociemba>::Coord, Output = <T as Kociemba>::Coord>
        + Copy,
    <T as Kociemba>::M: StaticExactSizeIterator
        + num_traits::FromPrimitive
        + num_traits::ToPrimitive
        + std::ops::Mul<<T as Kociemba>::Coord, Output = <T as Kociemba>::Coord>
        + Copy,
    Move: std::convert::From<<T as Kociemba>::M>,
{
    type Error = <<T as Kociemba>::Cube as std::convert::TryFrom<cube::CubieLevel>>::Error;

    fn solve(&self, cubie: cube::CubieLevel) -> Result<Vec<Move>, Self::Error> {
        self.solve(cubie)
    }
}

impl Phase2Solver {
    pub fn new_from_cache<R>(src: R) -> Result<Self, ()>
    where
        R: std::io::Read,
    {
        use crate::hash::{Digest, DigestWriter, Sha256};
        use crate::tee::TeeReader;

        let mut hasher = Sha256::new();
        let hashwriter = DigestWriter::new(&mut hasher);
        let reader = TeeReader::new(src, hashwriter);
        let result = bincode::deserialize_from(reader).or(Err(()))?;
        let hash = hasher.finalize();
        if hash[..] == hex!("562673e1f32373e41d653ec89967d5367924388812ca5f9a3245e2ec9be4f02c")[..]
        {
            return Ok(result);
        }
        Err(())
    }

    pub fn new() -> Self {
        let mut rawtable = vec![!0u8; CPERMCOSET_COUNT * EPERM_COUNT];
        {
            let mut queue = std::collections::VecDeque::with_capacity(37144996);

            let solved: Phase2Coord = Self::SOLVED_COORD();
            let solved: PruneCoord = solved.into();

            queue.push_back(solved);
            rawtable[usize::from(solved)] = 0;

            while let Some(pc) = queue.pop_front() {
                let dis: u8 = rawtable[usize::from(pc)];
                if dis >= MAX_STEPS as u8 {
                    break;
                }

                let cur: Phase2Vec = PruneVec::from(pc).into();

                for s in Sym16::iter() {
                    let cur = s * cur;

                    for m in P2Move::iter() {
                        let cur = m * cur;

                        let t: PruneCoord = PruneVec::from(cur).into();
                        let coord: usize = t.into();

                        if rawtable[coord] == !0 {
                            rawtable[coord] = dis + 1;
                            queue.push_back(t);
                        }
                    }
                }
            }
        }

        let mut p2 = Self {
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

pub trait StaticExactSizeIterator: Sized {
    type Iter: Iterator<Item = Self>;

    const COUNT: usize;
    fn iter() -> Self::Iter;
}

impl StaticExactSizeIterator for Sym16 {
    type Iter = crate::cube::Sym16Iterator;
    const COUNT: usize = crate::cube::SYM16_COUNT;
    fn iter() -> Self::Iter {
        Sym16::iter()
    }
}

impl StaticExactSizeIterator for P2Move {
    type Iter = <Self as crate::strum::IntoEnumIterator>::Iterator;
    const COUNT: usize = P2MOVE_COUNT;
    fn iter() -> Self::Iter {
        use crate::strum::IntoEnumIterator;
        <Self as IntoEnumIterator>::iter()
    }
}
