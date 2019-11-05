use crate::callbacks::*;

use super::auto::{AsAny, HasInner};
use super::member::{AMember, Member, MemberInner};

able_to!(Close: Member {} -> bool);

impl<II: CloseableInner, T: HasInner<I = II> + 'static> CloseableInner for T {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner_mut().close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<OnClose>) {
        self.inner_mut().on_close(callback)
    }
}

impl<T: CloseableInner> Closeable for AMember<T> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<OnClose>) {
        self.inner.on_close(callback)
    }
    fn as_closeable(&self) -> &dyn Closeable {
        self
    }
    fn as_closeable_mut(&mut self) -> &mut dyn Closeable {
        self
    }
    fn into_closeable(self: Box<Self>) -> Box<dyn Closeable> {
        self
    }
}
