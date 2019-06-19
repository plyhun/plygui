use crate::common::{self, *};

#[repr(C)]
pub struct TestableWindow {
    id: InnerId,
    child: Option<Box<dyn controls::Control>>,
    label: String,
    size: (u16, u16),
    position: (i32, i32),
    visibility: types::Visibility,
    menu: Vec<callbacks::Action>,
    on_close: Option<callbacks::OnClose>,
}

pub type Window = Member<SingleContainer<plygui_api::development::Window<TestableWindow>>>;

impl HasLabelInner for TestableWindow {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        self.label.into()
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
        //common::draw(self.id, Some(common::pos_id(self.id)), value)
        self.size = value;
        true
    }
}

impl WindowInner for TestableWindow {
    fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::Menu) -> Box<Window> {
        unsafe {
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
                            menu: if menu.is_some() { Vec::new() } else { vec![] },
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
        if let Some(on_close) = self.on_close {
        	
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
        destroy_id(self.id, 0, None);
    }
}

unsafe fn register_window_class() -> Vec<u16> {
    let class_name = OsStr::new("PlyguiWin32Window").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();

    let class = winuser::WNDCLASSEXW {
        cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as minwindef::UINT,
        style: winuser::CS_DBLCLKS,
        lpfnWndProc: Some(handler),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: libloaderapi::GetModuleHandleW(ptr::null()),
        hIcon: winuser::LoadIconW(ptr::null_mut(), winuser::IDI_APPLICATION),
        hCursor: winuser::LoadCursorW(ptr::null_mut(), winuser::IDC_ARROW),
        hbrBackground: (winuser::COLOR_BTNFACE + 1) as windef::HBRUSH,
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut(),
    };
    winuser::RegisterClassExW(&class);
    class_name
}

unsafe extern "system" fn handler(id: InnerId, msg: minwindef::UINT, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    let ww = winuser::GetWindowLongPtrW(id, winuser::GWLP_USERDATA);
    if ww == 0 {
        if winuser::WM_CREATE == msg {
            let cs: &mut winuser::CREATESTRUCTW = mem::transmute(lparam);
            winuser::SetWindowLongPtrW(id, winuser::GWLP_USERDATA, cs.lpCreateParams as isize);
        }
        return winuser::DefWindowProcW(id, msg, wparam, lparam);
    }

    match msg {
        winuser::WM_SIZE => {
            let width = minwindef::LOWORD(lparam as u32);
            let height = minwindef::HIWORD(lparam as u32);
            let w: &mut Window = mem::transmute(ww);

            w.as_inner_mut().as_inner_mut().as_inner_mut().redraw();

            winuser::InvalidateRect(w.as_inner().as_inner().as_inner().id, ptr::null_mut(), minwindef::TRUE);

            w.call_on_size(width, height);
            return 0;
        }
        winuser::WM_DESTROY => {
            let w: &mut Window = mem::transmute(ww);
            w.as_inner_mut().as_inner_mut().as_inner_mut().id = ptr::null_mut();
            //return 0;
        }
        winuser::WM_CLOSE => {
            let w: &mut Window = mem::transmute(ww);
            if !w.as_inner_mut().as_inner_mut().as_inner_mut().skip_callbacks {
                if let Some(ref mut on_close) = w.as_inner_mut().as_inner_mut().as_inner_mut().on_close {
                    let w2: &mut Window = mem::transmute(ww);
                    if !(on_close.as_mut())(w2) {
                        return 0;
                    }
                }
            }
        }
        winuser::WM_COMMAND => {
            let id = minwindef::LOWORD(wparam as u32);
            let _evt = minwindef::HIWORD(wparam as u32);
            let w: &mut Window = mem::transmute(ww);
            let w2: &mut Window = mem::transmute(ww);
            if let Some(a) = w.as_inner_mut().as_inner_mut().as_inner_mut().menu.get_mut(id as usize) {
                (a.as_mut())(w2);
            }
        }
        _ => {}
    }
    winuser::DefWindowProcW(id, msg, wparam, lparam)
}

default_impls_as!(Window);
