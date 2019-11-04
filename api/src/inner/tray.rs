use super::auto::{HasInner};
use super::member::{Member, MemberInner, AMember};
use super::closeable::{Closeable, CloseableInner};
use super::has_image::{HasImageInner, HasImage};
use super::has_label::{HasLabelInner, HasLabel};

use crate::types;

define! {
    Tray: Member + HasLabel + HasImage + Closeable {
        inner: {
            fn with_params(title: &str, menu: types::Menu) -> Box<AMember<ATray<Self>>>;
        }
    }
}
impl<T: TrayInner> HasLabelInner for ATray<T> {}
impl<T: TrayInner> CloseableInner for ATray<T> {}

impl<T: TrayInner> Tray for AMember<ATray<T>> {}

impl<T: TrayInner> AMember<ATray<T>> {
    #[inline]
    pub fn with_params(title: &str, menu: types::Menu) -> Box<dyn Tray> {
        T::with_params(title, menu)
    }
}