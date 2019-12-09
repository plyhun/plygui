use crate::callbacks::*;

use super::auto::{AsAny, HasInner};
use super::member::{Member, MemberBase, MemberInner};

has_reacted!(Size(u16, u16): Member);

impl<II: HasSizeInner, T: HasInner<I = II> + 'static> HasSizeInner for T {
    fn on_size_set(&mut self, member: &mut MemberBase, value: (u16, u16)) -> bool {
        self.inner_mut().on_size_set(member, value)
    }
}
