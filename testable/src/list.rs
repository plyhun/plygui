use crate::common::{self, *};

pub type List = AMember<AControl<AContainer<AAdapted<AList<TestableList>>>>>;

#[repr(C)]
pub struct TestableList {
    base: TestableControlBase<List>,
    items: Vec<Box<dyn controls::Control>>,
    on_item_click: Option<callbacks::OnItemClick>,
}

impl ListInner for TestableList {
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn controls::List> {
        let b = Box::new(AMember::with_inner(
            AControl::with_inner(
                AContainer::with_inner(
                    AAdapted::with_inner(
                        AList::with_inner(
                            TestableList {
                                base: TestableControlBase::new(),
                                items: Vec::with_capacity(adapter.len()),
                                on_item_click: None,
                            }
                        ),
                        adapter,
                    ),
                )
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b
    }
}
impl ItemClickableInner for TestableList {
    fn item_click(&mut self, i: usize, item_view: &mut dyn controls::Control, skip_callbacks: bool) {
        if !skip_callbacks{
            let self2 = self.base.as_outer_mut();
            if let Some(ref mut callback) = self.on_item_click {
                (callback.as_mut())(self2, i, item_view)
            }
        }
    }
    fn on_item_click(&mut self, callback: Option<callbacks::OnItemClick>) {
        self.on_item_click = callback;
    }
}
impl AdaptedInner for TestableList {
    fn on_item_change(&mut self, _base: &mut MemberBase, value: types::Change) {
        match value {
            types::Change::Added(index) => {
                println!("item added {}", index);
            },
            types::Change::Removed(index) => {
                println!("item removed {}", index);
            },
            types::Change::Edited(index) => {
                println!("item edited {}", index);
            }
        }
    }
}
impl Spawnable for TestableList {
    fn spawn() -> Box<dyn controls::Control> {
        Self::with_adapter(Box::new(types::imp::StringVecAdapter::<crate::imp::Text>::new())).into_control()
    }
}
impl ControlInner for TestableList {
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
    fn on_added_to_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, parent: &dyn controls::Container, px: i32, py: i32, pw: u16, ph: u16) {
        self.base.parent = Some(unsafe {parent.native_id() as InnerId});
	    self.base.position = (px, py);

        let (member, _, adapter, _) = List::as_adapted_parts_mut(unsafe { utils::base_to_impl_mut(member) });

        let mut y = 0;
        for i in 0..adapter.adapter.len() {
            let self2: &mut List = unsafe { utils::base_to_impl_mut(member) };
            let mut item = adapter.adapter.spawn_item_view(i, self2);
            item.on_added_to_container(self2, 0, y, utils::coord_to_size(pw as i32) as u16, utils::coord_to_size(ph as i32) as u16);
            let (_, yy) = item.size();
            self.items.push(item);
            y += yy as i32;
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {
        for ref mut item in self.items.as_mut_slice() {
            let self2: &mut List = unsafe { utils::base_to_impl_mut(member) };
            item.on_removed_from_container(self2);
        }
        self.base.parent = None;
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, _control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_TABLE;

        fill_from_markup_base!(self, member, markup, registry, List, [MEMBER_TYPE_TABLE]);
        //fill_from_markup_items!(self, member, markup, registry);
    }
}
impl ContainerInner for TestableList {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        for item in self.items.as_mut_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if item.as_member_mut().id() == *id {
                        return Some(item.as_mut());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = item.as_member_mut().tag() {
                        if tag.as_str() == mytag {
                            return Some(item.as_mut());
                        }
                    }
                }
            }
            if let Some(c) = item.is_container_mut() {
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
        for item in self.items.as_slice() {
            match arg {
                types::FindBy::Id(ref id) => {
                    if item.as_member().id() == *id {
                        return Some(item.as_ref());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = item.as_member().tag() {
                        if tag.as_str() == mytag {
                            return Some(item.as_ref());
                        }
                    }
                }
            }
            if let Some(c) = item.is_container() {
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
impl HasLayoutInner for TestableList {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}
impl HasNativeIdInner for TestableList {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.id.into()
    }
}
impl MemberInner for TestableList {}

impl HasSizeInner for TestableList {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<List>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        true
    }
}

impl HasVisibilityInner for TestableList {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl Drawable for TestableList {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw("List", control.coords, control.measured);
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
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

default_impls_as!(List);
