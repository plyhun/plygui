use crate::callbacks::*;
use crate::{types};
use crate::controls::{Application};

use super::member::{Member, MemberInner, MemberBase};

use std::any::Any;
use std::borrow::Cow;

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

able_to!(Close: Member {} -> bool);
able_to!(Click: Member);

has_settable!(Label(Cow<'_, str>): Member);
has_settable!(Image(Cow<'_, image::DynamicImage>): Member);
has_settable_reacted!(Progress(types::Progress): Member);

on!(Frame (&mut dyn Application) -> bool);
