use crate::callbacks::*;

use super::auto::{HasInner, AsAny, Abstract};
use super::member::{Member};
use super::control::{Control, ControlInner};

able_to!(ItemClick (usize, &mut dyn Control): Control);
/*
impl<T: ItemClickableInner + MemberInner> ItemClickable for AMember<AControl<T>> {
    #[inline]
    default fn on_item_click(&mut self, cb: Option<OnItemClick>) {
        self.inner.inner.on_item_click(cb)
    }
    #[inline]
    default fn item_click(&mut self, i: usize, parent: &mut dyn Control, skip_callbacks: bool) {
        self.inner.inner.item_click(i, parent, skip_callbacks)
    }

    #[inline]
    default fn as_item_clickable(&self) -> &dyn ItemClickable {
        self
    }
    #[inline]
    default fn as_item_clickable_mut(&mut self) -> &mut dyn ItemClickable {
        self
    }
    #[inline]
    default fn into_item_clickable(self: Box<Self>) -> Box<dyn ItemClickable> {
        self
    }
}*/

impl<II: ItemClickableInner, T: HasInner<I = II> + Abstract + 'static> ItemClickableInner for T {
    default fn item_click(&mut self, i: usize, parent: &mut dyn Control, skip_callbacks: bool) {
        self.inner_mut().item_click(i, parent, skip_callbacks)
    }
    default fn on_item_click(&mut self, callback: Option<OnItemClick>) {
        self.inner_mut().on_item_click(callback)
    }
}