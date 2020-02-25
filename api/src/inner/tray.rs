use super::auto::{HasInner, Abstract};
use super::application::Application;
use super::closeable::{Closeable, CloseableInner};
use super::has_image::{HasImage, HasImageInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member, MemberInner};

use crate::types;

define! {
    Tray: Member + HasLabel + HasImage + Closeable {
    	base: {
            pub application: Box<dyn Application>,
        }
        constructor: {
        	fn with_params<S: AsRef<str>>(title: S, icon: image::DynamicImage, menu: types::Menu) -> Box<dyn Tray>;
        }
        inner_constructor_params: {
            title: &str, icon: image::DynamicImage, menu: types::Menu
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
impl<T: TrayInner> NewTray for AMember<ATray<T>> {
    #[inline]
    fn with_params<S: AsRef<str>>(title: S, icon: image::DynamicImage, menu: types::Menu) -> Box<dyn Tray> {
        T::with_params(title, icon, menu)
    }
}
impl<II: TrayInner, T: HasInner<I = II> + Abstract + 'static> TrayInner for T {
    fn with_params<S: AsRef<str>>(title: S, icon: image::DynamicImage, menu: types::Menu) -> Box<dyn Tray> {
        <<Self as HasInner>::I as TrayInner>::with_params(title, icon, menu)
    }
}
impl<T: TrayInner> ATray<T> {
    pub fn with_inner(inner: T, application: Box<dyn Application>) -> Self {
        ATray {
            base: TrayBase {
                application,
            },
            inner,
        }
    }
}
