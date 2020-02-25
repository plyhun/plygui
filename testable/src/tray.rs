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
        app.inner_mut().unregister_tray(common::member_from_id::<Tray>(self.id).unwrap());

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
impl<O: controls::Tray> NewTrayInner<O> for TestableTray {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, title: &str, icon: image::DynamicImage, menu: types::Menu) -> Self {
        TestableTray {
        	id: unsafe { mem::transmute(u) },
            label: title.to_owned(),
            menu: menu,
            image: icon,
            on_close: None,
        }
    }
}
impl TrayInner for TestableTray {
    fn with_params<S: AsRef<str>>(title: S, icon: image::DynamicImage, menu: types::Menu) -> Box<dyn controls::Tray> {
        let mut b: Box<mem::MaybeUninit<Tray>> = Box::new_uninit();
        let ab = AMember::with_inner(
            ATray::with_inner(
                <Self as NewTrayInner<Tray>>::with_uninit_params(b.as_mut(), title.as_ref(), icon, menu),
	            crate::application::Application::get().unwrap(),
            )
        );
        /*if let Some(items) = menu {
            let menu = winuser::CreateMenu();
            common::make_menu(menu, items, &mut w.inner_mut().inner_mut().inner_mut().menu);
            winuser::SetMenu(id, menu);
        }*/
        let mut t: Box<dyn controls::Tray> = unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        };
        let app = crate::application::Application::get().unwrap();
        let mut app = app.into_any().downcast::<crate::application::Application>().unwrap();
        app.inner_mut().register_tray(&mut t);
        t
    }
}

impl HasNativeIdInner for TestableTray {
    type Id = common::TestableId;

    fn native_id(&self) -> Self::Id {
        self.id.into()
    }
}

impl MemberInner for TestableTray {}
