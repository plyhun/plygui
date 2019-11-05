use crate::callbacks::*;
use crate::types;

use super::auto::{AsAny, HasInner};
use super::member::{AMember, Member, MemberBase, MemberInner};

has_settable_reacted!(Progress(types::Progress): Member);

impl<II: HasProgressInner, T: HasInner<I = II> + 'static> HasProgressInner for T {
    fn progress(&self, member: &MemberBase) -> types::Progress {
        self.inner().progress(member)
    }
    fn set_progress(&mut self, member: &mut MemberBase, arg0: types::Progress) {
        self.inner_mut().set_progress(member, arg0)
    }
    fn on_progress(&mut self, member: &mut MemberBase, callback: Option<OnProgress>) {
        self.inner_mut().on_progress(member, callback)
    }
}
