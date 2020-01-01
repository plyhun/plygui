use super::auto::{HasInner, Abstract};

use std::any::Any;
use std::fmt::Debug;
use std::hash::Hash;

pub trait NativeId: Any + Debug + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Into<usize> + Sized {
    unsafe fn from_outer(arg: usize) -> Self;
}

pub trait HasNativeId: 'static {
    unsafe fn native_id(&self) -> usize;
}

pub trait HasNativeIdInner: 'static {
    type Id: NativeId;

    unsafe fn native_id(&self) -> Self::Id;
}

impl<II: HasNativeIdInner, T: HasInner<I = II> + Abstract + 'static> HasNativeIdInner for T {
    type Id = <T::I as HasNativeIdInner>::Id;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner().native_id()
    }
}
