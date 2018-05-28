//#![cfg_attr(feature = "markup", feature(unboxed_closures))]
#![feature(specialization)]

#[cfg(feature = "markup")]
extern crate serde;
#[cfg(feature = "markup")]
extern crate serde_json;
#[cfg(feature = "markup")]
extern crate typemap;

pub mod types;
pub mod development;
pub mod layout;
pub mod ids;
pub mod controls;
pub mod utils;

#[macro_use]
pub mod callbacks;
#[macro_use]
pub mod macros;

#[cfg(feature = "markup")]
pub mod markup;

#[cfg(test)]
mod tests;