use crate::callbacks::*;
//use crate::controls::{Application};

use super::auto::{HasInner, AsAny};
use super::member::{Member, AMember, MemberInner};

#[cfg(feature = "type_check")]
use std::any::TypeId;

able_to!(Click: Member);

impl<II: ClickableInner, T: HasInner<I=II> + 'static> ClickableInner for T {
    fn click(&mut self, skip_callbacks: bool) {
        self.inner_mut().click(skip_callbacks)
    }
    fn on_click(& mut self, callback: Option <OnClick>) {
        self.inner_mut().on_click(callback)
    }
}
