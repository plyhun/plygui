#![warn(bare_trait_objects)]
//#![cfg_attr(feature = "markup", feature(unboxed_closures))]
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
pub(crate) mod runtime;

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
