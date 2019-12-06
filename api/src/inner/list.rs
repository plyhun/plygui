use crate::types;

use super::auto::HasInner;
use super::container::AContainer;
use super::item_clickable::{ItemClickable, OnItemClick, ItemClickableInner};
use super::adapted::{AAdapted, Adapted, AdaptedInner};
use super::control::{AControl, Control};
use super::member::{AMember, MemberInner, MemberBase};

/*define! {
    List: Adapted + ItemClickable {
	    base: {
            pub on_item_click: Option<OnItemClick>,
        }
    }
}*/

pub trait List: Adapted + ItemClickable {
    fn as_list(&self) -> &dyn List;
    fn as_list_mut(&mut self) -> &mut dyn List;
    fn into_list(self: Box<Self>) -> Box<dyn List>;
}
pub trait ListInner: AdaptedInner {
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn List>;        
}

#[repr(C)]
pub struct ListBase {
    pub on_item_click: Option<OnItemClick>,
}
#[repr(C)]
pub struct AList<T: ListInner> {
    base: ListBase,
    inner: T,
}
impl <T: ListInner> AList<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        Self { base: ListBase { on_item_click: None }, inner }
    }
} 
impl < T : ListInner > HasInner for AList < T > {
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

impl<II: ListInner, T: HasInner<I = II> + 'static> ListInner for T {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn List> {
        <<Self as HasInner>::I as ListInner>::with_adapter(adapter)
    }
}

impl<T: ListInner> ItemClickableInner for AAdapted<AList<T>> {
    #[inline]
    fn on_item_click(&mut self, base: &mut MemberBase, cb: Option<OnItemClick>) {
        self.inner.inner.inner.inner.base.on_item_click = cb;
    }
    #[inline]
    fn item_click(&mut self, base: &mut MemberBase, arg: usize, item_view: &mut dyn Control, skip_callbacks: bool) {
        if !skip_callbacks{
            let self2 = self as *mut Self;
            if let Some(ref mut callback) = self.inner.inner.inner.inner.base.on_item_click {
                (callback.as_mut())(unsafe { &mut *self2 }, arg, item_view)
            }
        }
    }
}

impl<T: ListInner> List for AMember<AControl<AContainer<AAdapted<AList<T>>>>> {
    #[inline]
    fn as_list(&self) -> &dyn List {
        self
    }
    #[inline]
    fn as_list_mut(&mut self) -> &mut dyn List {
        self
    }
    #[inline]
    fn into_list(self: Box<Self>) -> Box<dyn List> {
        self
    }
}

