use crate::{callbacks, types::{self, adapter}, utils};

use super::auto::{HasInner, Abstract};
use super::container::{Container, ContainerInner, AContainer};
use super::control::{Control, AControl, ControlBase, ControlInner};
use super::member::{Member, AMember, MemberBase};

define_abstract! {
    Adapted: Control + Container {
        base: {
            pub adapter: Box<dyn types::Adapter>,
        }
        outer: {
            fn adapter(&self) -> &dyn types::Adapter;
            fn adapter_mut(&mut self) -> &mut dyn types::Adapter;
            
            fn len_at(&self, indexes: &[usize]) -> Option<usize> {
                self.adapter().len_at(indexes)
            }
        }
        inner: {
            fn on_item_change<'a>(&mut self, base: &mut MemberBase, value: adapter::Change<'a>);
        }
    }
}

impl<T: AdaptedInner + 'static> AAdapted<T> {
    #[inline]
    pub fn with_inner<O: Adapted>(inner: T, adapter: Box<dyn types::Adapter>, u: &mut ::std::mem::MaybeUninit<O>) -> Self {
        let mut t = Self { base: AdaptedBase { adapter }, inner };
        
        let base = u as *mut _ as *mut MemberBase;
        t.base.adapter.on_item_change(Some(AdapterInnerCallback {
            target: base,
            on_item_change: Self::on_item_change.into(),
        }));
        t
    }
    #[inline]
    fn on_item_change(base: &mut MemberBase, value: adapter::Change) {
        let this = base.as_any_mut().downcast_mut::<AMember<AControl<AContainer<AAdapted<T>>>>>().unwrap();
        let this2 = this as *mut AMember<AControl<AContainer<AAdapted<T>>>>; // bck is stupid;
        let inner = unsafe {&mut *this2}.inner_mut().inner_mut().inner_mut().inner_mut();
        inner.on_item_change(base, value)
    }
}

impl<T: AdaptedInner + ControlInner + 'static> AMember<AControl<AContainer<AAdapted<T>>>> {
    #[inline]
    pub fn as_adapted_parts_mut(&mut self) -> (&mut MemberBase, &mut ControlBase, &mut AdaptedBase, &mut T) {
        let this = self as *mut Self;
        (&mut unsafe { &mut *this }.base, &mut unsafe { &mut *this }.inner.base, &mut unsafe { &mut *this }.inner.inner.inner.base, &mut unsafe { &mut *this }.inner.inner.inner.inner)
    }
    #[inline]
    pub unsafe fn adapter_base_parts_mut(base: &mut MemberBase) -> (&mut MemberBase, &mut ControlBase, &mut AdaptedBase, &mut T) {
        utils::base_to_impl_mut::<Self>(base).as_adapted_parts_mut()
    }
}

pub struct AdapterInnerCallback {
    target: *mut MemberBase,
    on_item_change: callbacks::OnItemChange,
}
impl AdapterInnerCallback {
    pub fn on_item_change(&mut self, value: adapter::Change) {
        if !self.target.is_null() {
            (self.on_item_change.as_mut())(unsafe {&mut *self.target}, value)
        }
    }
}

impl<T: AdaptedInner> Adapted for AMember<AControl<AContainer<AAdapted<T>>>> {
    #[inline]
    default fn adapter(&self) -> &dyn types::Adapter {
        self.inner.inner.inner.base.adapter.as_ref()
    }
    #[inline]
    default fn adapter_mut(&mut self) -> &mut dyn types::Adapter {
        self.inner.inner.inner.base.adapter.as_mut()
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
impl<II: AdaptedInner, T: HasInner<I = II> + Abstract + 'static> AdaptedInner for T {
    #[inline]
    fn on_item_change(&mut self, base: &mut MemberBase, value: adapter::Change) {
        self.inner_mut().on_item_change(base, value)
    }
}
