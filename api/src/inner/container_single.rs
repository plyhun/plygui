use super::auto::{HasInner, Abstract};
use super::container::{Container, ContainerInner};
use super::control::{Control};
use super::member::{Member, AMember, MemberBase};

define_abstract! {
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
impl<T: SingleContainerInner> SingleContainer for AMember<T> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>> {
        self.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn Control> {
        self.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn Control> {
        self.inner.child_mut()
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
impl<II: SingleContainerInner, T: HasInner<I = II> + Abstract + 'static> SingleContainerInner for T {
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
