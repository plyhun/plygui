use super::common::*;
use super::*;

use plygui_api::controls;
use plygui_api::types;

pub struct TestableApplication {
    pub(crate) root: *mut Application,
    name: String,
    windows: Vec<Rc<RefCell<window::TestableWindow>>>,
    trays: Vec<Rc<RefCell<tray::TestableTray>>>,
}

pub type Application = ::plygui_api::development::Application<TestableApplication>;

impl ApplicationInner for TestableApplication {
    fn get() -> Box<Application> {
        let mut w = Box::new(Application::with_inner(
            TestableApplication {
            	root: ptr::null_mut(),
                name: String::new(), //name.into(), // TODO later
                windows: Vec::with_capacity(1),
                trays: Vec::with_capacity(0),
            },
            (),
        ));
        w.as_inner_mut().root = w.as_ptr();
        w
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let w = window::TestableWindow::with_params(title, size, menu);
        self.windows.push(w.as_inner().as_inner().clone());
        w
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        let mut tray = tray::TestableTray::with_params(title, menu);
        self.trays.push(tray.as_inner().clone());
        tray
    }
    fn remove_window(&mut self, id: Self::Id) {
        self.windows.retain(|t| t.borrow().id() != id);
    }
    fn remove_tray(&mut self, id: Self::Id) {
        self.trays.retain(|t| t.borrow().id() != id);
    }
    fn name<'a>(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(self.name.as_str())
    }
    fn start(&mut self) {
        loop {
            let mut frame_callbacks = 0;
            if let Some(w) = unsafe { cast_hwnd::<Application>(self.root) } {
                let w = w.base_mut();
                while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                    match w.queue().try_recv() {
                        Ok(mut cmd) => {
                            if (cmd.as_mut())(unsafe { cast_hwnd::<Application>(self.root) }.unwrap()) {
                                let _ = w.sender().send(cmd);
                            }
                            frame_callbacks += 1;
                        }
                        Err(e) => match e {
                            mpsc::TryRecvError::Empty => break,
                            mpsc::TryRecvError::Disconnected => unreachable!(),
                        },
                    }
                }
            }
            /*
            unsafe {
                synchapi::Sleep(10);

                if winuser::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winuser::PM_REMOVE) > 0 {
                    winuser::TranslateMessage(&mut msg);
                    winuser::DispatchMessageW(&mut msg);
                }
            }

            i = 0;
            while i < self.windows.len() {
                if dispatch_window(self.windows[i]) < 0 {
                    self.windows.remove(i);
                } else {
                    i += 1;
                }
            }
            if self.windows.len() < 1 && self.trays.len() < 1 {
                unsafe {
                    winuser::DestroyWindow(self.root);
                }
                break;
            }
            */
        }
    }
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
        use plygui_api::controls::Member;

        for window in self.windows.as_mut_slice() {
            if let Some(window) = common::member_from_hwnd::<window::Window>(*window) {
                match arg {
                    types::FindBy::Id(id) => {
                        if window.id() == id {
                            return Some(window.as_member_mut());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = window.tag() {
                            if tag.as_str() == mytag {
                                return Some(window.as_member_mut());
                            }
                        }
                    }
                }
                let found = window.find_control_mut(arg.clone()).map(|control| control.as_member_mut());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_mut_slice() {
            let tray = unsafe { &mut **tray };
            match arg {
                types::FindBy::Id(ref id) => {
                    if tray.id() == *id {
                        return Some(tray.as_member_mut());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = tray.tag() {
                        if tag.as_str() == mytag {
                            return Some(tray.as_member_mut());
                        }
                    }
                }
            }
        }
        None
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
        use plygui_api::controls::Member;

        for window in self.windows.as_slice() {
            if let Some(window) = common::member_from_hwnd::<window::Window>(*window) {
                match arg {
                    types::FindBy::Id(id) => {
                        if window.id() == id {
                            return Some(window.as_member());
                        }
                    }
                    types::FindBy::Tag(ref tag) => {
                        if let Some(mytag) = window.tag() {
                            if tag.as_str() == mytag {
                                return Some(window.as_member());
                            }
                        }
                    }
                }
                let found = window.find_control(arg.clone()).map(|control| control.as_member());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_slice() {
            let tray = unsafe { &mut **tray };
            match arg {
                types::FindBy::Id(ref id) => {
                    if tray.id() == *id {
                        return Some(tray.as_member());
                    }
                }
                types::FindBy::Tag(ref tag) => {
                    if let Some(mytag) = tray.tag() {
                        if tag.as_str() == mytag {
                            return Some(tray.as_member());
                        }
                    }
                }
            }
        }
        None
    }
    fn exit(&mut self, skip_on_close: bool) -> bool {
        for window in self.windows.as_mut_slice() {
            if !common::member_from_hwnd::<window::Window>(*window).unwrap().close(skip_on_close) {
                return false;
            }
        }
        for tray in self.trays.as_mut_slice() {
            if !(unsafe { &mut **tray }.close(skip_on_close)) {
                return false;
            }
        }
        true
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        Box::new(MemberIterator {
            inner: self,
            is_tray: false,
            index: 0,
            needs_window: true,
            needs_tray: true,
        })
    }
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        Box::new(MemberIteratorMut {
            inner: self,
            is_tray: false,
            index: 0,
            needs_window: true,
            needs_tray: true,
        })
    }
}

impl HasNativeIdInner for TestableApplication {
    type Id = common::Hwnd;

    unsafe fn native_id(&self) -> Self::Id {
        self.root.into()
    }
}

impl Drop for TestableApplication {
    fn drop(&mut self) {
        for w in self.windows.drain(..) {
            destroy_hwnd(w, 0, None);
        }
        for _ in self.trays.drain(..) {}
        destroy_hwnd(self.root, 0, None);
    }
}

struct MemberIterator<'a> {
    inner: &'a TestableApplication,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a> Iterator for MemberIterator<'a> {
    type Item = &'a (controls::Member + 'static);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get(self.index).map(|tray| unsafe { &**tray } as &controls::Member)
        } else if self.needs_window {
            self.inner.windows.get(self.index).map(|window| common::member_from_hwnd::<window::Window>(*window).unwrap() as &controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}

struct MemberIteratorMut<'a> {
    inner: &'a mut TestableApplication,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a> Iterator for MemberIteratorMut<'a> {
    type Item = &'a mut (controls::Member);

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_tray && self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get_mut(self.index).map(|tray| unsafe { &mut **tray } as &mut controls::Member)
        } else if self.needs_window {
            self.inner.windows.get_mut(self.index).map(|window| common::member_from_hwnd::<window::Window>(*window).unwrap() as &mut controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}
