use crate::callbacks::*;

use super::auto::{AsAny, HasInner, Abstract};
use super::member::{AMember, Member, MemberInner};

able_to!(Click: Member);

impl<T: ClickableInner + MemberInner> Clickable for AMember<T> {
    #[inline]
    fn on_click(&mut self, cb: Option<OnClick>) {
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
}

impl<II: ClickableInner, T: HasInner<I = II> + Abstract + 'static> ClickableInner for T {
    fn click(&mut self, skip_callbacks: bool) {
        self.inner_mut().click(skip_callbacks)
    }
    fn on_click(&mut self, callback: Option<OnClick>) {
        self.inner_mut().on_click(callback)
    }
}
