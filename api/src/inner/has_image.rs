use super::auto::{HasInner, AsAny};
use super::member::{Member, MemberInner, AMember, MemberBase};

use std::borrow::Cow;

has_settable!(Image(Cow<'_, image::DynamicImage>): Member);

impl<II: HasImageInner, T: HasInner<I=II> + 'static> HasImageInner for T {
    fn image(& self, member : & MemberBase) -> Cow <image::DynamicImage> {
        self.inner().image(member)
    } 
    fn set_image(& mut self, member : & mut MemberBase, arg0 : Cow <image::DynamicImage>) {
        self.inner_mut().set_image(member, arg0)
    }
}
