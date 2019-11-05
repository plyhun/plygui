use super::auto::{AsAny, HasInner};
use super::member::{AMember, Member, MemberBase, MemberInner};

use std::borrow::Cow;

has_settable!(Image(Cow<'_, image::DynamicImage>): Member);

impl<II: HasImageInner, T: HasInner<I = II> + 'static> HasImageInner for T {
    fn image(&self, member: &MemberBase) -> Cow<image::DynamicImage> {
        self.inner().image(member)
    }
    fn set_image(&mut self, member: &mut MemberBase, arg0: Cow<image::DynamicImage>) {
        self.inner_mut().set_image(member, arg0)
    }
}

impl<T: HasImageInner> HasImage for AMember<T> {
    fn image(&self) -> Cow<image::DynamicImage> {
        self.inner.image(&self.base)
    }
    fn set_image(&mut self, arg0: Cow<image::DynamicImage>) {
        self.inner.set_image(&mut self.base, arg0)
    }
    fn as_has_image(&self) -> &dyn HasImage {
        self
    }
    fn as_has_image_mut(&mut self) -> &mut dyn HasImage {
        self
    }
    fn into_has_image(self: Box<Self>) -> Box<dyn HasImage> {
        self
    }
}
