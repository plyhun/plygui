use crate::callbacks::*;
use crate::{types, layout};
use crate::controls::{Application};

use super::member::{Member, MemberInner, MemberBase};
use super::control::{Control};
use super::container::{Container};

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
has_settable_reacted!(Progress(types::Progress): Member);

has_reacted!(Size(u16, u16): Member);
has_reacted!(Visibility(types::Visibility): Member);

has_private!(Layout(layout::Size, layout::Size): Member {
    outer: {
        fn layout_width(&self) -> layout::Size;
        fn layout_height(&self) -> layout::Size;
        fn set_layout_width(&mut self, value: layout::Size);
        fn set_layout_height(&mut self, value: layout::Size);
        fn layout_margin(&self) -> layout::BoundarySize;
    },
    inner: {
        fn on_layout_changed(&mut self, base: &mut MemberBase);
        fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize;
    }
});

maybe!(Member);
maybe!(Control);
maybe!(Container);
maybe!(HasSize);
maybe!(HasVisibility);

on!(Frame (&mut dyn Application) -> bool);
