use crate::{layout, types};

use super::auto::{HasInner};
use super::drawable::Drawable;
use super::has_layout::{HasLayoutInner};
use super::control::{Control, ControlBase, ControlInner, AControl};
use super::member::{Member, MemberBase, AMember, MemberInner};
//use super::container_single::{SingleContainer, SingleContainerInner, MaybeSingleContainer};
//use super::container_multi::{MultiContainer, MultiContainerInner, MaybeMultiContainer};
use super::has_size::HasSizeInner;
use super::has_visibility::HasVisibilityInner;

define! {
    Container: Member {
        outer: {
            fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control>;
            fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control>;
        },
        inner: {
            fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control>;
            fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control>;
        }
    }
}

impl<T: ContainerInner> Container for AMember<T> {
    #[inline]
    default fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    default fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        self.inner.find_control(arg)
    }

    #[inline]
    default fn as_container(&self) -> &dyn Container {
        self
    }
    #[inline]
    default fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
    #[inline]
    default fn into_container(self: Box<Self>) -> Box<dyn Container> {
        self
    }
}
impl<T: ContainerInner + ControlInner> HasLayoutInner for AContainer<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
    #[inline]
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner.layout_margin(member)
    }
}
impl<T: ContainerInner + ControlInner> HasSizeInner for AContainer<T> {
    fn on_size_set(&mut self, base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.inner.on_size_set(base, value)
    }
}
impl<T: ContainerInner + ControlInner> HasVisibilityInner for AContainer<T> {
    fn on_visibility_set(&mut self, base: &mut MemberBase, value: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, value)
    }
}
impl<T: ContainerInner + ControlInner + Drawable> Drawable for AContainer<T> {
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
impl<T: ContainerInner + ControlInner> Container for AMember<AControl<AContainer<T>>> {
    #[inline]
    default fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
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
        self.inner.inner.inner.find_control_mut(arg)
    }
    #[inline]
    default fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
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
        self.inner.inner.inner.find_control(arg)
    }

    #[inline]
    default fn as_container(&self) -> &dyn Container {
        self
    }
    #[inline]
    default fn as_container_mut(&mut self) -> &mut dyn Container {
        self
    }
    #[inline]
    default fn into_container(self: Box<Self>) -> Box<dyn Container> {
        self
    }
}
impl<T: ContainerInner> MaybeContainer for AMember<AContainer<T>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<T: ContainerInner + ControlInner> MaybeContainer for AMember<AControl<AContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        Some(self)
    }
}
impl<II: ContainerInner, T: HasInner<I=II> + MemberInner + Drawable> ContainerInner for T {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control> {
        self.inner_mut().find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control> {
        self.inner().find_control(arg)
    }
}
