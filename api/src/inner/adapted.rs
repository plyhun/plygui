use crate::{callbacks, types};

use super::auto::HasInner;
use super::item_clickable::{ItemClickable, ItemClickableInner, OnItemClick};
use super::container::{Container, ContainerInner, AContainer};
use super::control::{Control, AControl, ControlBase, ControlInner};
use super::member::{AMember, MemberBase, MemberInner};

/*define! {
    Adapted: Control + Container + ItemClickable {
        base: {
            pub adapter: Box<dyn types::Adapter>,
            pub on_item_click: Option<OnItemClick>,
        }
        outer: {
            fn adapter(&self) -> &dyn types::Adapter;
            fn adapter_mut(&mut self) -> &mut dyn types::Adapter;
            
            fn len(&self) -> usize {
                self.adapter().len()
            }
        }
        inner: {
            fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Adapted>;
            fn on_item_change(&mut self, base: &mut MemberBase, value: types::Change);
        }
    }
}*/

pub trait Adapted: Control + Container + ItemClickable {
    fn adapter(&self) -> &dyn types::Adapter;
    fn adapter_mut(&mut self) -> &mut dyn types::Adapter;
    
    fn len(&self) -> usize {
        self.adapter().len()
    }
    
    fn as_adapted(&self) -> &dyn Adapted;
    fn as_adapted_mut(&mut self) -> &mut dyn Adapted;
    fn into_adapted(self: Box<Self>) -> Box<dyn Adapted>;
}
pub trait AdaptedInner: ControlInner + ContainerInner {
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Adapted>;
    fn on_item_change(&mut self, base: &mut MemberBase, value: types::Change);
}

#[repr(C)]
pub struct AdaptedBase {
    pub adapter: Box<dyn types::Adapter>,
    pub on_item_click: Option<OnItemClick>,
}
#[repr(C)]
pub struct AAdapted<T: AdaptedInner> {
    base: AdaptedBase,
    inner: T,
}

impl < T : AdaptedInner > HasInner for AAdapted < T > {
    type I = T; 
    fn inner (& self) -> & Self :: I { & self . inner } 
    fn inner_mut (& mut self) -> & mut Self :: I { & mut self . inner } 
    fn into_inner (self) -> Self :: I { self . inner }
} 
pub trait MaybeAdapted : 'static {
    fn is_adapted (& self) -> Option < & dyn Adapted > { None } 
    fn is_adapted_mut (& mut self) -> Option < & mut dyn Adapted > { None }
} 
impl < T : MemberInner > MaybeAdapted for AMember < T > {
    #[inline] 
    default fn is_adapted (& self) -> Option < & dyn Adapted > { None } 
    #[inline] 
    default fn is_adapted_mut (& mut self) -> Option < &mut dyn Adapted > { None }
}

impl<T: AdaptedInner + 'static> AMember<AControl<AContainer<AAdapted<T>>>> {
    #[inline]
    pub fn as_adapted_parts_mut(&mut self) -> (&mut MemberBase, &mut ControlBase, &mut AdaptedBase, &mut T) {
        let this = self as *mut Self;
        (&mut unsafe { &mut *this }.base, &mut unsafe { &mut *this }.inner.base, &mut unsafe { &mut *this }.inner.inner.inner.base, &mut unsafe { &mut *this }.inner.inner.inner.inner)
    }
    
    #[inline]
    pub(crate) fn on_item_change(base: &mut MemberBase, value: types::Change) {
        let this = base.as_any_mut().downcast_mut::<Self>().unwrap();
        let this2 = this as *mut Self; // bck is stupid;
        let inner = unsafe {&mut *this2}.inner_mut().inner_mut().inner_mut();
        inner.on_item_change(base, value)
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

impl<T: AdaptedInner> ItemClickableInner for AAdapted<T> {
    #[inline]
    fn on_item_click(&mut self, cb: Option<OnItemClick>) {
        self.inner.base.on_item_click = cb;
    }
    #[inline]
    fn item_click(&mut self, arg: usize, item_view: &mut dyn Control, skip_callbacks: bool) {
        if !skip_callbacks{
            let self2 = self as *mut Self;
            if let Some(ref mut callback) = self.inner.base.on_item_click {
                (callback.as_mut())(unsafe { &mut *self2 }, arg, item_view)
            }
        }
    }
}

impl<T: AdaptedInner + ControlInner> Adapted for AMember<AControl<AContainer<AAdapted<T>>>> {
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
impl<T: AdaptedInner + ControlInner> MaybeAdapted for AMember<AControl<AContainer<AAdapted<T>>>> {
    #[inline]
    fn is_adapted(&self) -> Option<&dyn Adapted> {
        Some(self)
    }
    #[inline]
    fn is_adapted_mut(&mut self) -> Option<&mut dyn Adapted> {
        Some(self)
    }
}
impl<II: AdaptedInner, T: HasInner<I = II> + 'static> AdaptedInner for T {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Adapted> {
        <<Self as HasInner>::I as AdaptedInner>::with_adapter(adapter)
    }
     #[inline]
    fn on_item_change(&mut self, base: &mut MemberBase, value: types::Change) {
        self.inner_mut().on_item_change(base, value)
    }
}
