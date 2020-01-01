use super::auto::{HasInner, Abstract};
use super::container::AContainer;
use super::container_single::{ASingleContainer, SingleContainer, SingleContainerInner};
use super::control::{AControl, Control, ControlInner};
use super::has_label::{HasLabel, HasLabelInner};
use super::member::{AMember, Member};

define! {
    Frame: SingleContainer + Control + HasLabel {
        constructor: {
            fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Frame>;
        }
    }
}

impl<II: FrameInner, T: HasInner<I = II> + Abstract + 'static> FrameInner for T {
    fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Frame> {
        <<Self as HasInner>::I as FrameInner>::with_label(label)
    }
}

impl<T: FrameInner> Frame for AMember<AControl<AContainer<ASingleContainer<AFrame<T>>>>> {
    #[inline]
    fn as_frame(&self) -> &dyn Frame {
        self
    }
    #[inline]
    fn as_frame_mut(&mut self) -> &mut dyn Frame {
        self
    }
    #[inline]
    fn into_frame(self: Box<Self>) -> Box<dyn Frame> {
        self
    }
}

impl<T: FrameInner> NewFrame for AMember<AControl<AContainer<ASingleContainer<AFrame<T>>>>> {
    #[inline]
    fn with_label<S: AsRef<str>>(label: S) -> Box<dyn Frame> {
        T::with_label(label)
    }
}
