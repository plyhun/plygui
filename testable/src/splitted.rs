use crate::common::{self, *};

const DEFAULT_BOUND: i32 = DEFAULT_PADDING;
const HALF_BOUND: i32 = DEFAULT_BOUND / 2;

pub type Splitted = AMember<AControl<AContainer<AMultiContainer<ASplitted<TestableSplitted>>>>>;

#[repr(C)]
pub struct TestableSplitted {
    base: common::TestableControlBase<Splitted>,

    orientation: layout::Orientation,

    splitter: f32,
    moving: bool,
    
    first: Box<dyn controls::Control>,
    second: Box<dyn controls::Control>,
}

impl TestableSplitted {
    fn children_sizes(&self, base: &ControlBase) -> (u16, u16) {
        let (w, h) = base.measured;
        let target = match self.orientation {
            layout::Orientation::Horizontal => w,
            layout::Orientation::Vertical => h,
        };
        (
            utils::coord_to_size((target as f32 * self.splitter) as i32 - DEFAULT_PADDING - HALF_BOUND),
            utils::coord_to_size((target as f32 * (1.0 - self.splitter)) as i32 - DEFAULT_PADDING - HALF_BOUND),
        )
    }
    fn draw_children(&mut self) {
        let mut x = DEFAULT_PADDING;
        let mut y = DEFAULT_PADDING;
        for ref mut child in [self.first.as_mut(), self.second.as_mut()].iter_mut() {
            child.draw(Some((x, y)));
            let (xx, yy) = child.size();
            match self.orientation {
                layout::Orientation::Horizontal => {
                    x += xx as i32;
                    x += DEFAULT_BOUND;
                }
                layout::Orientation::Vertical => {
                    y += yy as i32;
                    y += DEFAULT_BOUND;
                }
            }
        }
    }
    fn update_children_layout(&mut self, base: &ControlBase) {
        if self.base.parent.is_none() {
            return;
        }

        let orientation = self.orientation;
        let (first_size, second_size) = self.children_sizes(base);
        let (width, height) = base.measured;
        for (size, child) in [(first_size, self.first.as_mut()), (second_size, self.second.as_mut())].iter_mut() {
            match orientation {
                layout::Orientation::Horizontal => {
                    child.measure(cmp::max(0, *size) as u16, cmp::max(0, height as i32 - DEFAULT_PADDING - DEFAULT_PADDING) as u16);
                }
                layout::Orientation::Vertical => {
                    child.measure(cmp::max(0, width as i32 - DEFAULT_PADDING - DEFAULT_PADDING) as u16, cmp::max(0, *size) as u16);
                }
            }
        }
    }
}

impl SplittedInner for TestableSplitted {
    fn with_content(first: Box<dyn controls::Control>, second: Box<dyn controls::Control>, orientation: layout::Orientation) -> Box<dyn controls::Splitted> {
        let mut b = Box::new(AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AMultiContainer::with_inner(
                        ASplitted::with_inner(
                            TestableSplitted {
                                base: common::TestableControlBase::new(),
                                orientation: orientation,
        
                                splitter: 0.5,
                                moving: false,
        
                                first: first,
                                second: second,
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
    fn set_splitter(&mut self, _member: &mut MemberBase, pos: f32) {
        self.splitter = pos;
        self.base.invalidate();
    }
    fn splitter(&self) -> f32 {
        self.splitter
    }
    fn first(&self) -> &dyn controls::Control {
        self.first.as_ref()
    }
    fn second(&self) -> &dyn controls::Control {
        self.second.as_ref()
    }
    fn first_mut(&mut self) -> &mut dyn controls::Control {
        self.first.as_mut()
    }
    fn second_mut(&mut self) -> &mut dyn controls::Control {
        self.second.as_mut()
    }
}
impl Spawnable for TestableSplitted {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_content(super::text::TestableText::spawn(), super::text::TestableText::spawn(), layout::Orientation::Vertical).into_control()
    }
}
impl HasNativeIdInner for TestableSplitted {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.id.into()
    }
}

impl HasSizeInner for TestableSplitted {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<Splitted>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        
        unsafe { utils::base_to_impl_mut::<Splitted>(base) }.call_on_size(width, height);
        
        true
    }
}
impl HasVisibilityInner for TestableSplitted {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl MemberInner for TestableSplitted {}

impl ControlInner for TestableSplitted {
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
	    control.measured = (pw, ph); // for the measurement sake
        let (width, height, _) = self.measure(member, control, pw, ph);
        control.coords = Some((px as i32, py as i32));
        
        let self2: &mut Splitted = unsafe { mem::transmute(member) };
        let (first_size, second_size) = self.children_sizes(control);

        match self.orientation {
            layout::Orientation::Horizontal => {
                let h = utils::coord_to_size(height as i32 - DEFAULT_PADDING - DEFAULT_PADDING);
                self.first.on_added_to_container(self2, DEFAULT_PADDING, DEFAULT_PADDING, first_size, h);
                self.second.on_added_to_container(self2, DEFAULT_PADDING + DEFAULT_BOUND + first_size as i32, DEFAULT_PADDING, second_size, h);
            }
            layout::Orientation::Vertical => {
                let w = utils::coord_to_size(width as i32 - DEFAULT_PADDING - DEFAULT_PADDING);
                self.first.on_added_to_container(self2, DEFAULT_PADDING, DEFAULT_PADDING, w, first_size);
                self.second.on_added_to_container(self2, DEFAULT_PADDING, DEFAULT_PADDING + DEFAULT_BOUND + first_size as i32, w, second_size);
            }
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {
        let self2: &mut Splitted = unsafe { utils::base_to_impl_mut(member) };

        self.first.on_removed_from_container(self2);
        self.second.on_removed_from_container(self2);
        
        self.base.parent = None;
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, _control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_SPLITTED;

        fill_from_markup_base!(self, member, markup, registry, Splitted, [MEMBER_TYPE_SPLITTED]);
        fill_from_markup_children!(self, member, markup, registry);
    }
}

impl HasLayoutInner for TestableSplitted {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        //self.update_children_layout();
        self.base.invalidate();
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        layout::BoundarySize::AllTheSame(DEFAULT_PADDING)
    }
}

impl ContainerInner for TestableSplitted {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.first().as_member().id() == id {
                    return Some(self.first_mut());
                }
                if self.second().as_member().id() == id {
                    return Some(self.second_mut());
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.first.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.first_mut());
                    }
                }
                if let Some(mytag) = self.second.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.second_mut());
                    }
                }
            }
        }

        let self2: &mut TestableSplitted = unsafe { mem::transmute(self as *mut TestableSplitted) }; // bck is stupid
        if let Some(c) = self.first_mut().is_container_mut() {
            let ret = c.find_control_mut(arg.clone());
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self2.second_mut().is_container_mut() {
            let ret = c.find_control_mut(arg);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.first().as_member().id() == id {
                    return Some(self.first());
                }
                if self.second().as_member().id() == id {
                    return Some(self.second());
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.first.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.first.as_ref());
                    }
                }
                if let Some(mytag) = self.second.as_member().tag() {
                    if tag.as_str() == mytag {
                        return Some(self.second.as_ref());
                    }
                }
            }
        }
        if let Some(c) = self.first().is_container() {
            let ret = c.find_control(arg.clone());
            if ret.is_some() {
                return ret;
            }
        }
        if let Some(c) = self.second().is_container() {
            let ret = c.find_control(arg);
            if ret.is_some() {
                return ret;
            }
        }
        None
    }
}

impl MultiContainerInner for TestableSplitted {
    fn len(&self) -> usize {
        2
    }
    fn set_child_to(&mut self, _: &mut MemberBase, index: usize, mut child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        match index {
            0 => {
                if self.base.parent.is_some() {
                    let self2 = self.base.as_outer_mut();
                    let sizes = self.first.size();
                    self.first.on_removed_from_container(self2);
                    child.on_added_to_container(self2, DEFAULT_PADDING, DEFAULT_PADDING, sizes.0, sizes.1);
                }
                mem::swap(&mut self.first, &mut child);
            }
            1 => {
                if self.base.parent.is_some() {
                    let self2 = self.base.as_outer_mut();

                    let mut x = DEFAULT_PADDING;
                    let mut y = DEFAULT_PADDING;

                    let (xx, yy) = self.first.size();
                    match self.orientation {
                        layout::Orientation::Horizontal => {
                            x += xx as i32;
                            x += DEFAULT_BOUND;
                        }
                        layout::Orientation::Vertical => {
                            y += yy as i32;
                            y += DEFAULT_BOUND;
                        }
                    }
                    let sizes = self.second.size();
                    self.second.on_removed_from_container(self2);
                    child.on_added_to_container(self2, x, y, sizes.0, sizes.1);
                }
                mem::swap(&mut self.second, &mut child);
            }
            _ => return None,
        }

        Some(child)
    }
    fn remove_child_from(&mut self, _: &mut MemberBase, _: usize) -> Option<Box<dyn controls::Control>> {
        None
    }
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        match index {
            0 => Some(self.first()),
            1 => Some(self.second()),
            _ => None,
        }
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        match index {
            0 => Some(self.first_mut()),
            1 => Some(self.second_mut()),
            _ => None,
        }
    }
}

impl HasOrientationInner for TestableSplitted {
    fn orientation(&self, _: &MemberBase) -> layout::Orientation {
        self.orientation
    }
    fn set_orientation(&mut self, base: &mut MemberBase, orientation: layout::Orientation) {
        if orientation != self.orientation {
            self.orientation = orientation;
            self.update_children_layout(& (unsafe { utils::base_to_impl_mut::<Splitted>(base) }.inner()).base);
            self.base.invalidate();
        }
    }
}

impl Drawable for TestableSplitted {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw("Splitted", control.coords, control.measured);
        self.draw_children();
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        use std::cmp::max;

        let orientation = self.orientation;
        let old_size = control.measured;
        let hp = DEFAULT_PADDING + DEFAULT_PADDING + if orientation == layout::Orientation::Horizontal { DEFAULT_BOUND } else { 0 };
        let vp = DEFAULT_PADDING + DEFAULT_PADDING + if orientation == layout::Orientation::Vertical { DEFAULT_BOUND } else { 0 };
        let (first_size, second_size) = self.children_sizes(control);
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let mut measured = false;
                let w = match control.layout.width {
                    layout::Size::Exact(w) => w,
                    layout::Size::MatchParent => parent_width,
                    layout::Size::WrapContent => {
                        let mut w = 0;
                        for (size, child) in [(first_size, self.first.as_mut()), (second_size, self.second.as_mut())].iter_mut() {
                            match orientation {
                                layout::Orientation::Horizontal => {
                                    let (cw, _, _) = child.measure(max(0, *size) as u16, max(0, parent_height as i32 - vp) as u16);
                                    w += cw;
                                }
                                layout::Orientation::Vertical => {
                                    let (cw, _, _) = child.measure(max(0, parent_width as i32 - hp) as u16, max(0, *size) as u16);
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
                        for (size, child) in [(first_size, self.first.as_mut()), (second_size, self.second.as_mut())].iter_mut() {
                            let ch = if measured {
                                child.size().1
                            } else {
                                let (_, ch, _) = match orientation {
                                    layout::Orientation::Horizontal => child.measure(max(0, *size) as u16, max(0, parent_height as i32 - vp) as u16),
                                    layout::Orientation::Vertical => child.measure(max(0, parent_width as i32 - hp) as u16, max(0, *size) as u16),
                                };
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

default_impls_as!(Splitted);
