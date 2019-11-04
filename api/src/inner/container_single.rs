use crate::{layout, types};

use super::HasInner;
use super::auto::{HasSizeInner, HasVisibilityInner, HasLayoutInner, MaybeContainer};
use super::control::{Control, ControlInner, AControl, ControlBase};
use super::container::{Container, ContainerInner, AContainer};
use super::native_id::HasNativeIdInner;
use super::drawable::{Drawable};
use super::member::{Member, MemberBase, AMember, MemberInner};

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

impl<T: SingleContainerInner> HasNativeIdInner for ASingleContainer<T> {
    type Id = T::Id;

    #[inline]
    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}

impl<T: SingleContainerInner> MemberInner for ASingleContainer<T> {}

impl<T: SingleContainerInner + ContainerInner> ContainerInner for ASingleContainer<T> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        self.inner.find_control(arg)
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
        self.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner.inner.child_mut()
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
impl<T: SingleContainerInner> MaybeContainer for AMember<AContainer<ASingleContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<T: SingleContainerInner> Container for AMember<AContainer<ASingleContainer<T>>> {
    #[inline]
    fn is_single_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single(&self) -> Option<&dyn SingleContainer> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> SingleContainer for AMember<AControl<AContainer<ASingleContainer<T>>>> {
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
impl<T: SingleContainerInner + ControlInner> MaybeContainer for AMember<AControl<AContainer<ASingleContainer<T>>>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> Container for AMember<AControl<AContainer<ASingleContainer<T>>>> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id() == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id() == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.find_control(arg)
    }

    #[inline]
    fn is_single_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single(&self) -> Option<&dyn SingleContainer> {
        Some(self)
    }

    #[inline]
    fn as_container(&self) -> &dyn Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<dyn Container> {
        self
    }
}

