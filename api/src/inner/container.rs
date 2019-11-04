use crate::types;

use super::HasInner;
use super::control::Control;
use super::container_single::SingleContainer;
use super::container_multi::MultiContainer;
use super::member::{Member, AMember, MemberInner};

define! {
    Container: Member {
        outer: {
            fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control>;
            fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control>;
        
            fn is_multi_mut(&mut self) -> Option<&mut dyn MultiContainer> {
                None
            }
            fn is_multi(&self) -> Option<&dyn MultiContainer> {
                None
            }
            fn is_single_mut(&mut self) -> Option<&mut dyn SingleContainer> {
                None
            }
            fn is_single(&self) -> Option<&dyn SingleContainer> {
                None
            }
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
    #[inline]
    default fn is_single_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        None
    }
    #[inline]
    default fn is_single(&self) -> Option<&dyn SingleContainer> {
        None
    }
    #[inline]
    default fn is_multi_mut(&mut self) -> Option<&mut dyn MultiContainer> {
        None
    }
    #[inline]
    default fn is_multi(&self) -> Option<&dyn MultiContainer> {
        None
    }
}

