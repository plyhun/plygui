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
pub mod members;
pub mod layout;
pub mod ids;
pub mod traits;
#[macro_use]
pub mod callbacks;

#[cfg(feature = "markup")]
pub mod markup;


#[cfg(test)]
mod tests;