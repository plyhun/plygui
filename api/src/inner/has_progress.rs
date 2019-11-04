use crate::types;
use crate::callbacks::*;

use super::member::{Member, MemberInner, AMember, MemberBase};
use super::auto::{HasInner, AsAny};

has_settable_reacted!(Progress(types::Progress): Member);

impl<II: HasProgressInner, T: HasInner<I=II> + 'static> HasProgressInner for T {
    fn progress (& self, member : & MemberBase) -> types :: Progress {
        self.inner().progress(member)
    }
    fn set_progress (& mut self, member : & mut MemberBase, arg0 : types :: Progress) {
        self.inner_mut().set_progress(member, arg0)
    }
}
