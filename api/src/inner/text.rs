use super::auto::{HasInner, Abstract, Spawnable};
use super::control::{AControl, Control, ControlInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member};

define! {
    Text: Control + HasLabel {
        constructor: {
            fn with_text<S: AsRef<str>>(label: S) -> Box<dyn Text>;
        }
    }
}

impl<II: TextInner, T: HasInner<I = II> + Abstract + 'static> TextInner for T {
    fn with_text<S: AsRef<str>>(label: S) -> Box<dyn Text> {
        <<Self as HasInner>::I as TextInner>::with_text(label)
    }
}

impl<T: TextInner> Text for AMember<AControl<AText<T>>> {
    #[inline]
    fn as_text(&self) -> &dyn Text {
        self
    }
    #[inline]
    fn as_text_mut(&mut self) -> &mut dyn Text {
        self
    }
    #[inline]
    fn into_text(self: Box<Self>) -> Box<dyn Text> {
        self
    }
}

impl<T: TextInner> NewText for AMember<AControl<AText<T>>> {
    #[inline]
    fn with_text<S: AsRef<str>>(label: S) -> Box<dyn Text> {
        T::with_text(label)
    }
}

impl<T: TextInner> Spawnable for AMember<AControl<AText<T>>> {
    fn spawn() -> Box<dyn Control> {
        <T as Spawnable>::spawn()
    }
}

