use super::auto::HasInner;
use super::container::{AContainer, Container, ContainerInner};
use super::control::{AControl, Control, ControlInner};
use super::member::{AMember, MemberBase, MemberInner};

define! {
    MultiContainer: Container {
        outer: {
            fn len(&self) -> usize;
            fn set_child_to(&mut self, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>>;
            fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn Control>>;
            fn child_at(&self, index: usize) -> Option<&dyn Control>;
            fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control>;

            fn is_empty(&self) -> bool {
                self.len() < 1
            }
            fn clear(&mut self) {
                let len = self.len();
                for index in (0..len).rev() {
                    self.remove_child_from(index);
                }
            }
            fn push_child(&mut self, child: Box<dyn Control>) {
                let len = self.len();
                self.set_child_to(len, child);
            }
            fn pop_child(&mut self) -> Option<Box<dyn Control>> {
                let len = self.len();
                if len > 0 {
                    self.remove_child_from(len - 1)
                } else {
                    None
                }
            }
        }
        inner: {
            fn len(&self) -> usize;
            fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>>;
            fn remove_child_from(&mut self, base: &mut MemberBase, index: usize) -> Option<Box<dyn Control>>;
            fn child_at(&self, index: usize) -> Option<&dyn Control>;
            fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control>;

            #[inline]
            fn is_empty(&self) -> bool {
                self.len() < 1
            }
            #[inline]
            fn clear(&mut self, base: &mut MemberBase) {
                let len = self.len();
                for index in (0..len).rev() {
                    self.remove_child_from(base, index);
                }
            }
            #[inline]
            fn push_child(&mut self, base: &mut MemberBase, child: Box<dyn Control>) {
                let len = self.len();
                self.set_child_to(base, len, child);
            }
            #[inline]
            fn pop_child(&mut self, base: &mut MemberBase) -> Option<Box<dyn Control>> {
                let len = self.len();
                if len > 0 {
                    self.remove_child_from(base, len - 1)
                } else {
                    None
                }
            }
        }
    }
}
impl<T: MultiContainerInner> MaybeMultiContainer for AMember<AContainer<AMultiContainer<T>>> {
    #[inline]
    fn is_multi_container_mut(&mut self) -> Option<&mut dyn MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi_container(&self) -> Option<&dyn MultiContainer> {
        Some(self)
    }
}
impl<T: MultiContainerInner + ControlInner> MaybeMultiContainer for AMember<AControl<AContainer<AMultiContainer<T>>>> {
    #[inline]
    fn is_multi_container(&self) -> Option<&dyn MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi_container_mut(&mut self) -> Option<&mut dyn MultiContainer> {
        Some(self)
    }
}
impl<T: MultiContainerInner> MultiContainer for AMember<AContainer<AMultiContainer<T>>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&dyn Control> {
        self.inner.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control> {
        self.inner.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &dyn MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut dyn MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<dyn MultiContainer> {
        self
    }
}
impl<T: MultiContainerInner + ControlInner> MultiContainer for AMember<AControl<AContainer<AMultiContainer<T>>>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.inner.set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn Control>> {
        self.inner.inner.inner.inner.remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&dyn Control> {
        self.inner.inner.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control> {
        self.inner.inner.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &dyn MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut dyn MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<dyn MultiContainer> {
        self
    }
}
