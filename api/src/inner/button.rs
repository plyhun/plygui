use super::auto::HasInner;
use super::clickable::{Clickable, OnClick};
use super::control::{AControl, Control, ControlInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member};

define! {
    Button: Control + HasLabel {
        base: {
            pub on_click: Option<OnClick>,
        }
        inner: {
            fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Button>;
        }
        extends: {
            Clickable
        }
    }
}

impl<II: ButtonInner, T: HasInner<I = II> + 'static> ButtonInner for T {
    fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Button> {
        <<Self as HasInner>::I as ButtonInner>::with_label(label)
    }
}

impl<T: ButtonInner> AButton<T> {
    pub fn with_inner(inner: T) -> Self {
        Self { base: ButtonBase { on_click: None }, inner }
    }
}

impl<T: ButtonInner> Clickable for AMember<AControl<AButton<T>>> {
    #[inline]
    fn on_click(&mut self, cb: Option<OnClick>) {
        self.inner.inner.base.on_click = cb;
    }
    #[inline]
    fn click(&mut self, skip_callbacks: bool) {
        if !skip_callbacks {
            let this = self as *mut Self;
            if let Some(ref mut on_click) = self.inner.inner.base.on_click {
                (on_click.as_mut())(unsafe {&mut *this});
            }
        }
    }

    #[inline]
    fn as_clickable(&self) -> &dyn Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut dyn Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<dyn Clickable> {
        self
    }
}

impl<T: ButtonInner> Button for AMember<AControl<AButton<T>>> {
    #[inline]
    fn as_button(&self) -> &dyn Button {
        self
    }
    #[inline]
    fn as_button_mut(&mut self) -> &mut dyn Button {
        self
    }
    #[inline]
    fn into_button(self: Box<Self>) -> Box<dyn Button> {
        self
    }
}

impl<T: ButtonInner> AMember<AControl<AButton<T>>> {
    #[inline]
    pub fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Button> {
        T::with_label(label)
    }
}
