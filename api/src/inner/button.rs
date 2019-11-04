use super::auto::{HasLabel, HasLabelInner, HasInner};
use super::control::{Control, ControlInner, AControl};
use super::member::{MemberInner, AMember};
use super::clickable::{Clickable, ClickableInner};

define! {
    Button: Control + Clickable + HasLabel {
        inner: {
            fn with_label(label: &str) -> Box<AMember<AControl<AButton<Self>>>>;
        }
    }
}

impl<T: ButtonInner> Button for AMember<AControl<AButton<T>>> {}

impl<T: ButtonInner> AMember<AControl<AButton<T>>> {
    #[inline]
    pub fn with_label(label: &str) -> Box<dyn Button> {
        T::with_label(label)
    }
}
