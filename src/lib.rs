#![cfg_attr(feature = "markup", feature(unboxed_closures))] 

#[macro_use]
extern crate derive_builder;

#[cfg(feature = "markup")]
extern crate serde;
#[cfg(feature = "markup")]
extern crate serde_json;
#[cfg(feature = "markup")]
extern crate typemap;

#[macro_use]
pub mod types;
pub mod development;
pub mod members;
pub mod layout;
pub mod ids;
pub mod traits;
pub mod utils;

#[cfg(feature = "markup")]
pub mod markup;
