use crate::application::Application;
use crate::common::{self, *};

pub const SIZE: u32 = 64;

#[repr(C)]
pub struct TestableTray {
	id: InnerId,
    label: String,
//    cfg: shellapi::NOTIFYICONDATAW,
	menu: types::Menu,
	image: image::DynamicImage,
    on_close: Option<callbacks::OnClose>,
}

pub type Tray = AMember<ATray<TestableTray>>;

/*impl TestableTray {
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
}*/

impl HasLabelInner for TestableTray {
    fn label(&self, _base: &MemberBase) -> Cow<str> {
        Cow::Borrowed(self.label.as_ref())
    }
    fn set_label(&mut self, _base: &mut MemberBase, label: Cow<str>) {
        self.label = label.into();
    }
}

impl CloseableInner for TestableTray {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        if !skip_callbacks {
            if let Some(ref mut on_close) = self.on_close {
                if !(on_close.as_mut())(unsafe { &mut *(self.id as *mut Tray) }) {
                    return false;
                }
            }
        }
        let mut app = Application::get().unwrap();
        let app = app.as_any_mut().downcast_mut::<Application>().unwrap();
        app.inner_mut().remove_tray(self.id.into());

        println!("Tray '{}' closed ({:?})", self.label, self.id);
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
    	//use plygui_api::external::image::GenericImageView;
    	let i = i.resize(SIZE, SIZE, image::imageops::FilterType::Lanczos3);
    	self.image = i.into();
    }
}

impl TrayInner for TestableTray {
    fn with_params<S: AsRef<str>>(title: S, menu: types::Menu) -> Box<dyn controls::Tray> {
        let mut t = Box::new(AMember::with_inner(
            ATray::with_inner(
                TestableTray {
                	id: 0 as InnerId,
                    label: title.as_ref().to_owned(),
                    menu: menu,
                    image: image::DynamicImage::ImageRgba8(image::ImageBuffer::new(1,1)),
                    on_close: None,
                }
            )
        ));
        let this = t.as_mut();
        t.inner_mut().inner_mut().id = this as *mut _ as *mut MemberBase;

        /*let app = super::application::Application::get();
		if let Some(items) = menu {
            unsafe {
                let menu = winuser::CreatePopupMenu();
                common::make_menu(menu, items, &mut t.inner_mut().menu.1);
                t.inner_mut().menu.0 = menu;
            }
        }*/
        t
    }
}

impl HasNativeIdInner for TestableTray {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.id.into()
    }
}

impl MemberInner for TestableTray {}
