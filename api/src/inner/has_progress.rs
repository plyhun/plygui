use crate::types;

use super::auto::{AsAny, HasInner};
use super::member::{AMember, Member, MemberBase, MemberInner};

has_settable!(Progress(types::Progress): Member);

impl<II: HasProgressInner, T: HasInner<I = II> + 'static> HasProgressInner for T {
    fn progress(&self, member: &MemberBase) -> types::Progress {
        self.inner().progress(member)
    }
    fn set_progress(&mut self, member: &mut MemberBase, arg0: types::Progress) {
        self.inner_mut().set_progress(member, arg0)
    }
}

impl<T: HasProgressInner> HasProgress for AMember<T> {
    fn progress(&self) -> types::Progress {
        self.inner.progress(&self.base)
    }
    fn set_progress(&mut self, arg0: types::Progress) {
        self.inner.set_progress(&mut self.base, arg0)
    }
    fn as_has_progress(&self) -> &dyn HasProgress {
        self
    }
    fn as_has_progress_mut(&mut self) -> &mut dyn HasProgress {
        self
    }
    fn into_has_progress(self: Box<Self>) -> Box<dyn HasProgress> {
        self
    }
}
