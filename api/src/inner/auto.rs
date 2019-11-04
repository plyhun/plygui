use crate::callbacks::*;
use crate::{types};

//use super::application::{Application};
use super::member::{Member, MemberInner, AMember, MemberBase};

use std::any::Any;

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

//on!(Frame (&mut dyn Application) -> bool);
