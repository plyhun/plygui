use crate::common::{self, *};

pub type Image = Member<Control<TestableImage>>;

#[repr(C)]
pub struct TestableImage {
    base: TestableControlBase<Image>,

    bmp: Option<image::DynamicImage>,
    scale: types::ImageScalePolicy,
}

impl TestableImage {
    fn install_image(&mut self, content: image::DynamicImage) {
		self.bmp = Some(content);
    }
    fn remove_image(&mut self) {
        self.bmp = None;
    }
}

impl Drop for TestableImage {
    fn drop(&mut self) {
        self.remove_image();
    }
}

impl ImageInner for TestableImage {
    fn with_content(content: image::DynamicImage) -> Box<dyn controls::Image> {
        let mut i = Box::new(Member::with_inner(
            Control::with_inner(
                TestableImage {
                    base: TestableControlBase::new(),

                    bmp: None,
                    scale: types::ImageScalePolicy::FitCenter,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        i.as_inner_mut().as_inner_mut().install_image(content);
        i
    }
    fn set_scale(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, policy: types::ImageScalePolicy) {
        if self.scale != policy {
            self.scale = policy;
            self.base.invalidate();
        }
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.scale
    }
}

impl ControlInner for TestableImage {
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

    fn fill_from_markup(&mut self, member: &mut MemberBase, _control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_IMAGE;
        fill_from_markup_base!(self, member, markup, registry, Image, [MEMBER_TYPE_IMAGE]);
        //TODO image source
    }
}

impl HasLayoutInner for TestableImage {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl HasNativeIdInner for TestableImage {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.id.into()
    }
}

impl HasSizeInner for TestableImage {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<Image>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        true
    }
}

impl HasVisibilityInner for TestableImage {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl MemberInner for TestableImage {}

impl Drawable for TestableImage {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control.coords, control.measured);
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
            	use crate::plygui_api::external::image::GenericImageView;
            	
            	let w = match control.layout.width {
                    layout::Size::MatchParent => w,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        if let Some(ref bmp) = self.bmp {bmp.dimensions().0 as u16 } else { 0 }
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => h,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => {
                        if let Some(ref bmp) = self.bmp {bmp.dimensions().1 as u16 } else { 0 }
                    }
                };
                (cmp::max(0, w as i32) as u16, cmp::max(0, h as i32) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate()
    }
}

/*
#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    use super::NewImage;

    Image::with_content().into_control()
}
*/

fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
/*fn fmax(a: f32, b: f32) -> f32 {
    // leave for future non-centered fit
    if a > b {
        a
    } else {
        b
    }
}*/

default_impls_as!(Image);
