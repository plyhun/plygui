use super::auto::{HasInner, AsAny};

use super::member::{Member, MemberInner, AMember, MemberBase};

use std::borrow::Cow;

has_settable!(Label(Cow<'_, str>): Member);

impl<II: HasLabelInner, T: HasInner<I=II> + 'static> HasLabelInner for T {
    fn label(& self, member : & MemberBase) -> Cow <str > {
        self.inner().label(member)
    } 
    fn set_label(& mut self, member : & mut MemberBase, arg0 : Cow <str >) {
        self.inner_mut().set_label(member, arg0)
    }
}
