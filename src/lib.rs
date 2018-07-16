//#![warn(bare_trait_objects)]
//#![cfg_attr(feature = "markup", feature(unboxed_closures))]
#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
#![cfg_attr(feature = "cargo-clippy", allow(redundant_field_names))]
#![cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]

#![cfg_attr(feature = "type_check", feature(get_type_id))]
#![feature(specialization)]
//#![feature(generic_associated_types)]
//#![feature(associated_type_defaults)]

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
