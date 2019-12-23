use crate::types;

use super::auto::HasInner;
use super::control::{AControl, Control, ControlInner};
use super::has_image::{HasImage, HasImageInner};
use super::member::{AMember, MemberBase, Member};

define! {
    Image: Control + HasImage {
        outer: {
            fn set_scale(&mut self, policy: types::ImageScalePolicy);
            fn scale(&self) -> types::ImageScalePolicy;
        },
        inner: {
            fn set_scale(&mut self, member: &mut MemberBase, policy: types::ImageScalePolicy);
            fn scale(&self) -> types::ImageScalePolicy;
        }
        constructor: {
            fn with_content(content: image::DynamicImage) -> Box<dyn Image>;
        }
    }
}

impl<II: ImageInner, T: HasInner<I = II> + 'static> ImageInner for T {
    fn with_content(content: image::DynamicImage) -> Box<dyn Image> {
        <<Self as HasInner>::I as ImageInner>::with_content(content)
    }
    fn set_scale(&mut self, member: &mut MemberBase, policy: types::ImageScalePolicy) {
        self.inner_mut().set_scale(member, policy)
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.inner().scale()
    }
}

impl<T: ImageInner> Image for AMember<AControl<AImage<T>>> {
    fn set_scale(&mut self, policy: types::ImageScalePolicy) {
        let base1 = self as *mut _ as *mut AMember<AControl<AImage<T>>>;
        self.inner.inner.inner.set_scale(&mut (unsafe { (&mut *base1) }.base), policy)
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.inner.inner.inner.scale()
    }

    #[inline]
    fn as_image(&self) -> &dyn Image {
        self
    }
    #[inline]
    fn as_image_mut(&mut self) -> &mut dyn Image {
        self
    }
    #[inline]
    fn into_image(self: Box<Self>) -> Box<dyn Image> {
        self
    }
}

impl<T: ImageInner> NewImage for AMember<AControl<AImage<T>>> {
    #[inline]
    fn with_content(content: image::DynamicImage) -> Box<dyn Image> {
        T::with_content(content)
    }
}
