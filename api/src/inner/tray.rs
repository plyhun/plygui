use super::auto::HasInner;
use super::closeable::{Closeable, CloseableInner};
use super::has_image::{HasImage, HasImageInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member, MemberInner};

use crate::types;

define! {
    Tray: Member + HasLabel + HasImage + Closeable {
        inner: {
            fn with_params(title: &str, menu: types::Menu) -> Box<dyn Tray>;
        }
    }
}

impl<T: TrayInner> Tray for AMember<ATray<T>> {
    fn as_tray(&self) -> &dyn Tray {
        self
    }
    fn as_tray_mut(&mut self) -> &mut dyn Tray {
        self
    }
    fn into_tray(self: Box<Self>) -> Box<dyn Tray> {
        self
    }
}

impl<II: TrayInner, T: HasInner<I = II> + 'static> TrayInner for T {
    fn with_params(title: &str, menu: types::Menu) -> Box<dyn Tray> {
        <<Self as HasInner>::I as TrayInner>::with_params(title, menu)
    }
}

impl<T: TrayInner> AMember<ATray<T>> {
    #[inline]
    pub fn with_params(title: &str, menu: types::Menu) -> Box<dyn Tray> {
        T::with_params(title, menu)
    }
}
