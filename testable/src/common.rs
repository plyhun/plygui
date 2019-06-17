pub use plygui_api::development::*;
pub use plygui_api::{callbacks, controls, defaults, ids, layout, types, utils};
pub use plygui_api::external::image;

pub use std::borrow::Cow;
pub use std::ffi::{CString, IntoStringError, OsStr};
pub use std::marker::PhantomData;
pub use std::os::windows::ffi::OsStrExt;
pub use std::{cmp, mem, ops, ptr, str, sync::mpsc};

pub const DEFAULT_PADDING: i32 = 2;

pub type InnerId = ids::Id;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TestableId(InnerId);

impl From<InnerId> for TestableId {
    #[inline]
    fn from(a: InnerId) -> TestableId {
        TestableId(a)
    }
}
impl From<TestableId> for InnerId {
    #[inline]
    fn from(a: TestableId) -> InnerId {
        a.0
    }
}
impl From<TestableId> for usize {
    #[inline]
    fn from(a: TestableId) -> usize {
        unsafe { a.0.into_raw() }
    }
}
impl NativeId for TestableId {}

#[repr(C)]
pub struct TestableControlBase<T: controls::Control + Sized> {
    pub id: InnerId,
    _marker: PhantomData<T>,
}

impl<T: controls::Control + Sized> TestableControlBase<T> {
    pub fn new() -> TestableControlBase<T> {
        TestableControlBase {
            id: unsafe { InnerId::from_raw(0) },
            _marker: PhantomData,
        }
    }

    pub fn parent_id(&self) -> Option<InnerId> {
        unsafe {
            let parent_id = winuser::GetParent(self.id);
            if parent_id == self.id {
                None
            } else {
                Some(parent_id)
            }
        }
    }
    pub fn parent(&self) -> Option<&MemberBase> {
        unsafe {
            let parent_id = winuser::GetParent(self.id);
            if parent_id == self.id {
                return None;
            }

            let parent_ptr = winuser::GetWindowLongPtrW(parent_id, winuser::GWLP_USERDATA);
            mem::transmute(parent_ptr as *mut c_void)
        }
    }
    pub fn parent_mut(&mut self) -> Option<&mut MemberBase> {
        unsafe {
            let parent_id = winuser::GetParent(self.id);
            if parent_id == self.id {
                return None;
            }

            let parent_ptr = winuser::GetWindowLongPtrW(parent_id, winuser::GWLP_USERDATA);
            mem::transmute(parent_ptr as *mut c_void)
        }
    }
    pub fn root(&self) -> Option<&MemberBase> {
        unsafe {
            let parent_id = winuser::GetAncestor(self.id, 2); //GA_ROOT
            if parent_id == self.id {
                return None;
            }

            let parent_ptr = winuser::GetWindowLongPtrW(parent_id, winuser::GWLP_USERDATA);
            mem::transmute(parent_ptr as *mut c_void)
        }
    }
    pub fn root_mut(&mut self) -> Option<&mut MemberBase> {
        unsafe {
            let parent_id = winuser::GetAncestor(self.id, 2); //GA_ROOT
            if parent_id == self.id {
                return None;
            }

            let parent_ptr = winuser::GetWindowLongPtrW(parent_id, winuser::GWLP_USERDATA);
            mem::transmute(parent_ptr as *mut c_void)
        }
    }
    pub fn as_outer(&self) -> &T {
        member_from_id::<T>(self.id).unwrap()
    }
    pub fn as_outer_mut(&self) -> &mut T {
        member_from_id::<T>(self.id).unwrap()
    }
    pub fn invalidate(&mut self) {
        if self.id.is_null() {
            return;
        }
        let parent_id = self.parent_id();
        let this = self.as_outer_mut();
        if this.is_skip_draw() {
            return;
        }
        if let Some(parent_id) = parent_id {
            if let Some(mparent) = member_base_from_id(parent_id) {
                let (pw, ph) = mparent.as_member().is_has_size().unwrap().size();
                let (_, _, changed) = this.measure(pw, ph);

                if let Some(cparent) = mparent.as_member_mut().is_control_mut() {
                    if changed && !cparent.is_skip_draw() {
                        cparent.invalidate();
                    }
                } else {
                    this.draw(None);
                }
            }
        }
    }
    pub fn draw(&mut self, coords: Option<(i32, i32)>, (width, height): (u16, u16)) -> bool {
        draw(self.id, coords, (width, height))
    }
    pub fn on_set_visibility(&mut self, visibility: types::Visibility) -> bool {
        if !self.id.is_null() {
            unsafe {
                winuser::ShowWindow(self.id, if visibility == types::Visibility::Visible { winuser::SW_SHOW } else { winuser::SW_HIDE });
            }
            self.invalidate();
            true
        } else {
            false
        }
    }
}

pub fn size_id(id: InnerId) -> (u16, u16) {
    let rect = unsafe { window_rect(id) };
    ((rect.right - rect.left) as u16, (rect.bottom - rect.top) as u16)
}
pub fn pos_id(id: InnerId) -> (i32, i32) {
    let rect = unsafe { window_rect(id) };
    (rect.left as i32, rect.top as i32)
}

pub unsafe fn get_class_name_by_id(id: InnerId) -> Vec<u16> {
    let mut max_id = 256;
    let mut name = vec![0u16; max_id];
    max_id = winuser::GetClassNameW(id, name.as_mut_slice().as_ptr(), max_id as i32) as usize;
    name.truncate(max_id);
    name
}

pub unsafe fn create_control_id(
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    parent: InnerId,
    ex_style: minwindef::DWORD,
    class_name: ntdef::LPCWSTR,
    control_name: &str,
    style: minwindef::DWORD,
    param: minwindef::LPVOID,
    handler: Option<WndHandler>,
) -> (InnerId, usize) {
    let mut style = style;
    if (style & winuser::WS_TABSTOP) != 0 {
        style |= winuser::WS_GROUP;
    }
    let subclass_id = {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::Hasher;

        let mut hasher = DefaultHasher::new();
        hasher.write_usize(class_name as usize);
        hasher.finish()
    };
    let os_control_name = OsStr::new(control_name).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
    let id = winuser::CreateWindowExW(
        ex_style,
        class_name,
        os_control_name.as_ptr(),
        style | winuser::WS_CHILD | winuser::WS_VISIBLE,
        x,
        y,
        w,
        h,
        parent,
        ptr::null_mut(),
        hinstance(),
        param,
    );
    if id.is_null() {
        log_error();
        panic!("Cannot create window {}", control_name);
    }
    commctrl::SetWindowSubclass(id, handler, subclass_id as usize, param as usize);
    set_default_font(id);
    (id, subclass_id as usize)
}

pub fn str_to_wchar<S: AsRef<str>>(a: S) -> Vec<u16> {
    OsStr::new(a.as_ref()).encode_wide().chain(Some(0).into_iter()).collect()
}
pub unsafe fn wchar_to_str(p: *const u16) -> String {
    let len = (0..).take_while(|&i| *p.offset(i) != 0).count();
    let slice = std::slice::from_raw_parts(p, len);
    String::from_utf16_lossy(slice)
}

#[inline]
pub unsafe fn set_default_font(id: InnerId) {
    winuser::SendMessageW(id, winuser::WM_SETFONT, hfont() as usize, minwindef::TRUE as isize);
}

pub fn destroy_id(id: InnerId, subclass_id: usize, handler: Option<unsafe extern "system" fn(InnerId, msg: minwindef::UINT, minwindef::WPARAM, minwindef::LPARAM, usize, usize) -> isize>) {
    unsafe {
        if subclass_id != 0 {
            commctrl::RemoveWindowSubclass(id, handler, subclass_id);
        }
        if winuser::DestroyWindow(id) == 0 && winuser::IsWindow(id) > 0 {
            log_error();
        }
    }
}

#[inline]
pub fn draw(id: InnerId, coords: Option<(i32, i32)>, (width, height): (u16, u16)) -> bool {
    if let Some((x, y)) = coords {
        unsafe {
            winuser::SetWindowPos(id, ptr::null_mut(), x, y, width as i32, height as i32, 0);
        }
        true
    } else {
        false
    }
}

#[inline]
pub unsafe fn window_rect(id: InnerId) -> windef::RECT {
    let mut rect: windef::RECT = mem::zeroed();
    winuser::GetClientRect(id, &mut rect);
    rect
}

#[inline]
pub(crate) unsafe fn cast_id<'a, T>(id: InnerId) -> Option<&'a mut T>
where
    T: Sized,
{
    let id_ptr = winuser::GetWindowLongPtrW(id, winuser::GWLP_USERDATA);
    if id_ptr == 0 {
        None
    } else {
        Some(mem::transmute(id_ptr as *mut c_void))
    }
}
#[inline]
pub fn member_from_id<'a, T>(id: InnerId) -> Option<&'a mut T>
where
    T: Sized + controls::Member,
{
    unsafe { cast_id(id) }
}
#[inline]
pub fn member_base_from_id<'a>(id: InnerId) -> Option<&'a mut MemberBase> {
    unsafe { cast_id(id) }
}

pub unsafe fn make_menu(menu: windef::HMENU, mut items: Vec<types::MenuItem>, storage: &mut Vec<callbacks::Action>) {
    let mut options = Vec::new();
    let mut help = Vec::new();

    let append_item = |menu, label, action, storage: &mut Vec<callbacks::Action>| {
        let wlabel = str_to_wchar(label);
        let id = storage.len();
        storage.push(action);
        winuser::AppendMenuW(menu, winuser::MF_STRING, id, wlabel.as_ptr());
    };
    let append_level = |menu, label, items, storage: &mut Vec<callbacks::Action>| {
        let wlabel = str_to_wchar(label);
        let submenu = winuser::CreateMenu();
        make_menu(submenu, items, storage);
        winuser::AppendMenuW(menu, winuser::MF_POPUP, submenu as usize, wlabel.as_ptr());
    };
    let make_special = |menu, mut special: Vec<types::MenuItem>, storage: &mut Vec<callbacks::Action>| {
        for item in special.drain(..) {
            match item {
                types::MenuItem::Action(label, action, _) => {
                    append_item(menu, label, action, storage);
                }
                types::MenuItem::Sub(label, items, _) => {
                    append_level(menu, label, items, storage);
                }
                types::MenuItem::Delimiter => {
                    winuser::AppendMenuW(menu, winuser::MF_SEPARATOR, 0, ptr::null_mut());
                }
            }
        }
    };

    for item in items.drain(..) {
        match item {
            types::MenuItem::Action(label, action, role) => match role {
                types::MenuItemRole::None => {
                    append_item(menu, label, action, storage);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Action(label, action, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Action(label, action, role));
                }
            },
            types::MenuItem::Sub(label, items, role) => match role {
                types::MenuItemRole::None => {
                    append_level(menu, label, items, storage);
                }
                types::MenuItemRole::Options => {
                    options.push(types::MenuItem::Sub(label, items, role));
                }
                types::MenuItemRole::Help => {
                    help.push(types::MenuItem::Sub(label, items, role));
                }
            },
            types::MenuItem::Delimiter => {
                winuser::AppendMenuW(menu, winuser::MF_SEPARATOR, 0, ptr::null_mut());
            }
        }
    }

    make_special(menu, options, storage);
    make_special(menu, help, storage);
}

pub unsafe fn image_to_native(src: &image::DynamicImage, dst: *mut windef::HBITMAP) {
    use image::GenericImageView;

    let (w, h) = src.dimensions();

    let bminfo = wingdi::BITMAPINFO {
        bmiHeader: wingdi::BITMAPINFOHEADER {
            biSize: mem::size_of::<wingdi::BITMAPINFOHEADER>() as u32,
            biWidth: w as i32,
            biHeight: h as i32,
            biPlanes: 1,
            biBitCount: 32,
            biCompression: wingdi::BI_RGB,
            biSizeImage: 0,
            biXPelsPerMeter: 0,
            biYPelsPerMeter: 0,
            biClrUsed: 0,
            biClrImportant: 0,
        },
        bmiColors: mem::zeroed(),
    };

    let mut pv_image_bits = ptr::null_mut();
    let hdc_screen = winuser::GetDC(ptr::null_mut());
    *dst = wingdi::CreateDIBSection(hdc_screen, &bminfo, wingdi::DIB_RGB_COLORS, &mut pv_image_bits, ptr::null_mut(), 0);
    winuser::ReleaseDC(ptr::null_mut(), hdc_screen);
    if (*dst).is_null() {
        panic!("Could not load image.")
    }

    ptr::copy(src.flipv().to_rgba().into_raw().as_ptr(), pv_image_bits as *mut u8, (w * h * 4) as usize);
}

#[cfg(not(debug_assertions))]
pub unsafe fn log_error() {}

#[cfg(debug_assertions)]
pub unsafe fn log_error() {
    let error = errhandlingapi::GetLastError();
    if error == 0 {
        return;
    }

    let mut string = vec![0u16; 127];
    winbase::FormatMessageW(
        winbase::FORMAT_MESSAGE_FROM_SYSTEM | winbase::FORMAT_MESSAGE_IGNORE_INSERTS,
        ptr::null_mut(),
        error,
        ntdef::LANG_SYSTEM_DEFAULT as u32,
        string.as_mut_ptr(),
        string.len() as u32,
        ptr::null_mut(),
    );

    println!("Last error #{}: {}", error, String::from_utf16_lossy(&string));
}
