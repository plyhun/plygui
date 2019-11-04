use super::seal::Sealed;
use super::member::MemberBase;
use super::control::ControlBase;
use super::auto::HasInner;

pub trait OuterDrawable: Sealed {
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
    fn invalidate(&mut self);
    fn set_skip_draw(&mut self, skip: bool);
    fn is_skip_draw(&self) -> bool;

    fn as_drawable(&self) -> &dyn OuterDrawable;
    fn as_drawable_mut(&mut self) -> &mut dyn OuterDrawable;
    fn into_drawable(self: Box<Self>) -> Box<dyn OuterDrawable>;
}

pub trait Drawable: Sized + 'static {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase);
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool);
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase);
}

impl<II: Drawable, T: HasInner<I=II> + Drawable + 'static> Drawable for T {
    #[inline]
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner_mut().draw(member, control)
    }
    #[inline]
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner_mut().measure(member, control, w, h)
    }
    #[inline]
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner_mut().invalidate(member, control)
    }
}
