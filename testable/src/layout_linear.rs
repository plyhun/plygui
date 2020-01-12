use crate::common::{self, *};

pub type LinearLayout = AMember<AControl<AContainer<AMultiContainer<ALinearLayout<TestableLinearLayout>>>>>;

#[repr(C)]
pub struct TestableLinearLayout {
    base: TestableControlBase<LinearLayout>,
    orientation: layout::Orientation,
    children: Vec<Box<dyn controls::Control>>,
}

impl LinearLayoutInner for TestableLinearLayout {
    fn with_orientation(orientation: layout::Orientation) -> Box<dyn controls::LinearLayout> {
        let mut b = Box::new(AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AMultiContainer::with_inner(
                        ALinearLayout::with_inner(
                            TestableLinearLayout {
                                base: TestableControlBase::new(),
                                orientation: orientation,
                                children: Vec::new(),
                            }
                        ),
                    )
                ),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b.inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().base.id = &mut b.base;
        b
    }
}
impl Spawnable for TestableLinearLayout {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_orientation(layout::Orientation::Vertical).into_control()
    }
}
impl HasOrientationInner for TestableLinearLayout {
    fn orientation(&self, _: &MemberBase) -> layout::Orientation {
        self.orientation
    }
    fn set_orientation(&mut self, _base: &mut MemberBase, orientation: layout::Orientation) {
        if orientation != self.orientation {
            self.orientation = orientation;
            self.base.invalidate();
        }
    }
}
impl MultiContainerInner for TestableLinearLayout {
    fn len(&self) -> usize {
        self.children.len()
    }
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        let old = self.remove_child_from(base, index);

        self.children.insert(index, child);
        if self.base.parent.is_some() {
            let (w, h) = base.as_any().downcast_ref::<LinearLayout>().unwrap().inner().base.measured;
            self.children.get_mut(index).unwrap().on_added_to_container(
                self.base.as_outer_mut(),
                w as i32 - DEFAULT_PADDING,
                h as i32 - DEFAULT_PADDING,
                utils::coord_to_size(w as i32 - DEFAULT_PADDING),
                utils::coord_to_size(h as i32 - DEFAULT_PADDING),
            );
            self.base.invalidate();
        }
        old
    }
    fn remove_child_from(&mut self, _base: &mut MemberBase, index: usize) -> Option<Box<dyn controls::Control>> {
        if index < self.children.len() {
            let mut old = self.children.remove(index);
            if self.base.parent.is_some() {
                old.on_removed_from_container(self.base.as_outer_mut());
                self.base.invalidate();
            }
            Some(old)
        } else {
            None
        }
    }
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        self.children.get(index).map(|c| c.as_ref())
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        //self.children.get_mut(index).map(|c| c.as_mut()) //the anonymous lifetime #1 does not necessarily outlive the static lifetime
        if let Some(c) = self.children.get_mut(index) {
            Some(c.as_mut())
        } else {
            None
        }
    }
}
impl ControlInner for TestableLinearLayout {
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
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, px: i32, py: i32, pw: u16, ph: u16) {
        self.base.parent = Some(unsafe {parent.native_id() as InnerId});
        self.base.position = (px, py);
	    control.coords = Some((px as i32, py as i32));
        let mut x = DEFAULT_PADDING;
        let mut y = DEFAULT_PADDING;
        for ref mut child in self.children.as_mut_slice() {
            let self2: &mut LinearLayout = unsafe { utils::base_to_impl_mut(member) };
            child.on_added_to_container(
                self2,
                x,
                y,
                utils::coord_to_size(pw as i32 - DEFAULT_PADDING - DEFAULT_PADDING) as u16,
                utils::coord_to_size(ph as i32 - DEFAULT_PADDING - DEFAULT_PADDING) as u16,
            );
            let (xx, yy) = child.size();
            match self.orientation {
                layout::Orientation::Horizontal => x += xx as i32,
                layout::Orientation::Vertical => y += yy as i32,
            }
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {
        for ref mut child in self.children.as_mut_slice() {
            let self2: &mut LinearLayout = unsafe { utils::base_to_impl_mut(member) };
            child.on_removed_from_container(self2);
        }
        self.base.parent = None;
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, _control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_LINEAR_LAYOUT;

        fill_from_markup_base!(self, member, markup, registry, LinearLayout, [MEMBER_TYPE_LINEAR_LAYOUT]);
        fill_from_markup_children!(self, member, markup, registry);
    }
}
impl HasLayoutInner for TestableLinearLayout {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        layout::BoundarySize::AllTheSame(DEFAULT_PADDING)
    }
}
impl HasNativeIdInner for TestableLinearLayout {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.id.into()
    }
}
impl MemberInner for TestableLinearLayout {}

impl HasSizeInner for TestableLinearLayout {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<LinearLayout>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        
        unsafe { utils::base_to_impl_mut::<LinearLayout>(base) }.call_on_size::<LinearLayout>(width, height);
        
        true
    }
}

impl HasVisibilityInner for TestableLinearLayout {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl ContainerInner for TestableLinearLayout {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        for child in self.children.as_mut_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if child.as_member_mut().id() == *id {
                        return Some(child.as_mut());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = child.as_member_mut().tag() {
                        if tag.as_str() == mytag {
                            return Some(child.as_mut());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container_mut() {
                let ret = c.find_control_mut(arg.clone());
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        for child in self.children.as_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if child.as_member().id() == *id {
                        return Some(child.as_ref());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = child.as_member().tag() {
                        if tag.as_str() == mytag {
                            return Some(child.as_ref());
                        }
                    }
                }
            }
            if let Some(c) = child.is_container() {
                let ret = c.find_control(arg.clone());
                if ret.is_none() {
                    continue;
                }
                return ret;
            }
        }
        None
    }
}

impl Drawable for TestableLinearLayout {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw("LinearLayout", control.coords, control.measured);
        let mut x = DEFAULT_PADDING;
        let mut y = DEFAULT_PADDING;
        for ref mut child in self.children.as_mut_slice() {
            child.draw(Some((x, y)));
            let (xx, yy) = child.size();
            match self.orientation {
                layout::Orientation::Horizontal => x += xx as i32,
                layout::Orientation::Vertical => y += yy as i32,
            }
        }
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        use std::cmp::max;

        let orientation = self.orientation;
        let old_size = control.measured;
        let hp = DEFAULT_PADDING + DEFAULT_PADDING;
        let vp = DEFAULT_PADDING + DEFAULT_PADDING;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut measured = false;
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent => parent_width,
                    layout::Size::WrapContent => {
                        let mut w = 0;
                        for child in self.children.as_mut_slice() {
                            let (cw, _, _) = child.measure(max(0, parent_width as i32 - hp) as u16, max(0, parent_height as i32 - vp) as u16);
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    w += cw;
                                }
                                layout::Orientation::Vertical => {
                                    w = max(w, cw);
                                }
                            }
                        }
                        measured = true;
                        max(0, w as i32 + hp) as u16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent => parent_height,
                    layout::Size::WrapContent => {
                        let mut h = 0;
                        for child in self.children.as_mut_slice() {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = child.measure(max(0, parent_width as i32 - hp) as u16, max(0, parent_height as i32 - vp) as u16);
                                ch
                            };
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    h = max(h, ch);
                                }
                                layout::Orientation::Vertical => {
                                    h += ch;
                                }
                            }
                        }
                        max(0, h as i32 + vp) as u16
                    }
                };
                (w, h)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
    fn invalidate(&mut self, _member: &mut MemberBase, _control: &mut ControlBase) {
        self.base.invalidate()
    }
}

default_impls_as!(LinearLayout);
