use super::auto::HasInner;
use super::container::{AContainer, Container, ContainerInner};
use super::control::{AControl, Control, ControlInner};
use super::member::{AMember, MemberBase, MemberInner};

define! {
    SingleContainer: Container {
        outer: {
            fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>>;
            fn child(&self) -> Option<&dyn Control>;
            fn child_mut(&mut self) -> Option<&mut dyn Control>;
        }
        inner: {
            fn set_child(&mut self, base: &mut MemberBase, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>>;
            fn child(&self) -> Option<&dyn Control>;
            fn child_mut(&mut self) -> Option<&mut dyn Control>;
        }
    }
}
impl<T: SingleContainerInner + ControlInner> SingleContainer for AMember<AControl<AContainer<ASingleContainer<T>>>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner.inner.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner.inner.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &dyn SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut dyn SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<dyn SingleContainer> {
        self
    }
}
impl<T: SingleContainerInner> SingleContainer for AMember<AContainer<ASingleContainer<T>>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &dyn SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut dyn SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<dyn SingleContainer> {
        self
    }
}
impl<II: SingleContainerInner, T: HasInner<I = II> + MemberInner> SingleContainerInner for T {
    #[inline]
    fn set_child(&mut self, base: &mut MemberBase, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>> {
        self.inner_mut().set_child(base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner().child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner_mut().child_mut()
    }
}
impl<T: SingleContainerInner> MaybeSingleContainer for AMember<AContainer<ASingleContainer<T>>> {
    #[inline]
    fn is_single_container_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single_container(&self) -> Option<&dyn SingleContainer> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> MaybeSingleContainer for AMember<AControl<AContainer<ASingleContainer<T>>>> {
    #[inline]
    fn is_single_container_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single_container(&self) -> Option<&dyn SingleContainer> {
        Some(self)
    }
}
