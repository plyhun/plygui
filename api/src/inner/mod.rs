use std::any::Any;

mod auto;
mod member;
mod native_id;
mod control;
mod drawable;
mod container;
mod container_single;
mod container_multi;

//mod button;

#[cfg(feature = "type_check")]
use std::any::TypeId;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait HasInner {
    type I: Sized + 'static;
    
    fn inner(&self) -> &Self::I;
    fn inner_mut(&mut self) -> &mut Self::I;
}

pub(crate) mod seal {
    pub trait Sealed {}
}
