#![warn(bare_trait_objects)]
//#![cfg_attr(feature = "markup", feature(unboxed_closures))]
#![cfg_attr(feature = "cargo-clippy", allow(cast_lossless))]
#![cfg_attr(feature = "cargo-clippy", allow(redundant_field_names))]
#![cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
#![feature(specialization)]

#[macro_use]
extern crate plygui_macros;

pub mod controls;
pub mod defaults;
pub mod ids;
pub mod layout;
pub mod types;
pub mod utils;

pub(crate) mod inner;

pub mod sdk;

#[macro_use]
pub mod callbacks;
#[macro_use]
pub mod macros;

#[cfg(feature = "markup")]
pub mod markup;

pub mod external {
    pub use image;
}
