use crate::types;

use super::auto::{HasInner, Abstract};
use super::container::AContainer;
use super::item_clickable::{ItemClickable, OnItemClick};
use super::adapted::{AAdapted, Adapted, AdaptedInner};
use super::control::{AControl, Control, ControlInner};
use super::member::{AMember, Member};

define! {
    List: Control + Adapted {
	    base: {
            pub on_item_click: Option<OnItemClick>,
        }
	    constructor: {
    	    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn List>;
	    }
	    extends: { ItemClickable }
    }
}
impl<T: ListInner + 'static> AList<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        Self { base: ListBase { on_item_click: None }, inner }
    }
}
impl<II: ListInner, T: HasInner<I = II> + Abstract + 'static> ListInner for T {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn List> {
        <<Self as HasInner>::I as ListInner>::with_adapter(adapter)
    }
}
impl<T: ListInner> NewList for AMember<AControl<AContainer<AAdapted<AList<T>>>>> {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn List> {
        <<Self as HasInner>::I as ListInner>::with_adapter(adapter)
    }
}
impl<T: ListInner> ItemClickable for AMember<AControl<AContainer<AAdapted<AList<T>>>>> {
    #[inline]
    fn on_item_click(&mut self, cb: Option<OnItemClick>) {
        self.inner.inner.inner.inner.base.on_item_click = cb;
    }
    #[inline]
    fn item_click(&mut self, arg: usize, item_view: &mut dyn Control, skip_callbacks: bool) {
        if !skip_callbacks{
            let self2 = self as *mut Self;
            if let Some(ref mut callback) = self.inner.inner.inner.inner.base.on_item_click {
                (callback.as_mut())(unsafe { &mut *self2 }, arg, item_view)
            }
        }
    }
    #[inline]
    fn as_item_clickable(&self) -> &dyn ItemClickable {
        self
    }
    #[inline]
    fn as_item_clickable_mut(&mut self) -> &mut dyn ItemClickable {
        self
    }
    #[inline]
    fn into_item_clickable(self: Box<Self>) -> Box<dyn ItemClickable> {
        self
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

