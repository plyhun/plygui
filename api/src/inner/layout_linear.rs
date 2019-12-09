use crate::layout;

use super::auto::HasInner;
use super::container::AContainer;
use super::container_multi::{AMultiContainer, MultiContainer, MultiContainerInner};
use super::control::{AControl, Control, ControlInner};
use super::has_orientation::{HasOrientation, HasOrientationInner};
use super::member::{AMember, Member};

define! {
    LinearLayout: MultiContainer + Control + HasOrientation {
        inner: {
            fn with_orientation(orientation: layout::Orientation) -> Box<dyn LinearLayout>;
        }
    }
}

impl<II: LinearLayoutInner, T: HasInner<I = II> + 'static> LinearLayoutInner for T {
    fn with_orientation(orientation: layout::Orientation) -> Box<dyn LinearLayout> {
        <<Self as HasInner>::I as LinearLayoutInner>::with_orientation(orientation)
    }
}

impl<T: LinearLayoutInner> LinearLayout for AMember<AControl<AContainer<AMultiContainer<ALinearLayout<T>>>>> {
    #[inline]
    fn as_linear_layout(&self) -> &dyn LinearLayout {
        self
    }
    #[inline]
    fn as_linear_layout_mut(&mut self) -> &mut dyn LinearLayout {
        self
    }
    #[inline]
    fn into_linear_layout(self: Box<Self>) -> Box<dyn LinearLayout> {
        self
    }
}

impl<T: LinearLayoutInner> AMember<AControl<AContainer<AMultiContainer<ALinearLayout<T>>>>> {
    #[inline]
    pub fn with_orientation(orientation: layout::Orientation) -> Box<dyn LinearLayout> {
        T::with_orientation(orientation)
    }
}
