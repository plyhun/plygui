use super::auto::HasInner;
use super::has_label::{HasLabel, HasLabelInner};
use super::control::{Control, ControlInner, AControl};
use super::member::{MemberInner, AMember};
use super::clickable::{Clickable, ClickableInner};

define! {
    Button: Control + Clickable + HasLabel {
        inner: {
            fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Button>;
        }
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
