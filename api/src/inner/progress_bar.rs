use crate::types;

use super::auto::HasInner;
use super::control::{AControl, Control, ControlInner};
use super::has_progress::{HasProgress, HasProgressInner};
use super::member::{AMember, Member};

define! {
    ProgressBar: Control + HasProgress {
        inner: {
            fn with_progress(progress: types::Progress) -> Box<dyn ProgressBar>;
        }
    }
}

impl<II: ProgressBarInner, T: HasInner<I = II> + 'static> ProgressBarInner for T {
    fn with_progress(progress: types::Progress) -> Box<dyn ProgressBar> {
        <<Self as HasInner>::I as ProgressBarInner>::with_progress(progress)
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
    pub fn with_progress(progress: types::Progress) -> Box<dyn ProgressBar> {
        T::with_progress(progress)
    }
}
