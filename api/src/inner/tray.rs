use super::auto::HasInner;
use super::closeable::{Closeable, CloseableInner, OnClose};
use super::has_image::{HasImage, HasImageInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member, MemberInner};

use crate::types;

define! {
    Tray: Member + HasLabel + HasImage + Closeable {
        inner: {
            fn with_params<S: AsRef<str>>(title: S, menu: types::Menu) -> Box<dyn Tray>;
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

impl<T: TrayInner> Closeable for AMember<ATray<T>> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<OnClose>) {
        self.inner.inner.on_close(callback)
    }
    fn as_closeable(&self) -> &dyn Closeable {
        self
    }
    fn as_closeable_mut(&mut self) -> &mut dyn Closeable {
        self
    }
    fn into_closeable(self: Box<Self>) -> Box<dyn Closeable> {
        self
    }
}

impl<II: TrayInner, T: HasInner<I = II> + 'static> TrayInner for T {
    fn with_params<S: AsRef<str>>(title: S, menu: types::Menu) -> Box<dyn Tray> {
        <<Self as HasInner>::I as TrayInner>::with_params(title, menu)
    }
}

impl<T: TrayInner> AMember<ATray<T>> {
    #[inline]
    pub fn with_params<S: AsRef<str>>(title: S, menu: types::Menu) -> Box<dyn Tray> {
        T::with_params(title, menu)
    }
}
