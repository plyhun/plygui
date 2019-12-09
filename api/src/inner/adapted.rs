use crate::{callbacks, types};

use super::auto::HasInner;
use super::container::{Container, ContainerInner, AContainer};
use super::control::{Control, AControl, ControlBase, ControlInner};
use super::member::{Member, AMember, MemberBase};

define! {
    Adapted: Container {
        base: {
            pub adapter: Box<dyn types::Adapter>,
        }
        outer: {
            fn adapter(&self) -> &dyn types::Adapter;
            fn adapter_mut(&mut self) -> &mut dyn types::Adapter;
            
            fn len(&self) -> usize {
                self.adapter().len()
            }
        }
        inner: {
            fn adapter(&self) -> &dyn types::Adapter;
            fn adapter_mut(&mut self) -> &mut dyn types::Adapter;
            
            fn on_item_change(&mut self, base: &mut MemberBase, value: types::Change);
        }
    }
}

impl<T: AdaptedInner + 'static> AAdapted<T> {
    #[inline]
    pub fn with_inner(inner: T, adapter: Box<dyn types::Adapter>) -> Self {
        Self { base: AdaptedBase { adapter }, inner }
    }
}

impl<T: AdaptedInner + ControlInner + 'static> AMember<AControl<AContainer<AAdapted<T>>>> {
    #[inline]
    pub fn as_adapted_parts_mut(&mut self) -> (&mut MemberBase, &mut ControlBase, &mut AdaptedBase, &mut T) {
        let this = self as *mut Self;
        (&mut unsafe { &mut *this }.base, &mut unsafe { &mut *this }.inner.base, &mut unsafe { &mut *this }.inner.inner.inner.base, &mut unsafe { &mut *this }.inner.inner.inner.inner)
    }
}

pub struct AdapterInnerCallback {
    target: *mut MemberBase,
    on_item_change: callbacks::OnItemChange,
}
impl AdapterInnerCallback {
    pub fn on_item_change(&mut self, value: types::Change) {
        if !self.target.is_null() {
            (self.on_item_change.as_mut())(unsafe {&mut *self.target}, value)
        }
    }
}

impl<T: AdaptedInner> Adapted for AMember<T> {
    #[inline]
    default fn adapter(&self) -> &dyn types::Adapter {
        self.inner().adapter()
    }
    #[inline]
    default fn adapter_mut(&mut self) -> &mut dyn types::Adapter {
        self.inner_mut().adapter_mut()
    }

    #[inline]
    default fn as_adapted(&self) -> &dyn Adapted {
        self
    }
    #[inline]
    default fn as_adapted_mut(&mut self) -> &mut dyn Adapted {
        self
    }
    #[inline]
    default fn into_adapted(self: Box<Self>) -> Box<dyn Adapted> {
        self
    }
}

impl<T: AdaptedInner> AdaptedInner for AAdapted<T> {
    fn adapter(&self) -> &dyn types::Adapter { self.base.adapter.as_ref() }
    fn adapter_mut(&mut self) -> &mut dyn types::Adapter { self.base.adapter.as_mut() }
}

impl<II: AdaptedInner, T: HasInner<I = II> + 'static> AdaptedInner for T {
    #[inline]
    default fn adapter(&self) -> &dyn types::Adapter {
        self.inner().adapter()
    }
    #[inline]
    default fn adapter_mut(&mut self) -> &mut dyn types::Adapter {
        self.inner_mut().adapter_mut()
    }
    #[inline]
    fn on_item_change(&mut self, base: &mut MemberBase, value: types::Change) {
        self.inner_mut().on_item_change(base, value)
    }
}
