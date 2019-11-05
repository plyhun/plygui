use super::auto::HasInner;
use super::has_label::{HasLabel, HasLabelInner};
use super::control::{Control, ControlInner, AControl};
use super::member::{MemberInner, AMember};

define! {
    ProgressBar: Control + HasLabel {
        inner: {
            fn with_progress_bar<S: AsRef<str>>(label: S) -> Box<dyn ProgressBar>;
        }
    }
}

impl<II: ProgressBarInner, T: HasInner<I = II> + 'static> ProgressBarInner for T {
    fn with_progress_bar<S: AsRef<str>>(label: S) -> Box<dyn ProgressBar> {
        <<Self as HasInner>::I as ProgressBarInner>::with_progress_bar(label)
    }
}

impl<T: ProgressBarInner> ProgressBar for AMember<AControl<AProgressBar<T>>> {
    #[inline]
    fn as_progress_bar(&self) -> &dyn ProgressBar {
        self
    }
    #[inline]
    fn as_progress_bar_mut(&mut self) -> &mut dyn ProgressBar {
        self
    }
    #[inline]
    fn into_progress_bar(self: Box<Self>) -> Box<dyn ProgressBar> {
        self
    }
}

impl<T: ProgressBarInner> AMember<AControl<AProgressBar<T>>> {
    #[inline]
    pub fn with_progress_bar<S: AsRef<str>>(label: S) -> Box<dyn ProgressBar> {
        T::with_progress_bar(label)
    }
}
