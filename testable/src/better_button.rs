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
        let mut b = Box::new(AMember::with_inner(
            AControl::with_inner(
                AButton::with_inner(
                    TestableBetterButton {
                        base: common::TestableControlBase::new(),
                        h_left_clicked: None,
                        label: label.as_ref().to_owned(),
                    }
                ),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b.inner_mut().inner_mut().inner_mut().base.id = &mut b.base;
        b
    }
}
impl Spawnable for TestableBetterButton {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_label("").into_control()
    }
}
impl ControlInner for TestableBetterButton {
    fn on_added_to_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, _pw: u16, _ph: u16) {
	    self.base.parent = Some(unsafe {parent.native_id() as InnerId});
	    self.base.position = (x, y);
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {
	    self.base.parent = None;
    }
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.base.parent().map(|p| p.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.parent_mut().map(|p| p.as_member_mut())
    }
    fn root(&self) -> Option<&dyn controls::Member> {
        self.base.root().map(|p| p.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.base.root_mut().map(|p| p.as_member_mut())
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
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl HasNativeIdInner for TestableBetterButton {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.id.into()
    }
}

impl HasSizeInner for TestableBetterButton {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<Button>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        
        unsafe { utils::base_to_impl_mut::<Button>(base) }.call_on_size(width, height);
        
        true
    }
}

impl HasVisibilityInner for TestableBetterButton {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl MemberInner for TestableBetterButton {}

impl Drawable for TestableBetterButton {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(format!("Button '{}'", self.label).as_str(), control.coords, control.measured);
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;

        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let label_size = (self.label.len(), 1);
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        label_size.0 as i32 + DEFAULT_PADDING + DEFAULT_PADDING
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        label_size.1 as i32 + DEFAULT_PADDING + DEFAULT_PADDING
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate()
    }
}
