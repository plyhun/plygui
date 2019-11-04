use crate::{layout, types};

use super::auto::{HasInner};
use super::control::{Control, ControlInner, AControl, ControlBase};
use super::container::{Container, ContainerInner, AContainer};
use super::drawable::{Drawable};
use super::has_layout::{HasLayoutInner};
use super::member::{Member, MemberBase, AMember, MemberInner};
use super::has_size::HasSizeInner;
use super::has_visibility::HasVisibilityInner;

define! {
    SingleContainer: Container {
        outer: {
            fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>>;
            fn child(&self) -> Option<&dyn Control>;
            fn child_mut(&mut self) -> Option<&mut dyn Control>;
        }
        inner: {
            fn set_child(&mut self, base: &mut MemberBase, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>>;
            fn child(&self) -> Option<&dyn Control>;
            fn child_mut(&mut self) -> Option<&mut dyn Control>;
        }
    }
}

impl<T: SingleContainerInner + ControlInner + Drawable> Drawable for ASingleContainer<T> {
    #[inline]
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.draw(member, control)
    }
    #[inline]
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(member, control, w, h)
    }
    #[inline]
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
impl<T: SingleContainerInner + ControlInner> HasLayoutInner for ASingleContainer<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
    #[inline]
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner.layout_margin(member)
    }
}
impl<T: SingleContainerInner + ControlInner> HasSizeInner for ASingleContainer<T> {
    fn on_size_set(&mut self, base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.inner.on_size_set(base, value)
    }
}
impl<T: SingleContainerInner + ControlInner> HasVisibilityInner for ASingleContainer<T> {
    fn on_visibility_set(&mut self, base: &mut MemberBase, value: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, value)
    }
}
impl<T: SingleContainerInner + ControlInner> ControlInner for ASingleContainer<T> {
    #[inline]
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16) {
        self.inner.on_added_to_container(member, control, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container) {
        self.inner.on_removed_from_container(member, control, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&dyn Member> {
        self.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn Member> {
        self.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &crate::markup::Markup, registry: &mut crate::markup::MarkupRegistry) {
        self.inner.fill_from_markup(member, control, markup, registry)
    }
}
impl<T: SingleContainerInner> SingleContainer for AMember<AContainer<ASingleContainer<T>>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &dyn SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut dyn SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<dyn SingleContainer> {
        self
    }
}
impl<T: SingleContainerInner + ControlInner> SingleContainer for AMember<AControl<AContainer<ASingleContainer<T>>>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner.inner.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner.inner.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &dyn SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut dyn SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<dyn SingleContainer> {
        self
    }
}
impl<II: SingleContainerInner, T: HasInner<I=II> + MemberInner + Drawable> SingleContainerInner for T {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>> {
        self.inner_mut().set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner().child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner_mut().child_mut()
    }
}
impl<T: SingleContainerInner> MaybeSingleContainer for AMember<AContainer<ASingleContainer<T>>> {
    #[inline]
    fn is_single_container_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single_container(&self) -> Option<&dyn SingleContainer> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> MaybeSingleContainer for AMember<AControl<AContainer<ASingleContainer<T>>>> {
    #[inline]
    fn is_single_container_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single_container(&self) -> Option<&dyn SingleContainer> {
        Some(self)
    }
}
