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

pub type Window = AMember<AContainer<ASingleContainer<ACloseable<AWindow<TestableWindow>>>>>;

impl TestableWindow {
	pub fn draw(&mut self) {
		println!("Window '{}' drawn ({} px, {} px) at {:?} ({:?})", self.label, self.size.0, self.size.1, self.position, self.id);
		if let Some(ref mut child) = self.child {
			child.draw(Some((0, 0)));
		}
	}
}

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
    fn on_size_set(&mut self, base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.size = value;
        
        unsafe { utils::base_to_impl_mut::<Window>(base) }.call_on_size::<Window>(value.0, value.1);
        
        true
    }
}
impl<O: controls::Window> NewWindowInner<O> for TestableWindow {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, title: &str, window_size: types::WindowStartSize, menu: types::Menu) -> Self {
        TestableWindow {
            id: unsafe { mem::transmute(u) },
            label: title.to_owned(),
            size: match window_size {
                types::WindowStartSize::Exact(w, h) => (w, h), 
			    types::WindowStartSize::Fullscreen => (1280, 800)
            },
		    position: (0, 0),
		    visibility: types::Visibility::Visible,
            child: None,
            menu: menu,
            on_close: None,
        }
    }
}
impl WindowInner for TestableWindow {
    fn with_params<S: AsRef<str>>(app: &mut dyn controls::Application, title: S, window_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let mut b: Box<mem::MaybeUninit<Window>> = Box::new_uninit();
        let ab = AMember::with_inner(
            AContainer::with_inner(
                ASingleContainer::with_inner(
                    ACloseable::with_inner(
                        AWindow::with_inner(
                            <Self as NewWindowInner<Window>>::with_uninit_params(b.as_mut(), title.as_ref(), window_size, menu),
    	                ),
                        unsafe { app.native_id() }
                    )
                )
            )
        );
        /*if let Some(items) = menu {
            let menu = winuser::CreateMenu();
            common::make_menu(menu, items, &mut w.inner_mut().inner_mut().inner_mut().menu);
            winuser::SetMenu(id, menu);
        }*/
		unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
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
        use crate::plygui_api::controls::SingleContainer;
        
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
    	use crate::plygui_api::controls::Member;
        
        let this = common::member_from_id::<Window>(self.id).unwrap();
        let id = this.id();
        this.inner_mut().inner_mut().inner_mut().application_impl_mut::<crate::application::Application>().close_root(types::FindBy::Id(id), skip_callbacks);
        
        println!("Window '{}' closed ({:?})", self.label, self.id);
        true
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
    fn application<'a>(&'a self, base: &'a MemberBase) -> &'a dyn controls::Application {
        unsafe { utils::base_to_impl::<Window>(base) }.inner().inner().inner().application_impl::<crate::application::Application>()
    }
    fn application_mut<'a>(&'a mut self, base: &'a mut MemberBase) -> &'a mut dyn controls::Application {
        unsafe { utils::base_to_impl_mut::<Window>(base) }.inner_mut().inner_mut().inner_mut().application_impl_mut::<crate::application::Application>()
    }
}
impl HasNativeIdInner for TestableWindow {
    type Id = common::TestableId;

    fn native_id(&self) -> Self::Id {
        self.id.into()
    }
}
impl MemberInner for TestableWindow {}

impl Drop for TestableWindow {
    fn drop(&mut self) {
        if let Some(self2) = common::member_from_id::<Window>(self.id) {
            self.set_child(&mut self2.base, None);
        }
    }
}
