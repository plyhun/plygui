use crate::layout;

use super::auto::{AsAny, HasInner, Abstract};
use super::member::{Member, MemberBase, MemberInner};

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
        fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
            layout::BoundarySize::AllTheSame(0)
        }
    }
});

impl<II: HasLayoutInner, T: HasInner<I = II> + Abstract + 'static> HasLayoutInner for T {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner_mut().on_layout_changed(base)
    }
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner().layout_margin(member)
    }
}
