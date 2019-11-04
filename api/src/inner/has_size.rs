use crate::callbacks::*;

use super::member::{Member, MemberInner, MemberBase};
use super::auto::AsAny;

has_reacted!(Size(u16, u16): Member);
/*
maybe!(HasSize);

impl<T: MemberInner> MaybeHasSize for AMember<T> {
    default fn is_has_size(&self) -> Option<&dyn HasSize> {
        None
    }
    default fn is_has_size_mut(&mut self) -> Option<&mut dyn HasSize> {
        None
    }
}*/
