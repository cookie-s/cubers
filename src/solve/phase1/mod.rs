mod flip;
mod twist;
mod udslice;

#[allow(unused_imports)]
use self::flip::*;
#[allow(unused_imports)]
use self::twist::*;
#[allow(unused_imports)]
use self::udslice::*;

#[allow(unused_imports)]
use crate::cube;
#[allow(unused_imports)]
use crate::solve::util::VecU2;
#[allow(unused_imports)]
use cube::{Move, Sym16};

#[allow(unused_imports)]
use num_traits::cast::FromPrimitive;
#[allow(unused_imports)]
use std::convert::TryInto;
#[allow(unused_imports)]
use std::ops::Mul;
#[allow(unused_imports)]
use strum::IntoEnumIterator;

#[derive(Deserialize, Serialize)]
pub struct Phase1 {
    prunetable: VecU2, // CPERMCOSET_COUNT * EPERM_COUNT
}
