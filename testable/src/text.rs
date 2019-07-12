use crate::common::{self, *};

pub type Text = Member<Control<TestableText>>;

#[repr(C)]
pub struct TestableText {
    base: common::TestableControlBase<Text>,
    text: String,
}

impl HasLabelInner for TestableText {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        Cow::Borrowed(self.text.as_ref())
    }
    fn set_label(&mut self, _base: &mut MemberBase, label: Cow<str>) {
        self.text = label.into();
        self.base.invalidate();
    }
}

impl TextInner for TestableText {
    fn with_text(text: &str) -> Box<Text> {
        let b: Box<Text> = Box::new(Member::with_inner(
            Control::with_inner(
                TestableText {
                    base: common::TestableControlBase::new(),
                    text: text.to_owned(),
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b
    }
}

impl ControlInner for TestableText {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
	    self.base.parent = Some(unsafe {parent.native_id() as InnerId});
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

    fn fill_from_markup(&mut self, member: &mut MemberBase, _control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_BUTTON;
        fill_from_markup_base!(self, member, markup, registry, Text, [MEMBER_TYPE_BUTTON]);
        fill_from_markup_label!(self, member, markup);
    }
}

impl HasLayoutInner for TestableText {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl HasSizeInner for TestableText {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<Text>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        true
    }
}
impl HasVisibilityInner for TestableText {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl HasNativeIdInner for TestableText {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.id.into()
    }
}

impl MemberInner for TestableText {}

impl Drawable for TestableText {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control.coords, control.measured);
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;

        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                /*let mut label_size: windef::SIZE = unsafe { mem::zeroed() };
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => {
                        if label_size.cx < 1 {
                            let label = OsStr::new(self.text.as_str()).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
                            unsafe {
                                wingdi::GetTextExtentPointW(winuser::GetDC(self.base.hwnd), label.as_ptr(), self.text.len() as i32, &mut label_size);
                            }
                        }
                        label_size.cx as i32 + DEFAULT_PADDING + DEFAULT_PADDING
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => {
                        if label_size.cy < 1 {
                            let label = OsStr::new(self.text.as_str()).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
                            unsafe {
                                wingdi::GetTextExtentPointW(winuser::GetDC(self.base.hwnd), label.as_ptr(), self.text.len() as i32, &mut label_size);
                            }
                        }
                        label_size.cy as i32 + DEFAULT_PADDING + DEFAULT_PADDING
                    }
                };
                (cmp::max(0, w) as u16, cmp::max(0, h) as u16)*/
                (0,0)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate()
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    Text::empty().into_control()
}

default_impls_as!(Text);
