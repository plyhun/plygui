use crate::layout;

use super::auto::HasInner;
use super::container::AContainer;
use super::container_multi::{AMultiContainer, MultiContainer, MultiContainerInner};
use super::control::{AControl, Control, ControlInner};
use super::has_orientation::{HasOrientation, HasOrientationInner};
use super::member::{AMember, MemberBase, Member};

define! {
    Splitted: MultiContainer + Control + HasOrientation {
        outer: {
            fn first(&self) -> &dyn Control;
            fn second(&self) -> &dyn Control;
            fn first_mut(&mut self) -> &mut dyn Control;
            fn second_mut(&mut self) -> &mut dyn Control;

            fn set_splitter(&mut self, pos: f32);
            fn splitter(&self) -> f32;
        }
        inner: {
            fn with_content(first: Box<dyn Control>, second: Box<dyn Control>, orientation: layout::Orientation) -> Box<dyn Splitted>;
            fn set_splitter(&mut self, member: &mut MemberBase, pos: f32);
            fn splitter(&self) -> f32;
            fn first(&self) -> &dyn Control;
            fn second(&self) -> &dyn Control;
            fn first_mut(&mut self) -> &mut dyn Control;
            fn second_mut(&mut self) -> &mut dyn Control;
        }
    }
}

impl<II: SplittedInner, T: HasInner<I = II> + 'static> SplittedInner for T {
    #[inline]
    fn with_content(first: Box<dyn Control>, second: Box<dyn Control>, orientation: layout::Orientation) -> Box<dyn Splitted> {
        <<Self as HasInner>::I as SplittedInner>::with_content(first, second, orientation)
    }
    #[inline]
    fn set_splitter(&mut self, member: &mut MemberBase, pos: f32) {
        self.inner_mut().set_splitter(member, pos)
    }
    #[inline]
    fn splitter(&self) -> f32 {
        self.inner().splitter()
    }
    #[inline]
    fn first(&self) -> &dyn Control {
        self.inner().first()
    }
    #[inline]
    fn second(&self) -> &dyn Control {
        self.inner().second()
    }
    #[inline]
    fn first_mut(&mut self) -> &mut dyn Control {
        self.inner_mut().first_mut()
    }
    #[inline]
    fn second_mut(&mut self) -> &mut dyn Control {
        self.inner_mut().second_mut()
    }
}

impl<T: SplittedInner> Splitted for AMember<AControl<AContainer<AMultiContainer<ASplitted<T>>>>> {
    #[inline]
    fn set_splitter(&mut self, pos: f32) {
        self.inner.inner.inner.inner.inner.set_splitter(&mut self.base, pos)
    }
    #[inline]
    fn splitter(&self) -> f32 {
        self.inner.inner.inner.inner.inner.splitter()
    }
    #[inline]
    fn first(&self) -> &dyn Control {
        self.inner.inner.inner.inner.inner.first()
    }
    #[inline]
    fn second(&self) -> &dyn Control {
        self.inner.inner.inner.inner.inner.second()
    }
    #[inline]
    fn first_mut(&mut self) -> &mut dyn Control {
        self.inner.inner.inner.inner.inner.first_mut()
    }
    #[inline]
    fn second_mut(&mut self) -> &mut dyn Control {
        self.inner.inner.inner.inner.inner.second_mut()
    }
    #[inline]
    fn as_splitted(&self) -> &dyn Splitted {
        self
    }
    #[inline]
    fn as_splitted_mut(&mut self) -> &mut dyn Splitted {
        self
    }
    #[inline]
    fn into_splitted(self: Box<Self>) -> Box<dyn Splitted> {
        self
    }
}

impl<T: SplittedInner> AMember<AControl<AContainer<AMultiContainer<ASplitted<T>>>>> {
    #[inline]
    pub fn with_content(first: Box<dyn Control>, second: Box<dyn Control>, orientation: layout::Orientation) -> Box<dyn Splitted> {
        T::with_content(first, second, orientation)
    }
}
