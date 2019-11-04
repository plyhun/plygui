use crate::callbacks::*;

use super::auto::{HasInner, AsAny};
use super::member::{Member, AMember, MemberInner};

able_to!(Close: Member {} -> bool);

impl<II: CloseableInner, T: HasInner<I=II> + 'static> CloseableInner for T {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner_mut().close(skip_callbacks)
    }
    fn on_close(& mut self, callback: Option <OnClose>) {
        self.inner_mut().on_close(callback)
    }
}
