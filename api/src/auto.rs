use crate::callbacks::*;
use crate::types;
use crate::controls::{Member, Control, Container, Application};
use crate::development::{MemberInner, MemberBase};

use std::any::Any;
use std::borrow::Cow;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait HasNativeId: 'static {
    unsafe fn native_id(&self) -> usize;
}

able_to!(Close: Member {} -> bool);
able_to!(Click: Member);

has_settable!(Label(Cow<'_, str>): Member);
has_settable!(Image(Cow<'_, image::DynamicImage>): Member);
has_settable!(Progress(types::Progress): Member);

has_reacted!(Size(u16, u16): Member);
has_reacted!(Visibility(types::Visibility): Member);

maybe!(Member);
maybe!(Control);
maybe!(Container);
maybe!(HasSize);
maybe!(HasVisibility);

on!(Frame (&mut dyn Application) -> bool);
