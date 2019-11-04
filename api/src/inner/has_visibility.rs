use crate::callbacks::*;
use crate::types;

use super::member::{Member, MemberInner, AMember, MemberBase};
use super::auto::{HasInner, AsAny};

has_reacted!(Visibility(types::Visibility): Member);

impl<II: HasVisibilityInner, T: HasInner<I=II> + 'static> HasVisibilityInner for T {
    fn on_visibility_set(&mut self, member : &mut MemberBase, value : types::Visibility) -> bool {
        self.inner_mut().on_visibility_set(member, value)
    }
}
