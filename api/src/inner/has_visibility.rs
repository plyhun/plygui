use crate::callbacks::*;
use crate::types;

use super::member::{Member, MemberInner, MemberBase};
use super::auto::AsAny;

has_reacted!(Visibility(types::Visibility): Member);

/*maybe!(HasVisibility);

impl<T: MemberInner> MaybeHasVisibility for AMember<T> {}

*/