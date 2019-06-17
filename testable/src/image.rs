use crate::common::{self, *};

lazy_static! {
    pub static ref WINDOW_CLASS: Vec<u16> = OsStr::new("STATIC").encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>();
}

pub type Image = Member<Control<TestableImage>>;

#[repr(C)]
pub struct TestableImage {
    base: TestableControlBase<Image>,

    bmp: windef::HBITMAP,
    scale: types::ImageScalePolicy,
}

impl TestableImage {
    fn install_image(&mut self, content: image::DynamicImage) {
		unsafe { common::image_to_native(&content, &mut self.bmp); }
    }
    fn remove_image(&mut self) {
        unsafe {
            wingdi::DeleteObject(self.bmp as *mut c_void);
        }
        self.bmp = ptr::null_mut();
    }
}

impl Drop for TestableImage {
    fn drop(&mut self) {
        self.remove_image();
    }
}

impl ImageInner for TestableImage {
    fn with_content(content: image::DynamicImage) -> Box<controls::Image> {
        let mut i = Box::new(Member::with_inner(
            Control::with_inner(
                TestableImage {
                    base: TestableControlBase::new(),

                    bmp: ptr::null_mut(),
                    scale: types::ImageScalePolicy::FitCenter,
                },
                (),
            ),
            MemberFunctions::new(_as_any, _as_any_mut, _as_member, _as_member_mut),
        ));

        i.as_inner_mut().as_inner_mut().install_image(content);
        i
    }
    fn set_scale(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, policy: types::ImageScalePolicy) {
        if self.scale != policy {
            self.scale = policy;
            self.base.invalidate();
        }
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.scale
    }
}

impl ControlInner for TestableImage {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &controls::Container, x: i32, y: i32, pw: u16, ph: u16) {
        let selfptr = member as *mut _ as *mut c_void;
        let (hwnd, id) = unsafe {
            self.base.hwnd = parent.native_id() as windef::HWND; // required for measure, as we don't have own hwnd yet
            let (w, h, _) = self.measure(member, control, pw, ph);
            create_control_hwnd(
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                self.base.hwnd,
                0,
                WINDOW_CLASS.as_ptr(),
                "",
                winuser::SS_BITMAP | winuser::SS_CENTERIMAGE | winuser::WS_TABSTOP,
                selfptr,
                Some(handler),
            )
        };
        self.base.hwnd = hwnd;
        self.base.subclass_id = id;
    }
    fn on_removed_from_container(&mut self, _member: &mut MemberBase, _control: &mut ControlBase, _: &controls::Container) {
        destroy_hwnd(self.base.hwnd, self.base.subclass_id, Some(handler));
        self.base.hwnd = 0 as windef::HWND;
        self.base.subclass_id = 0;
    }

    fn parent(&self) -> Option<&controls::Member> {
        self.base.parent().map(|p| p.as_member())
    }
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.parent_mut().map(|p| p.as_member_mut())
    }
    fn root(&self) -> Option<&controls::Member> {
        self.base.root().map(|p| p.as_member())
    }
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.base.root_mut().map(|p| p.as_member_mut())
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &plygui_api::markup::Markup, registry: &mut plygui_api::markup::MarkupRegistry) {
        use plygui_api::markup::MEMBER_TYPE_IMAGE;
        fill_from_markup_base!(self, member, markup, registry, Image, [MEMBER_TYPE_IMAGE]);
        //TODO image source
    }
}

impl HasLayoutInner for TestableImage {
    fn on_layout_changed(&mut self, _base: &mut MemberBase) {
        self.base.invalidate();
    }
}

impl HasNativeIdInner for TestableImage {
    type Id = common::Hwnd;

    unsafe fn native_id(&self) -> Self::Id {
        self.base.hwnd.into()
    }
}

impl HasSizeInner for TestableImage {
    fn on_size_set(&mut self, base: &mut MemberBase, (width, height): (u16, u16)) -> bool {
        use plygui_api::controls::HasLayout;

        let this = base.as_any_mut().downcast_mut::<Image>().unwrap();
        this.set_layout_width(layout::Size::Exact(width));
        this.set_layout_width(layout::Size::Exact(height));
        self.base.invalidate();
        true
    }
}

impl HasVisibilityInner for TestableImage {
    fn on_visibility_set(&mut self, _base: &mut MemberBase, value: types::Visibility) -> bool {
        self.base.on_set_visibility(value)
    }
}

impl MemberInner for TestableImage {}

impl Drawable for TestableImage {
    fn draw(&mut self, _member: &mut MemberBase, control: &mut ControlBase) {
        self.base.draw(control.coords, control.measured);
    }
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => w,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => {
                        let mut bm: wingdi::BITMAP = unsafe { mem::zeroed() };
                        unsafe {
                            wingdi::GetObjectW(self.bmp as *mut c_void, mem::size_of::<wingdi::BITMAP>() as i32, &mut bm as *mut _ as *mut c_void);
                        }
                        bm.bmWidth as u16
                    }
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => h,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => {
                        let mut bm: wingdi::BITMAP = unsafe { mem::zeroed() };
                        unsafe {
                            wingdi::GetObjectW(self.bmp as *mut c_void, mem::size_of::<wingdi::BITMAP>() as i32, &mut bm as *mut _ as *mut c_void);
                        }
                        bm.bmHeight as u16
                    }
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

/*
#[allow(dead_code)]
pub(crate) fn spawn() -> Box<controls::Control> {
    use super::NewImage;

    Image::with_content().into_control()
}
*/

unsafe extern "system" fn handler(hwnd: windef::HWND, msg: minwindef::UINT, wparam: minwindef::WPARAM, lparam: minwindef::LPARAM, _: usize, param: usize) -> isize {
    let sc: &mut Image = mem::transmute(param);
    let ww = winuser::GetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA);
    if ww == 0 {
        winuser::SetWindowLongPtrW(hwnd, winuser::GWLP_USERDATA, param as isize);
    }
    match msg {
        winuser::WM_SIZE => {
            let width = lparam as u16;
            let height = (lparam >> 16) as u16;

            sc.call_on_size(width, height);
        }
        winuser::WM_PAINT => {
            use plygui_api::controls::HasSize;

            let (pw, ph) = sc.size();
            let sc = sc.as_inner_mut().as_inner_mut();
            let hoffs = DEFAULT_PADDING;
            let voffs = DEFAULT_PADDING;
            let hdiff = hoffs + DEFAULT_PADDING;
            let vdiff = voffs + DEFAULT_PADDING;
            let inner_h = pw as i32 - hdiff;
            let inner_v = ph as i32 - vdiff;

            let mut bm: wingdi::BITMAP = mem::zeroed();
            let mut ps: winuser::PAINTSTRUCT = mem::zeroed();

            let hdc = winuser::BeginPaint(hwnd, &mut ps);
            let hdc_mem = wingdi::CreateCompatibleDC(hdc);
            wingdi::SelectObject(hdc_mem, sc.bmp as *mut c_void); //let hbm_old =
            wingdi::GetObjectW(sc.bmp as *mut c_void, mem::size_of::<wingdi::BITMAP>() as i32, &mut bm as *mut _ as *mut c_void);

            let (wrate, hrate) = (inner_h as f32 / bm.bmWidth as f32, inner_v as f32 / bm.bmHeight as f32);
            let less_rate = fmin(wrate, hrate);

            let blendfunc = wingdi::BLENDFUNCTION {
                BlendOp: 0,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: 1,
            };

            let (dst_x, dst_y, dst_w, dst_h, src_x, src_y, src_w, src_h) = match sc.scale {
                types::ImageScalePolicy::FitCenter => {
                    let bm_h = (bm.bmWidth as f32 * less_rate) as i32;
                    let bm_v = (bm.bmHeight as f32 * less_rate) as i32;
                    let xoffs = (pw as i32 - bm_h) / 2;
                    let yoffs = (ph as i32 - bm_v) / 2;
                    (xoffs, yoffs, bm_h, bm_v, 0, 0, bm.bmWidth, bm.bmHeight)
                }
                types::ImageScalePolicy::CropCenter => {
                    let half_diff_h = (bm.bmWidth - pw as i32) / 2;
                    let half_diff_v = (bm.bmHeight - ph as i32) / 2;
                    (
                        hoffs + cmp::min(hoffs, half_diff_h).abs(),
                        voffs + cmp::min(voffs, half_diff_v).abs(),
                        cmp::min(pw as i32, bm.bmWidth),
                        cmp::min(ph as i32, bm.bmHeight),
                        cmp::max(0, half_diff_h),
                        cmp::max(0, half_diff_v),
                        cmp::min(bm.bmWidth, inner_h),
                        cmp::min(bm.bmHeight, inner_v),
                    )
                }
            };
            println!("{}/{}/{}/{} s {}/{}/{}/{}", dst_x, dst_y, dst_w, dst_h, src_x, src_y, src_w, src_h);
            wingdi::GdiAlphaBlend(hdc, dst_x, dst_y, dst_w, dst_h, hdc_mem, src_x, src_y, src_w, src_h, blendfunc);

            wingdi::DeleteDC(hdc_mem);
            winuser::EndPaint(hwnd, &ps);
        }
        _ => {}
    }

    commctrl::DefSubclassProc(hwnd, msg, wparam, lparam)
}

fn fmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
/*fn fmax(a: f32, b: f32) -> f32 {
    // leave for future non-centered fit
    if a > b {
        a
    } else {
        b
    }
}*/

default_impls_as!(Image);
