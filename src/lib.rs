#![feature(test)]
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

#[macro_use]
extern crate hex_literal;

pub mod cube;
mod display;
pub mod solve;
pub use display::RubikCube;

mod hash;
mod tee;

extern crate test;
#[cfg(test)]
mod bench;
