//#![cfg_attr(feature = "markup", feature(unboxed_closures))] 
//#![cfg_attr(test, feature(plugin))]
//#![cfg_attr(test, plugin(clippy))]

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

#[cfg(feature = "markup")]
pub mod markup;
