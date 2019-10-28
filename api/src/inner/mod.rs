use std::any::Any;

mod auto;
mod member;
mod native_id;
mod control;
mod drawable;
mod container;

#[cfg(feature = "type_check")]
use std::any::TypeId;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait HasBase: Sized + 'static {
    type Base: Sized;

    fn base(&self) -> &Self::Base;
    fn base_mut(&mut self) -> &mut Self::Base;
}

pub trait HasInner: Sized + 'static {
    type Inner: Sized;
    type Params: Sized;

    fn with_inner(inner: Self::Inner, params: Self::Params) -> Self;
    fn as_inner(&self) -> &Self::Inner;
    fn as_inner_mut(&mut self) -> &mut Self::Inner;
    fn into_inner(self) -> Self::Inner;
}

pub(crate) mod seal {
    pub trait Sealed {}
}
