//#![cfg_attr(feature = "markup", feature(unboxed_closures))]
#![cfg_attr(feature = "cargo-clippy", allow(wrong_self_convention))]
#![cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
#![cfg_attr(feature = "cargo-clippy", allow(borrowed_box))]

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
pub mod utils;
#[macro_use]
pub mod callbacks;
#[macro_use]
pub mod macros;

#[cfg(feature = "markup")]
pub mod markup;
