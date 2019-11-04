use crate::callbacks::*;

use super::member::{Member, MemberInner, AMember, MemberBase};
use super::control::{ControlInner, AControl};
use super::auto::{HasInner, AsAny};

has_reacted!(Size(u16, u16): Member);

impl<T: ControlInner> MaybeHasSize for AMember<AControl<T>> {
    fn is_has_size(&self) -> Option<&dyn HasSize> {
        Some(self)
    }
    fn is_has_size_mut(&mut self) -> Option<&mut dyn HasSize> {
        Some(self)
    }
}
impl<II: HasSizeInner, T: HasInner<I=II> + 'static> HasSizeInner for T {
    fn on_size_set(&mut self, member : &mut MemberBase, value : (u16, u16)) -> bool {
        self.inner_mut().on_size_set(member, value)
    }
}
