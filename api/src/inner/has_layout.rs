use crate::layout;

use super::auto::{HasInner, AsAny};
use super::member::{Member, AMember, MemberInner, MemberBase};
use super::control::{ControlInner, AControl};

has_private!(Layout(layout::Size, layout::Size): Member {
    outer: {
        fn layout_width(&self) -> layout::Size;
        fn layout_height(&self) -> layout::Size;
        fn set_layout_width(&mut self, value: layout::Size);
        fn set_layout_height(&mut self, value: layout::Size);
        fn layout_margin(&self) -> layout::BoundarySize;
    },
    inner: {
        fn on_layout_changed(&mut self, base: &mut MemberBase);
        fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize;
    }
});

impl<T: ControlInner> HasLayout for AMember<AControl<T>> {
    fn layout(&self) -> (layout :: Size, layout :: Size) {
        (self.inner.base.layout.width, self.inner.base.layout.height)
    }
    fn set_layout(&mut self, width: layout::Size, height: layout::Size) {
        self.inner.base.layout.width = width;
        self.inner.base.layout.width = height;
    }
    
    #[inline]
    fn layout_width(&self) -> layout::Size {
        self.inner.base.layout.width
    }
    #[inline]
    fn layout_height(&self) -> layout::Size {
        self.inner.base.layout.height
    }
    #[inline]
    fn layout_margin(&self) -> layout::BoundarySize {
        self.inner.inner.layout_margin(&self.base)
    }

    #[inline]
    fn set_layout_width(&mut self, value: layout::Size) {
        self.inner.base.layout.width = value;
        self.inner.inner.on_layout_changed(&mut self.base);
    }
    #[inline]
    fn set_layout_height(&mut self, value: layout::Size) {
        self.inner.base.layout.height = value;
        self.inner.inner.on_layout_changed(&mut self.base);
    }

    #[inline]
    fn as_has_layout(&self) -> &dyn HasLayout {
        self
    }
    #[inline]
    fn as_has_layout_mut(&mut self) -> &mut dyn HasLayout {
        self
    }
    #[inline]
    fn into_has_layout(self: Box<Self>) -> Box<dyn HasLayout> {
        self
    }
}
impl<II: HasLayoutInner, T: HasInner<I=II> + 'static> HasLayoutInner for T {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner_mut().on_layout_changed(base)
    }
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner().layout_margin(member)
    }
}
