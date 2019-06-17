use crate::common::{self, *};

lazy_static! {
    pub static ref WINDOW_CLASS_GBOX: Vec<u16> = OsStr::new("Button").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
    pub static ref WINDOW_CLASS: Vec<u16> = unsafe { register_window_class() };
}

pub type Frame = Member<Control<SingleContainer<TestableFrame>>>;

#[repr(C)]
pub struct TestableFrame {
    base: common::TestableControlBase<Frame>,
    hwnd_gbox: windef::HWND,
    label: String,
    label_padding: i32,
    child: Option<Box<dyn controls::Control>>,
}

impl FrameInner for TestableFrame {
    fn with_label(label: &str) -> Box<Frame> {
        let b = Box::new(Member::with_inner(
            Control::with_inner(
                SingleContainer::with_inner(
                    TestableFrame {
                        base: common::TestableControlBase::new(),
                        child: None,
                        hwnd_gbox: 0 as windef::HWND,
                        label: label.to_owned(),
                        label_padding: 0,
                    },
                    (),
                ),
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        b
    }
}

impl HasLayoutInner for TestableFrame {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        let hwnd = self.base.hwnd;
        if !hwnd.is_null() {
            unsafe { winuser::RedrawWindow(self.hwnd_gbox, ptr::null_mut(), ptr::null_mut(), winuser::RDW_INVALIDATE | winuser::RDW_UPDATENOW) };
            self.base.invalidate();
        }
    }
    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        layout::BoundarySize::Distinct(DEFAULT_PADDING, DEFAULT_PADDING + self.label_padding, DEFAULT_PADDING, DEFAULT_PADDING)
    }
}

impl HasLabelInner for TestableFrame {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        Cow::Borrowed(self.label.as_ref())
    }
    fn set_label(&mut self, base: &mut MemberBase, label: Cow<str>) {
        self.label = label.into();
        let hwnd = self.base.hwnd;
        if !hwnd.is_null() {
            let control_name = OsStr::new(&self.label).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
            unsafe {
                winuser::SetWindowTextW(self.base.hwnd, control_name.as_ptr());
            }
        }
        self.on_layout_changed(base);
    }
}

impl SingleContainerInner for TestableFrame {
    fn set_child(&mut self, base: &mut MemberBase, child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        let mut old = self.child.take();
        if let Some(old) = old.as_mut() {
            if !self.base.hwnd.is_null() {
                old.on_removed_from_container(self.base.as_outer_mut());
            }
        }
        self.child = child;

        if self.child.is_some() {
            if !self.base.hwnd.is_null() {
                let (w, h) = base.as_any().downcast_ref::<Frame>().unwrap().as_inner().base().measured;
                if let Some(new) = self.child.as_mut() {
                    new.as_mut().on_added_to_container(
                        self.base.as_outer_mut(),
                        DEFAULT_PADDING,
                        DEFAULT_PADDING,
                        cmp::max(0, w as i32 - DEFAULT_PADDING - DEFAULT_PADDING) as u16,
                        cmp::max(0, h as i32 - DEFAULT_PADDING - DEFAULT_PADDING) as u16,
                    );
                }
            }
        }
        self.on_layout_changed(base);

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

impl ContainerInner for TestableFrame {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        if let Some(child) = self.child.as_mut() {
            match arg {
                types::FindBy::Id(id) => {
                    if child.as_member_mut().id() == id {
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
                c.find_control_mut(arg)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        if let Some(child) = self.child.as_ref() {
            match arg {
                types::FindBy::Id(id) => {
                    if child.as_member().id() == id {
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
                c.find_control(arg)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl ControlInner for TestableFrame {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, px: i32, py: i32, pw: u16, ph: u16) {
        let selfptr = member as *mut _ as *mut c_void;
        let (hwnd, hwnd_gbox, id) = unsafe {
            self.base.hwnd = parent.native_id() as windef::HWND; // required for measure, as we don't have own hwnd yet
            let (width, height, _) = self.measure(member, control, pw, ph);
            let (hwnd, id) = common::create_control_hwnd(
                px,
                py + self.label_padding,
                width as i32,
                height as i32 - self.label_padding,
                self.base.hwnd,
                winuser::WS_EX_CONTROLPARENT,
                WINDOW_CLASS.as_ptr(),
                "",
                0,
                selfptr,
                None,
            );
            let hwnd_gbox = winuser::CreateWindowExW(
                0,
                WINDOW_CLASS_GBOX.as_ptr(),
                OsStr::new(self.label.as_str()).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>().as_ptr(),
                winuser::BS_GROUPBOX | winuser::WS_CHILD | winuser::WS_VISIBLE,
                px,
                py,
                width as i32,
                height as i32,
                self.base.hwnd,
                ptr::null_mut(),
                common::hinstance(),
                ptr::null_mut(),
            );
            common::set_default_font(hwnd_gbox);
            (hwnd, hwnd_gbox, id)
        };
        self.base.hwnd = hwnd;
        self.hwnd_gbox = hwnd_gbox;
        self.base.subclass_id = id;
        control.coords = Some((px, py));
        if let Some(ref mut child) = self.child {
            let self2: &mut Frame = unsafe { utils::base_to_impl_mut(member) };
            child.on_added_to_container(
                self2,
                DEFAULT_PADDING,
                DEFAULT_PADDING,
                cmp::max(0, control.measured.0 as i32 - DEFAULT_PADDING - DEFAULT_PADDING) as u16,
                cmp::max(0, control.measured.1 as i32 - DEFAULT_PADDING - DEFAULT_PADDING - self.label_padding) as u16,
            );
        }
    }
    fn on_removed_from_container(&mut self, member: &mut MemberBase, _control: &mut ControlBase, _: &dyn controls::Container) {
        if let Some(ref mut child) = self.child {
            let self2: &mut Frame = unsafe { utils::base_to_impl_mut(member) };
            child.on_removed_from_container(self2);
        }
        common::destroy_hwnd(self.hwnd_gbox, 0, None);
        common::destroy_hwnd(self.base.hwnd, self.base.subclass_id, None);
        self.base.hwnd = 0 as windef::HWND;
        self.hwnd_gbox = 0 as windef::HWND;
        self.base.subclass_id = 0;
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
        use plygui_api::markup::MEMBER_TYPE_FRAME;

        fill_from_markup_base!(self, member, markup, registry, Frame, [MEMBER_TYPE_FRAME]);
        fill_from_markup_label!(self, member, markup);
        fill_from_markup_child!(self, member, markup, registry);
    }
}

impl HasNativeIdInner for TestableFrame {
    type Id = common::Hwnd;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.hwnd.into()
    }
}

impl HasSizeInner for TestableFrame {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<Frame>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        true
    }
}
impl HasVisibilityInner for TestableFrame {
    fn on_visibility_set(&mut self, base: &mut MemberBase, visibility: types::Visibility) -> bool {
        let hwnd = self.base.hwnd;
        if !hwnd.is_null() {
            unsafe {
                winuser::ShowWindow(self.hwnd_gbox, if visibility == types::Visibility::Visible { winuser::SW_SHOW } else { winuser::SW_HIDE });
                winuser::ShowWindow(self.base.hwnd, if visibility == types::Visibility::Visible { winuser::SW_SHOW } else { winuser::SW_HIDE });
            }
            self.on_layout_changed(base);
            true
        } else {
            false
        }
    }
}

impl MemberInner for TestableFrame {}

impl Drawable for TestableFrame {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        if let Some((x, y)) = control.coords {
            unsafe {
                winuser::SetWindowPos(self.base.hwnd, ptr::null_mut(), x, y + self.label_padding, control.measured.0 as i32, control.measured.1 as i32 - self.label_padding, 0);
                winuser::SetWindowPos(self.hwnd_gbox, ptr::null_mut(), x, y, control.measured.0 as i32, control.measured.1 as i32, 0);
            }
            /*if let Some(ref mut child) = self.child {
                child.draw(Some((DEFAULT_PADDING, DEFAULT_PADDING)));
            }*/
        }
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        use std::cmp::max;

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
                        if let Some(ref mut child) = self.child {
                            self.label_padding = update_label_size(self.label.as_str(), self.base.hwnd);
                            let (cw, _, _) = child.measure(max(0, parent_width as i32 - hp) as u16, max(0, parent_height as i32 - vp - self.label_padding) as u16);
                            w += cw as i32;
                            measured = true;
                        }
                        max(0, w as i32 + hp) as u16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::Exact(h) => h,
                    layout::Size::MatchParent => parent_height,
                    layout::Size::WrapContent => {
                        let mut h = 0;
                        if let Some(ref mut child) = self.child {
                            let ch = if measured {
                                child.size().1
                            } else {
                                self.label_padding = update_label_size(self.label.as_str(), self.base.hwnd);
                                let (_, ch, _) = child.measure(max(0, parent_width as i32 - hp) as u16, max(0, parent_height as i32 - vp - self.label_padding) as u16);
                                ch
                            };
                            h += ch as i32;
                            h += self.label_padding;
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
        unsafe { winuser::RedrawWindow(self.hwnd_gbox, ptr::null_mut(), ptr::null_mut(), winuser::RDW_INVALIDATE | winuser::RDW_UPDATENOW) };
        self.base.invalidate();
    }
}

#[allow(dead_code)]
pub(crate) fn spawn() -> Box<dyn controls::Control> {
    Frame::with_label("").into_control()
}

fn update_label_size(label: &str, hwnd: windef::HWND) -> i32 {
    let mut label_size: windef::SIZE = unsafe { mem::zeroed() };
    let label = OsStr::new(label).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
    unsafe {
        wingdi::GetTextExtentPointW(winuser::GetDC(hwnd), label.as_ptr(), label.len() as i32, &mut label_size);
    }
    label_size.cy as i32 / 2
}

unsafe fn register_window_class() -> Vec<u16> {
    let class_name = OsStr::new("PlyguiWin32Frame").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
    let class = winuser::WNDCLASSEXW {
        cbSize: mem::size_of::<winuser::WNDCLASSEXW>() as minwindef::UINT,
        style: winuser::CS_DBLCLKS,
        lpfnWndProc: Some(whandler),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: libloaderapi::GetModuleHandleW(ptr::null()),
        hIcon: winuser::LoadIconW(ptr::null_mut(), winuser::IDI_APPLICATION),
        hCursor: winuser::LoadCursorW(ptr::null_mut(), winuser::IDC_ARROW),
        hbrBackground: ptr::null_mut(),
        lpszMenuName: ptr::null(),
        lpszClassName: class_name.as_ptr(),
        hIconSm: ptr::null_mut(),
    };
    winuser::RegisterClassExW(&class);
    class_name
}

unsafe extern "system" fn whandler(hwnd: windef::HWND, msg: minwindef::UINT, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {
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
            let width = lparam as u16;
            let height = (lparam >> 16) as u16;
            let frame: &mut Frame = mem::transmute(ww);
            let label_padding = frame.as_inner().as_inner().as_inner().label_padding;
            let hp = DEFAULT_PADDING + DEFAULT_PADDING;
            let vp = DEFAULT_PADDING + DEFAULT_PADDING + label_padding;

            if let Some(ref mut child) = frame.as_inner_mut().as_inner_mut().as_inner_mut().child {
                child.measure(cmp::max(0, width as i32 - hp) as u16, cmp::max(0, height as i32 - vp) as u16);
                child.draw(Some((DEFAULT_PADDING, DEFAULT_PADDING)));
            }
            frame.call_on_size(width, height);
            return 0;
        }
        _ => {}
    }

    winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
}

default_impls_as!(Frame);
