use crate::callbacks::*;

use super::auto::{AsAny, HasInner};
use super::member::{Member, MemberInner};

able_to!(Click: Member);
/*
impl<T: ClickableInner> Clickable for AMember<T> {
    #[inline]
    default fn on_click(&mut self, cb: Option<OnClick>) {
        self.inner.on_click(cb)
    }
    #[inline]
    fn click(&mut self, skip_callbacks: bool) {
        self.inner.click(skip_callbacks)
    }

    #[inline]
    fn as_clickable(&self) -> &dyn Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut dyn Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<dyn Clickable> {
        self
    }
}*/
