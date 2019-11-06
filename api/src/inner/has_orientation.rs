use crate::layout;

use super::auto::{AsAny, HasInner};
use super::member::{AMember, Member, MemberBase, MemberInner};

has_settable!(Orientation(layout::Orientation): Member);

impl<II: HasOrientationInner, T: HasInner<I = II> + 'static> HasOrientationInner for T {
    fn orientation(&self, member: &MemberBase) -> layout::Orientation {
        self.inner().orientation(member)
    }
    fn set_orientation(&mut self, member: &mut MemberBase, arg0: layout::Orientation) {
        self.inner_mut().set_orientation(member, arg0)
    }
}

impl<T: HasOrientationInner> HasOrientation for AMember<T> {
    fn orientation(&self) -> layout::Orientation {
        self.inner.orientation(&self.base)
    }
    fn set_orientation(&mut self, arg0: layout::Orientation) {
        self.inner.set_orientation(&mut self.base, arg0)
    }
    fn as_has_orientation(&self) -> &dyn HasOrientation {
        self
    }
    fn as_has_orientation_mut(&mut self) -> &mut dyn HasOrientation {
        self
    }
    fn into_has_orientation(self: Box<Self>) -> Box<dyn HasOrientation> {
        self
    }
}
