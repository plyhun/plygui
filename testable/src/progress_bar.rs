use crate::common::{self, *};

pub type ProgressBar = AMember<AControl<AProgressBar<TestableProgressBar>>>;

#[repr(C)]
pub struct TestableProgressBar {
    base: common::TestableControlBase<ProgressBar>,
    progress: types::Progress,
}

impl HasProgressInner for TestableProgressBar {
    fn progress(&self, _: &MemberBase) -> types::Progress {
        self.progress.clone()
    }
    fn set_progress(&mut self, _base: &mut MemberBase, arg: types::Progress) {
        self.progress = arg;
        self.base.invalidate();
    }
}
impl<O: controls::ProgressBar> NewProgressBarInner<O> for TestableProgressBar {
    fn with_uninit(u: &mut mem::MaybeUninit<O>) -> Self {
        TestableProgressBar {
            base: common::TestableControlBase::with_id(u),
            progress: Default::default(),
        }
    }
}
impl ProgressBarInner for TestableProgressBar {
    fn with_progress(arg: types::Progress) -> Box<dyn controls::ProgressBar> {
        let mut b: Box<mem::MaybeUninit<ProgressBar>> = Box::new_uninit();
        let mut ab = AMember::with_inner(
            AControl::with_inner(
                AProgressBar::with_inner(
                    <Self as NewProgressBarInner<ProgressBar>>::with_uninit(b.as_mut()),
                ),
            )
        );
        controls::HasProgress::set_progress(&mut ab, arg);
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
}
impl Spawnable for TestableProgressBar {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_progress(types::Progress::None).into_control()
    }
}
impl ControlInner for TestableProgressBar {
    fn on_added_to_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, _pw: u16, _ph: u16) {
        self.base.parent = Some(unsafe { parent.native_id() as InnerId });
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
        use plygui_api::markup::MEMBER_TYPE_PROGRESS_BAR;
        fill_from_markup_base!(self, member, markup, registry, ProgressBar, [MEMBER_TYPE_PROGRESS_BAR]);
    }
}

impl HasLayoutInner for TestableProgressBar {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl HasNativeIdInner for TestableProgressBar {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.id.into()
    }
}

impl HasSizeInner for TestableProgressBar {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<ProgressBar>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();

        unsafe { utils::base_to_impl_mut::<ProgressBar>(base) }.call_on_size::<ProgressBar>(width, height);

        true
    }
}

impl HasVisibilityInner for TestableProgressBar {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl MemberInner for TestableProgressBar {}

impl Drawable for TestableProgressBar {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw("ProgressBar", control.coords, control.measured);
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;

        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width as i32,
                    layout::Size::Exact(w) => w as i32,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING as i32 + DEFAULT_PADDING + DEFAULT_PADDING,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height as i32,
                    layout::Size::Exact(h) => h as i32,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING as i32 + DEFAULT_PADDING + DEFAULT_PADDING,
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
