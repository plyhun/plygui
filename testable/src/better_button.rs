use crate::common::{self, *};
use super::button::TestableButton;

pub type BetterButton = AMember<AControl<AButton<TestableBetterButton>>>;

#[repr(C)]
pub struct TestableBetterButton {
    inner: TestableButton,
}

impl HasLabelInner for TestableBetterButton {
    fn label<'a>(&'a self, base: &MemberBase) -> Cow<'a, str> {
        self.inner.label(base)
    }
    fn set_label(&mut self, base: &mut MemberBase, label: Cow<str>) {
        self.inner.set_label(base, label)
    }
}

impl ClickableInner for TestableBetterButton {
    fn on_click(&mut self, handle: Option<callbacks::OnClick>) {
        self.inner.on_click(handle)
    }
    fn click(&mut self, skip_callbacks: bool) {
        self.inner.click(skip_callbacks)
    }
}

impl ButtonInner for TestableBetterButton {
    fn with_label<S: AsRef<str>>(label: S) -> Box<dyn controls::Button> {
    	let mut b: Box<mem::MaybeUninit<BetterButton>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AButton::with_inner(
                    TestableBetterButton {
                        inner: <TestableButton as NewButtonInner<BetterButton>>::with_uninit(b.as_mut())
                    }
                ),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        );
        controls::HasLabel::set_label(&mut ab, label.as_ref().into());
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}
impl Spawnable for TestableBetterButton {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_label("").into_control()
    }
}
impl ControlInner for TestableBetterButton {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
	    self.inner.on_added_to_container(member, control, parent,x,y,pw,ph)
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container) {
	    self.inner.on_removed_from_container(member, control, parent)
    }
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.inner.parent()
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.parent_mut()
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.inner.root()
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, _control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_BUTTON;
        fill_from_markup_base!(self, member, markup, registry, Button, [MEMBER_TYPE_BUTTON]);
        fill_from_markup_label!(self, member, markup);
        fill_from_markup_callbacks!(self, markup, registry, [on_click => plygui_api::callbacks::OnClick]);
    }
}

impl HasLayoutInner for TestableBetterButton {
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base);
    }
}

impl HasNativeIdInner for TestableBetterButton {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}

impl HasSizeInner for TestableBetterButton {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<BetterButton>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        let (m, c, _) = this.as_control_parts_mut();
        self.inner.invalidate(m, c);
        
        unsafe { utils::base_to_impl_mut::<BetterButton>(base) }.call_on_size(width, height);
        
        true
    }
}

impl HasVisibilityInner for TestableBetterButton {
    fn on_visibility_set(&mut self, base: &mut MemberBase, value: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, value)
    }
}

impl MemberInner for TestableBetterButton {}

impl Drawable for TestableBetterButton {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.base.draw(format!("BetterButton '{}'", self.label(member)).as_str(), control.coords, control.measured);
    }
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
		self.inner.measure(member, control, parent_width, parent_height)
    }
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
default_impls_as!(BetterButton);
