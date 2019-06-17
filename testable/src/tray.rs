use crate::application::Application;
use crate::common::{self, *};

use winapi::um::shellapi;

pub const MESSAGE: u32 = 0xbaba;

#[repr(C)]
pub struct TestableTray {
    label: String,
    cfg: shellapi::NOTIFYICONDATAW,
    menu: (windef::HMENU, Vec<callbacks::Action>, isize),
    on_close: Option<callbacks::OnClose>,
    this: *mut Tray,
}

pub type Tray = Member<TestableTray>;

impl TestableTray {
    pub(crate) fn toggle_menu(&mut self) {
        if !self.menu.0.is_null() {
            unsafe {
                let hwnd = Application::get().unwrap().native_id() as windef::HWND;
                if self.menu.2 > -2 {
                    self.menu.2 = -2;
                    winuser::SendMessageW(hwnd, winuser::WM_CANCELMODE, 0, 0);
                } else {
                    self.menu.2 = -1;
                    let mut click_point = mem::zeroed();
                    winuser::GetCursorPos(&mut click_point);
                    winuser::TrackPopupMenu(self.menu.0, winuser::TPM_LEFTALIGN | winuser::TPM_LEFTBUTTON | winuser::TPM_BOTTOMALIGN, click_point.x, click_point.y, 0, hwnd, ptr::null_mut());
                }
            }
        }
    }
    pub(crate) fn is_menu_shown(&self) -> bool {
        self.menu.2 > -2
    }
    pub(crate) fn select_menu(&mut self, id: usize) {
        self.menu.2 = id as isize;
    }
    pub(crate) fn run_menu(&mut self, this: &mut Tray) {
        if self.menu.2 > -1 {
            if let Some(a) = self.menu.1.get_mut(self.menu.2 as usize) {
                (a.as_mut())(this);
            }
        }
        self.menu.2 = -2;
    }
}

impl HasLabelInner for TestableTray {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        Cow::Borrowed(self.label.as_ref())
    }
    fn set_label(&mut self, _base: &mut MemberBase, label: Cow<str>) {
        self.label = label.into();
        if !self.cfg.hWnd.is_null() {
            let control_name = common::str_to_wchar(&self.label);
            unsafe {
                winuser::SetWindowTextW(self.cfg.hWnd, control_name.as_ptr());
            }
        }
    }
}

impl CloseableInner for TestableTray {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        if !skip_callbacks {
            if let Some(ref mut on_close) = self.on_close {
                if !(on_close.as_mut())(unsafe { &mut *self.this }) {
                    return false;
                }
            }
        }

        let mut app = Application::get().unwrap();
        let app = app.as_any_mut().downcast_mut::<Application>().unwrap();

        unsafe {
            if shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, &mut self.cfg) == minwindef::FALSE {
                common::log_error();
            }
        }
        app.as_inner_mut().remove_tray((self.this as windef::HWND).into());

        true
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.on_close = callback;
    }
}

impl HasImageInner for TestableTray {
	fn image(&self, _base: &MemberBase) -> Cow<image::DynamicImage> {
        unimplemented!()
    }
    #[inline]
    fn set_image(&mut self, _base: &mut MemberBase, i: Cow<image::DynamicImage>) {
    	use plygui_api::external::image::GenericImageView;
    	
    	let i = unsafe {
    		let status_size = winuser::GetSystemMetrics(winuser::SM_CXSMICON) as u32;
    		i.resize(status_size, status_size, image::FilterType::Lanczos3)
    	};
    	
    	let (w,h) = i.dimensions();
    	let mut mask = image::ImageBuffer::new(w, h);
	    for x in 0..w {
	        for y in 0..h {
	            let bright = std::u8::MAX;
	            mask.put_pixel(x, y, image::Rgba([bright, bright, bright, 0xff]));
	        }
	    }
    	unsafe {
    		if !self.cfg.hIcon.is_null() {
    			winuser::DestroyIcon(self.cfg.hIcon);
    		}
	        let mut ii: winuser::ICONINFO = mem::zeroed();
	        ii.fIcon = minwindef::TRUE;
	        common::image_to_native(&image::DynamicImage::ImageRgba8(mask), &mut ii.hbmMask);
	        common::image_to_native(&i, &mut ii.hbmColor);
	        self.cfg.hIcon = winuser::CreateIconIndirect(&mut ii);
	        if shellapi::Shell_NotifyIconW(shellapi::NIM_MODIFY, &mut self.cfg) == minwindef::FALSE {
                common::log_error();
            }
    	}
    }
}

impl TrayInner for TestableTray {
    fn with_params(title: &str, menu: types::Menu) -> Box<Member<Self>> {
        use plygui_api::controls::Member as OuterMember;

        let mut t = Box::new(Member::with_inner(
            TestableTray {
                label: title.into(),
                cfg: unsafe { mem::zeroed() },
                menu: (ptr::null_mut(), if menu.is_some() { Vec::new() } else { vec![] }, -2),
                on_close: None,
                this: ptr::null_mut(),
            },
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));
        let this = t.as_mut() as *mut Tray;
        t.as_inner_mut().this = this;

        let app = super::application::Application::get();
        let tip_size = t.as_inner_mut().cfg.szTip.len();
        let title = OsStr::new(t.as_inner().label.as_str()).encode_wide().take(tip_size - 1).chain(Some(0).into_iter()).collect::<Vec<_>>();

        t.as_inner_mut().cfg.hWnd = unsafe { app.unwrap().native_id() as windef::HWND };
        t.as_inner_mut().cfg.cbSize = mem::size_of::<shellapi::NOTIFYICONDATAW>() as u32;
        t.as_inner_mut().cfg.uID = unsafe { t.id().into_raw() as u32 };
        //t.as_inner_mut().cfg.hIcon = unsafe { winuser::GetClassLongW(app.as_inner().root.into(), winuser::GCL_HICON) as windef::HICON };

        unsafe {
            commctrl::LoadIconMetric(ptr::null_mut(), winuser::MAKEINTRESOURCEW(32512), commctrl::LIM_SMALL as i32, &mut t.as_inner_mut().cfg.hIcon);
        }

        t.as_inner_mut().cfg.uFlags = shellapi::NIF_ICON | shellapi::NIF_TIP | shellapi::NIF_MESSAGE | shellapi::NIF_SHOWTIP;
        t.as_inner_mut().cfg.uCallbackMessage = MESSAGE;
        t.as_inner_mut().cfg.szTip[..title.len()].clone_from_slice(title.as_slice());
        unsafe {
            if shellapi::Shell_NotifyIconW(shellapi::NIM_ADD, &mut t.as_inner_mut().cfg) == minwindef::FALSE {
                common::log_error();
            }
            *t.as_inner_mut().cfg.u.uVersion_mut() = shellapi::NOTIFYICON_VERSION_4;
            if shellapi::Shell_NotifyIconW(shellapi::NIM_SETVERSION, &mut t.as_inner_mut().cfg) == minwindef::FALSE {
                common::log_error();
            }
        }
        if let Some(items) = menu {
            unsafe {
                let menu = winuser::CreatePopupMenu();
                common::make_menu(menu, items, &mut t.as_inner_mut().menu.1);
                t.as_inner_mut().menu.0 = menu;
            }
        }

        t
    }
}

impl HasNativeIdInner for TestableTray {
    type Id = common::Hwnd;

    unsafe fn native_id(&self) -> Self::Id {
        self.cfg.hWnd.into()
    }
}

impl MemberInner for TestableTray {}

impl Drop for TestableTray {
    fn drop(&mut self) {
        unsafe {
            if !self.menu.0.is_null() {
                winuser::DeleteMenu(self.menu.0, 0, 0);
                if shellapi::Shell_NotifyIconW(shellapi::NIM_DELETE, &mut self.cfg) == minwindef::FALSE {
                    common::log_error();
                }
            }
        }
    }
}

default_impls_as!(Tray);
