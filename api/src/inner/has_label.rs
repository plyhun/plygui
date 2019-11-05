use super::auto::{AsAny, HasInner};
use super::member::{AMember, Member, MemberBase, MemberInner};

use std::borrow::Cow;

has_settable!(Label(Cow<'_, str>): Member);

impl<II: HasLabelInner, T: HasInner<I = II> + 'static> HasLabelInner for T {
    fn label(&self, member: &MemberBase) -> Cow<str> {
        self.inner().label(member)
    }
    fn set_label(&mut self, member: &mut MemberBase, arg0: Cow<str>) {
        self.inner_mut().set_label(member, arg0)
    }
}

impl<T: HasLabelInner> HasLabel for AMember<T> {
    fn label(&self) -> Cow<str> {
        self.inner.label(&self.base)
    }
    fn set_label(&mut self, arg0: Cow<str>) {
        self.inner.set_label(&mut self.base, arg0)
    }
    fn as_has_label(&self) -> &dyn HasLabel {
        self
    }
    fn as_has_label_mut(&mut self) -> &mut dyn HasLabel {
        self
    }
    fn into_has_label(self: Box<Self>) -> Box<dyn HasLabel> {
        self
    }
}
