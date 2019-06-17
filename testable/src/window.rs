use crate::common::{self, *};

lazy_static! {
    pub static ref WINDOW_CLASS: Vec<u16> = unsafe { register_window_class() };
}

#[repr(C)]
pub struct TestableWindow {
    hwnd: windef::HWND,
    msg: winuser::MSG,
    child: Option<Box<dyn controls::Control>>,
    menu: Vec<callbacks::Action>,
    on_close: Option<callbacks::OnClose>,
    skip_callbacks: bool,
}

pub type Window = Member<SingleContainer<plygui_api::development::Window<TestableWindow>>>;

impl TestableWindow {
    pub(crate) fn dispatch(&mut self) -> i32 {
        let ret = unsafe { winuser::PeekMessageW(&mut self.msg, ptr::null_mut(), 0, 0, winuser::PM_REMOVE) };
        if ret > 0 {
            unsafe {
                winuser::TranslateMessage(&mut self.msg);
                winuser::DispatchMessageW(&mut self.msg);
            }
        }
        ret
    }
    fn size_inner(&self) -> (u16, u16) {
        let rect = unsafe { window_rect(self.hwnd) };
        ((rect.right - rect.left) as u16, (rect.bottom - rect.top) as u16)
    }
    fn redraw(&mut self) {
        let size = self.size_inner();
        if let Some(ref mut child) = self.child {
            child.measure(size.0, size.1);
            child.draw(Some((0, 0)));
        }
    }
}

impl HasLabelInner for TestableWindow {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        if self.hwnd != 0 as windef::HWND {
            let mut wbuffer = vec![0u16; 4096];
            let len = unsafe { winuser::GetWindowTextW(self.hwnd, wbuffer.as_mut_slice().as_mut_ptr(), 4096) };
            Cow::Owned(String::from_utf16_lossy(&wbuffer.as_slice()[..len as usize]))
        } else {
            unreachable!();
        }
    }
    fn set_label(&mut self, _: &mut MemberBase, label: Cow<str>) {
        if self.hwnd != 0 as windef::HWND {
            let control_name = OsStr::new(label.as_ref()).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
            unsafe {
                winuser::SetWindowTextW(self.hwnd, control_name.as_ptr());
            }
        }
    }
}

impl HasVisibilityInner for TestableWindow {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        unsafe {
            winuser::ShowWindow(self.hwnd, if value == types::Visibility::Visible { winuser::SW_SHOW } else { winuser::SW_HIDE });
        }
        true
    }
}
impl HasSizeInner for TestableWindow {
    fn on_size_set(&mut self, _base: &mut MemberBase, value: (u16, u16)) -> bool {
        common::draw(self.hwnd, Some(common::pos_hwnd(self.hwnd)), value)
    }
}

impl WindowInner for TestableWindow {
    fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::Menu) -> Box<Window> {
        unsafe {
            let mut rect = match window_size {
                types::WindowStartSize::Exact(width, height) => windef::RECT {
                    left: 0,
                    top: 0,
                    right: width as i32,
                    bottom: height as i32,
                },
                types::WindowStartSize::Fullscreen => {
                    let mut rect = windef::RECT { left: 0, right: 0, top: 0, bottom: 0 };
                    if winuser::SystemParametersInfoW(winuser::SPI_GETWORKAREA, 0, &mut rect as *mut _ as *mut c_void, 0) == 0 {
                        log_error();
                        windef::RECT { left: 0, top: 0, right: 640, bottom: 480 }
                    } else {
                        windef::RECT {
                            left: 0,
                            top: 0,
                            right: rect.right,
                            bottom: rect.bottom,
                        }
                    }
                }
            };
            let style = winuser::WS_OVERLAPPEDWINDOW;
            let exstyle = winuser::WS_EX_APPWINDOW | winuser::WS_EX_COMPOSITED;

            winuser::AdjustWindowRectEx(&mut rect, style, minwindef::FALSE, exstyle);
            let window_name = OsStr::new(title).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();

            let mut w: Box<Window> = Box::new(Member::with_inner(
                SingleContainer::with_inner(
                    plygui_api::development::Window::with_inner(
                        TestableWindow {
                            hwnd: 0 as windef::HWND,
                            msg: mem::zeroed(),
                            child: None,
                            menu: if menu.is_some() { Vec::new() } else { vec![] },
                            on_close: None,
                            skip_callbacks: false,
                        },
                        (),
                    ),
                    (),
                ),
                MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
            ));

            let hwnd = winuser::CreateWindowExW(
                exstyle,
                WINDOW_CLASS.as_ptr(),
                window_name.as_ptr() as ntdef::LPCWSTR,
                style | winuser::WS_VISIBLE | winuser::CS_HREDRAW | winuser::CS_VREDRAW,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT,
                rect.right - rect.left,
                rect.bottom - rect.top,
                ptr::null_mut(),
                ptr::null_mut(),
                hinstance(),
                w.as_mut() as *mut _ as *mut c_void,
            );
            w.as_inner_mut().as_inner_mut().as_inner_mut().hwnd = hwnd;

            if let Some(items) = menu {
                let menu = winuser::CreateMenu();
                common::make_menu(menu, items, &mut w.as_inner_mut().as_inner_mut().as_inner_mut().menu);
                winuser::SetMenu(hwnd, menu);
            }

            w
        }
    }
    fn size(&self) -> (u16, u16) {
        common::size_hwnd(self.hwnd)
    }
    fn position(&self) -> (i32, i32) {
        common::pos_hwnd(self.hwnd)
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
        if let Some(outer_self) = common::member_from_hwnd::<Window>(self.hwnd) {
            if let Some(old) = old.as_mut() {
                let outer_self = outer_self.as_single_container_mut().as_container_mut();
                old.on_removed_from_container(outer_self);
            }
            if let Some(new) = child.as_mut() {
                let outer_self = outer_self.as_single_container_mut().as_container_mut();
                let size = self.size_inner();
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
        self.skip_callbacks = skip_callbacks;
        unsafe {
            winuser::SendMessageW(self.hwnd, winuser::WM_SYSCOMMAND, winuser::SC_CLOSE, 0);
        }
        self.hwnd.is_null()
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
}
impl HasNativeIdInner for TestableWindow {
    type Id = common::Hwnd;

    unsafe fn native_id(&self) -> Self::Id {
        self.hwnd.into()
    }
}
impl MemberInner for TestableWindow {}

impl Drop for TestableWindow {
    fn drop(&mut self) {
        if let Some(self2) = common::member_from_hwnd::<Window>(self.hwnd) {
            self.set_child(self2.base_mut(), None);
        }
        destroy_hwnd(self.hwnd, 0, None);
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

unsafe extern "system" fn handler(hwnd: windef::HWND, msg: minwindef::UINT, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
    let ww = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA);
    if ww == 0 {
        if winuser::WM_CREATE == msg {
            let cs: &mut winuser::CREATESTRUCTW = mem::transmute(lparam);
            winuser::SetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA, cs.lpCreateParams as isize);
        }
        return winuser::DefWindowProcW(hwnd, msg, wparam, lparam);
    }

    match msg {
        winuser::WM_SIZE => {
            let width = minwindef::LOWORD(lparam as u32);
            let height = minwindef::HIWORD(lparam as u32);
            let w: &mut Window = mem::transmute(ww);

            w.as_inner_mut().as_inner_mut().as_inner_mut().redraw();

            winuser::InvalidateRect(w.as_inner().as_inner().as_inner().hwnd, ptr::null_mut(), minwindef::TRUE);

            w.call_on_size(width, height);
            return 0;
        }
        winuser::WM_DESTROY => {
            let w: &mut Window = mem::transmute(ww);
            w.as_inner_mut().as_inner_mut().as_inner_mut().hwnd = ptr::null_mut();
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
    winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
}

default_impls_as!(Window);
