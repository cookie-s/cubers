#![feature(type_alias_enum_variants)]

extern crate bincode;

extern crate serde;
extern crate serde_bytes;
#[macro_use]
extern crate serde_derive;

#[macro_use]
extern crate lazy_static;

extern crate strum;
#[macro_use]
extern crate strum_macros;

#[macro_use]
extern crate num_derive;

pub mod cube;
mod display;
pub mod solve;
pub use display::RubikCube;
