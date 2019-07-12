use crate::common::{self, *};

#[repr(C)]
pub struct TestableWindow {
    id: InnerId,
    child: Option<Box<dyn controls::Control>>,
    label: String,
    size: (u16, u16),
    position: (i32, i32),
    visibility: types::Visibility,
    menu: types::Menu,
    on_close: Option<callbacks::OnClose>,
}

pub type Window = Member<SingleContainer<plygui_api::development::Window<TestableWindow>>>;

impl HasLabelInner for TestableWindow {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        Cow::Borrowed(&self.label)
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        self.label = label.into();
    }
}

impl HasVisibilityInner for TestableWindow {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
    	self.visibility = value;
        true
    }
}
impl HasSizeInner for TestableWindow {
    fn on_size_set(&mut self, _base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.size = value;
        true
    }
}

impl WindowInner for TestableWindow {
    fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::Menu) -> Box<Window> {
        let mut w: Box<Window> = Box::new(Member::with_inner(
            SingleContainer::with_inner(
                plygui_api::development::Window::with_inner(
                    TestableWindow {
                        id: 0 as InnerId,
                        label: title.into(),
                        size: (0, 0),
					    position: (0, 0),
					    visibility: types::Visibility::Visible,
                        child: None,
                        menu: menu,
                        on_close: None,
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        w.as_inner_mut().as_inner_mut().as_inner_mut().id = w.base_mut();

        /*if let Some(items) = menu {
            let menu = winuser::CreateMenu();
            common::make_menu(menu, items, &mut w.as_inner_mut().as_inner_mut().as_inner_mut().menu);
            winuser::SetMenu(id, menu);
        }*/

        w
    }
    fn size(&self) -> (u16, u16) {
        self.size
    }
    fn position(&self) -> (i32, i32) {
        self.position
    }
}

impl ContainerInner for TestableWindow {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            if let Some(c) = child.is_container_mut() {
                return c.find_control_mut(arg);
            }
        }
        None
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            if let Some(c) = child.is_container() {
                return c.find_control(arg);
            }
        }
        None
    }
}

impl SingleContainerInner for TestableWindow {
    fn set_child(&mut self, _: &mut MemberBase, mut child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        use plygui_api::controls::SingleContainer;

        let mut old = self.child.take();
        if let Some(outer_self) = common::member_from_id::<Window>(self.id.into()) {
            if let Some(old) = old.as_mut() {
                let outer_self = outer_self.as_single_container_mut().as_container_mut();
                old.on_removed_from_container(outer_self);
            }
            if let Some(new) = child.as_mut() {
                let outer_self = outer_self.as_single_container_mut().as_container_mut();
                let size = self.size();
                new.on_added_to_container(outer_self, 0, 0, size.0, size.1)
            }
        }
        self.child = child;

        old
    }
    fn child(&self) -> Option<&dyn controls::Control> {
        self.child.as_ref().map(|c| c.as_ref())
    }
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            Some(child.as_mut())
        } else {
            None
        }
    }
}

impl CloseableInner for TestableWindow {
    fn close(&mut self, skip_callbacks: bool) -> bool {
    	if skip_callbacks { return true; }
    	
        if let Some(on_close) = self.on_close.as_mut() {
        	let this = common::member_from_id::<Window>(self.id).unwrap();
        	(on_close.as_mut())(this)
        } else {
        	true
        }
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
}
impl HasNativeIdInner for TestableWindow {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.id.into()
    }
}
impl MemberInner for TestableWindow {}

impl Drop for TestableWindow {
    fn drop(&mut self) {
        if let Some(self2) = common::member_from_id::<Window>(self.id) {
            self.set_child(self2.base_mut(), None);
        }
    }
}

default_impls_as!(Window);
