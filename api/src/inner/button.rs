use super::auto::{HasInner, Spawnable, Abstract};
use super::clickable::{Clickable, ClickableInner};
use super::control::{AControl, Control, ControlInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member};

define! {
    Button: Control + Clickable + HasLabel {
        constructor: {
            fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Button>;
        }
    }
}

impl<II: ButtonInner, T: HasInner<I = II> + Abstract + 'static> ButtonInner for T {
    fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Button> {
        <<Self as HasInner>::I as ButtonInner>::with_label(label)
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

impl<T: ButtonInner> NewButton for AMember<AControl<AButton<T>>> {
    #[inline]
    fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Button> {
        T::with_label(label)
    }
}

impl<T: ButtonInner> Spawnable for AMember<AControl<AButton<T>>> {
    fn spawn() -> Box<dyn Control> {
        <T as Spawnable>::spawn()
    }
}

