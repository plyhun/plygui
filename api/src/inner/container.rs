use crate::types;

use super::auto::{HasInner, Abstract};
use super::control::{AControl, Control, ControlInner};
use super::member::{AMember, Member, MemberInner};
use super::container_single::{MaybeSingleContainer};
use super::container_multi::{MaybeMultiContainer};
use super::adapted::{MaybeAdapted};

define_abstract! {
    Container: Member {
        outer: {
            fn find_control_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn Control>;
            fn find_control<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn Control>;
        },
        inner: {
            fn find_control_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn Control>;
            fn find_control<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn Control>;
        }
        extends: {
            MaybeSingleContainer + MaybeMultiContainer + MaybeAdapted
        }
    }
}
impl<T: ContainerInner> Container for AMember<T> {
    #[inline]
    default fn find_control_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    default fn find_control<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn Control> {
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
impl<T: ContainerInner + ControlInner> Container for AMember<AControl<T>> {
    #[inline]
    default fn find_control_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id() == *id {
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
        self.inner.find_control_mut(arg)
    }
    #[inline]
    default fn find_control<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id() == *id {
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
impl<II: ContainerInner, T: HasInner<I = II> + Abstract + 'static> ContainerInner for T {
    #[inline]
    fn find_control_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn Control> {
        self.inner_mut().find_control_mut(arg)
    }
    #[inline]
    fn find_control<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn Control> {
        self.inner().find_control(arg)
    }
}
