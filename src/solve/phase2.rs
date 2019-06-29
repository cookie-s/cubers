use super::cube;
use cube::Inv;
use cube::{Move, Sym16, SYM16_COUNT};
use num_traits::cast::FromPrimitive;
use std::ops::Mul;
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

impl From<Move> for P2Move {
    fn from(m: Move) -> P2Move {
        match m {
            Move::U1 => P2Move::U1,
            Move::U2 => P2Move::U2,
            Move::U3 => P2Move::U3,
            Move::D1 => P2Move::D1,
            Move::D2 => P2Move::D2,
            Move::D3 => P2Move::D3,
            Move::F2 => P2Move::F2,
            Move::B2 => P2Move::B2,
            Move::L2 => P2Move::L2,
            Move::R2 => P2Move::R2,
            _ => panic!("invalid argument"),
        }
    }
}
impl From<P2Move> for Move {
    fn from(m: P2Move) -> Move {
        match m {
            P2Move::U1 => Move::U1,
            P2Move::U2 => Move::U2,
            P2Move::U3 => Move::U3,
            P2Move::D1 => Move::D1,
            P2Move::D2 => Move::D2,
            P2Move::D3 => Move::D3,
            P2Move::F2 => Move::F2,
            P2Move::B2 => Move::B2,
            P2Move::L2 => Move::L2,
            P2Move::R2 => Move::R2,
        }
    }
}

impl std::ops::Mul<cube::CubieLevel> for P2Move {
    type Output = cube::CubieLevel;
    fn mul(self, rhs: cube::CubieLevel) -> cube::CubieLevel {
        let m: Move = self.into();
        m * rhs
    }
}
impl Mul<P2Move> for Sym16 {
    type Output = P2Move;
    fn mul(self, rhs: P2Move) -> Self::Output {
        (self * Move::from(rhs)).unwrap().into()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
struct CPerm(u16); // Corner Permutation Coordinate

#[derive(Debug, Copy, Clone)]
struct CPermIterator(u16);
const CPERM_COUNT: usize = FACT8;
impl std::iter::Iterator for CPermIterator {
    type Item = CPerm;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < CPERM_COUNT {
            return Some(CPerm(i));
        }
        None
    }
}
impl CPerm {
    fn iter() -> CPermIterator {
        CPermIterator(0)
    }
}

#[derive(Debug, Copy, Clone)]
struct EPermIterator(u16);
const EPERM_COUNT: usize = FACT8;
impl std::iter::Iterator for EPermIterator {
    type Item = EPerm;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < EPERM_COUNT {
            return Some(EPerm(i));
        }
        None
    }
}
impl EPerm {
    fn iter() -> EPermIterator {
        EPermIterator(0)
    }
}

#[derive(Debug, Copy, Clone)]
struct UDSliceIterator(u8);
const UDSLICE_COUNT: usize = FACT4;
impl std::iter::Iterator for UDSliceIterator {
    type Item = UDSlice;
    fn next(&mut self) -> Option<Self::Item> {
        let i = self.0;
        self.0 += 1;
        if (i as usize) < UDSLICE_COUNT {
            return Some(UDSlice(i));
        }
        None
    }
}
impl UDSlice {
    fn iter() -> UDSliceIterator {
        UDSliceIterator(0)
    }
}
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
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
    prunetable: Vec<u8>, // 2768 * FACT8
}

impl Mul<CPerm> for P2Move {
    type Output = CPerm;
    fn mul(self, rhs: CPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<CPerm> = {
                let mut memo = vec![CPerm(!0); CPERM_COUNT * P2MOVE_COUNT];
                for cp in CPerm::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for m in P2Move::iter() {
                        let v: CPerm = (m * cube).into();
                        memo[cp.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}
impl Mul<EPerm> for P2Move {
    type Output = EPerm;
    fn mul(self, rhs: EPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<EPerm> = {
                let mut memo = vec![EPerm(!0); EPERM_COUNT * P2MOVE_COUNT];
                for ep in EPerm::iter() {
                    let cube: cube::CubieLevel = ep.into();
                    for m in P2Move::iter() {
                        let v: EPerm = (m * cube).into();
                        memo[ep.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}
impl Mul<UDSlice> for P2Move {
    type Output = UDSlice;
    fn mul(self, rhs: UDSlice) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<UDSlice> = {
                let mut memo = vec![UDSlice(!0); UDSLICE_COUNT * P2MOVE_COUNT];
                for uds in UDSlice::iter() {
                    let cube: cube::CubieLevel = uds.into();
                    for m in P2Move::iter() {
                        let v: UDSlice = (m * cube).into();
                        memo[uds.0 as usize * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}

impl Mul<CPerm> for Sym16 {
    type Output = CPerm;
    fn mul(self, rhs: CPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<CPerm> = {
                let mut memo = vec![CPerm(!0); CPERM_COUNT * SYM16_COUNT];
                for cp in CPerm::iter() {
                    let cube: cube::CubieLevel = cp.into();
                    for s in Sym16::iter() {
                        let v: CPerm = (s * cube).into();
                        memo[cp.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}
#[test]
fn mul_cperm_sym() {
    for cp in CPerm::iter() {
        for s in Sym16::iter() {
            assert_eq!(s.inv() * (s * cp), cp);
        }
    }
}
impl Mul<EPerm> for Sym16 {
    type Output = EPerm;
    fn mul(self, rhs: EPerm) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<EPerm> = {
                let mut memo = vec![EPerm(!0); EPERM_COUNT * SYM16_COUNT];
                for ep in EPerm::iter() {
                    let cube: cube::CubieLevel = ep.into();
                    for s in Sym16::iter() {
                        let v: EPerm = (s * cube).into();
                        memo[ep.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}
impl Mul<UDSlice> for Sym16 {
    type Output = UDSlice;
    fn mul(self, rhs: UDSlice) -> Self::Output {
        lazy_static! {
            static ref MEMO: Vec<UDSlice> = {
                let mut memo = vec![UDSlice(!0); UDSLICE_COUNT * SYM16_COUNT];
                for uds in UDSlice::iter() {
                    let cube: cube::CubieLevel = uds.into();
                    for s in Sym16::iter() {
                        let v: UDSlice = (s * cube).into();
                        memo[uds.0 as usize * SYM16_COUNT + (s.0 as usize)] = v;
                    }
                }
                memo
            };
        }
        MEMO[rhs.0 as usize * SYM16_COUNT + self.0 as usize]
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, FromPrimitive)]
struct CPermCoset(u16);
const CPERMCOSET_COUNT: usize = 2768;

impl From<CPerm> for CPermCoset {
    fn from(src: CPerm) -> CPermCoset {
        lazy_static! {
            static ref MEMO: [CPermCoset; FACT8] = {
                let mut memo = [CPermCoset(!0); FACT8];
                let mut cnt = 0;

                for i in 0..FACT8 {
                    let cp = CPerm(i as u16);
                    let cube: cube::CubieLevel = cp.into();
                    let mut found = None;

                    for s in Sym16::iter() {
                        let cube = s * cube;
                        let v: CPerm = cube.into();

                        if memo[v.0 as usize] != CPermCoset(!0) {
                            found = Some(memo[v.0 as usize]);
                            break;
                        }
                    }
                    if found == None {
                        found = Some(CPermCoset(cnt as u16));
                        cnt += 1;
                    }
                    memo[i] = found.unwrap()
                }
                assert_eq!(cnt, CPERMCOSET_COUNT);

                memo
            };
        }
        MEMO[src.0 as usize]
    }
}
impl From<CPermCoset> for CPerm {
    // representation

    fn from(src: CPermCoset) -> CPerm {
        lazy_static! {
            static ref MEMO: [CPerm; CPERMCOSET_COUNT] = {
                let mut memo = [CPerm(!0); CPERMCOSET_COUNT];

                for i in 0..FACT8 {
                    let cp = CPerm(i as u16);
                    if memo[CPermCoset::from(cp).0 as usize] == CPerm(!0) {
                        memo[CPermCoset::from(cp).0 as usize] = cp;
                    }
                }
                memo
            };
        }
        MEMO[src.0 as usize]
    }
}

impl Mul<CPermCoset> for P2Move {
    type Output = CPermCoset;

    fn mul(self, rhs: CPermCoset) -> Self::Output {
        lazy_static! {
            static ref MEMO: [CPermCoset; CPERMCOSET_COUNT * P2MOVE_COUNT] = {
                let mut memo = [CPermCoset(!0); CPERMCOSET_COUNT * P2MOVE_COUNT];
                for i in 0..CPERMCOSET_COUNT {
                    let cp: CPerm = CPermCoset(i as u16).into();

                    for m in P2Move::iter() {
                        let v: CPermCoset = (m * cp).into();
                        memo[i * P2MOVE_COUNT + (m as usize)] = v;
                    }
                }

                memo
            };
        }
        MEMO[rhs.0 as usize * P2MOVE_COUNT + self as usize]
    }
}

impl CPermCoset {
    fn index(self, cp: CPerm) -> Option<Sym16> {
        let src: CPerm = self.into();
        Sym16::iter().find(|&s| s * src == cp)
    }
}

impl Phase2 {
    pub fn new() -> &'static Self {
        lazy_static! {
            static ref P2: Phase2 = {
                let mut p2 = Phase2 {
                    prunetable: vec![!0; 2768 * FACT8],
                };

                for m in P2Move::iter() {
                    let cp: CPerm = (m * cube::SOLVED).into();
                    let ep: EPerm = (m * cube::SOLVED).into();
                    println!("{:?} {:?} {:?} {:?}", m, cp, CPermCoset::from(cp), ep);
                }

                {
                    let mut queue = std::collections::VecDeque::new();

                    let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
                    let solved: Phase2Coord = solved.into();
                    let solved: PruneCoord = solved.into();

                    queue.push_back((0u8, solved));

                    while let Some((dis, pc)) = queue.pop_front() {
                        if dis > 7 {
                            break;
                        }

                        let cur = pc;
                        for s in Sym16::iter() {
                            let cur = s * cur;

                            for m in P2Move::iter() {
                                let cur = m * cur;

                                if p2.prunetable[cur.coord()] > dis + 1 {
                                    p2.prunetable[cur.coord()] = dis + 1;
                                    queue.push_back((dis + 1, cur));
                                }
                            }
                        }
                    }

                    let mut t = [0; 25];
                    let mut sum = 0;
                    for &v in p2.prunetable.iter() {
                        if v == !0 {
                            continue;
                        }
                        sum += 1;
                        if v >= 25 {
                            continue;
                        }
                        t[v as usize] += 1;
                    }
                    println!("{:?} {}", t, sum);
                }

                println!("init done");
                panic!();
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

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Phase2Vec {
    cp: CPerm,
    ep: EPerm,
    uds: UDSlice,
}
impl From<Phase2Coord> for Phase2Vec {
    fn from(src: Phase2Coord) -> Self {
        let (cp, uds) = (src.0 / FACT4 as u32, src.0 % FACT4 as u32);
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
            src.cp.0 as u32 * FACT4 as u32 + src.uds.0 as u32,
            src.ep.0 as u16,
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Phase2Coord(u32, u16);
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
struct PruneCoord(u32); // TODO: usize

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
        let s = Sym16::iter()
            .find(|&s| s * cp == CPerm::from(coset))
            .unwrap();

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
impl Mul<PruneVec> for Sym16 {
    type Output = PruneVec;
    fn mul(self, rhs: PruneVec) -> Self::Output {
        PruneVec {
            coset: rhs.coset,
            ep: rhs.ep,
        }
    }
}
impl Mul<PruneCoord> for Sym16 {
    type Output = PruneCoord;
    fn mul(self, rhs: PruneCoord) -> Self::Output {
        (self * PruneVec::from(rhs)).into()
    }
}
impl From<Phase2Coord> for PruneVec {
    fn from(src: Phase2Coord) -> Self {
        let src: Phase2Vec = src.into();
        let coset: CPermCoset = src.cp.into();
        let s = Sym16::iter()
            .find(|&s| CPerm::from(coset) == s * src.cp)
            .unwrap();

        PruneVec {
            coset: coset,
            ep: s * src.ep,
        }
    }
}

#[test]
fn prune_mul_sym() {
    let cube = P2Move::U1 * (P2Move::F2 * (P2Move::U3 * (P2Move::D1 * cube::SOLVED)));

    for s in Sym16::iter() {
        let c1: PruneVec = Phase2Coord::from(Phase2Cube::try_from(s * cube).unwrap()).into();

        let c2: PruneVec = Phase2Coord::from(Phase2Cube::try_from(cube).unwrap()).into();
        let c2 = s * c2;

        println!(
            "{:?}",
            PruneVec::from(Phase2Coord::from(Phase2Cube::try_from(s * cube).unwrap()))
        );
        assert_eq!(c1, c2, "{:?}", s);
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
    let p2 = Phase2::new();

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
                let p2move = P2Move::from_usize(rotates as usize % 10).unwrap();
                res[dist - 1 - i] = p2move.into();
                rotates /= 10;
            }
            res
        }

        let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
        let solved: Phase2Coord = solved.into();

        let src = (*src).0;
        let src: Phase2Cube = src.try_into()?;
        let src: Phase2Coord = src.into();

        fn cur_lowerbound(p2: &Phase2, src: Phase2Coord) -> u8 {
            return 0;
            // let mut heap = BinaryHeap::new();
            // heap.push((-0, PruneCoord::from(self)));
            // let solved: Phase2Cube = cube::SOLVED.try_into().unwrap();
            // let solved: Phase2Coord = solved.into();
            // let goalpc = PruneCoord::from(solved);

            // loop {
            //     let (dist, pc) = heap.pop().unwrap();
            //     let dist = -dist;
            //     if pc == goalpc {
            //         return dist as u8;
            //     }

            //     let Phase2Vec { coset: i, ep: ep } = pc.into();

            //     for m in P2Move::iter() {
            //         // FIXME
            //         let npc = m * pc;
            //         m * pc;

            //         let npc = j * FACT8 + nep;
            //         if p2.prunetable[npc] == p2.prunetable[pc] - 1 {
            //             println!("{}", p2.prunetable[npc]);
            //             heap.push((-(dist + 1), npc));
            //         }
            //     }
            // }
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
                let nstate = m * state;
                if set.contains(&nstate) {
                    continue;
                }
                set.insert(nstate); // TODO: ayashii

                let nlb = self.prunetable[PruneCoord::from(nstate).coord()];

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
