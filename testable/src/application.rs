use std::{thread, time};

use super::common::*;
use super::*;

pub struct TestableApplication {
    pub(crate) root: *mut Application,
    name: String,
    sleep: u32,
    windows: Vec<TestableId>,
    trays: Vec<TestableId>,
}

pub type Application = AApplication<TestableApplication>;

impl ApplicationInner for TestableApplication {
    fn get() -> Box<Application> {
        let mut w = Box::new(AApplication::with_inner(
            TestableApplication {
            	root: ptr::null_mut(),
                name: String::new(), //name.into(), // TODO later
                windows: Vec::with_capacity(1),
                trays: Vec::with_capacity(0),
            },
        ));
        w.inner_mut().root = w.as_mut();
        w
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window> {
        let w = window::TestableWindow::with_params(title, size, menu);
        self.windows.push(unsafe { TestableId::from_outer(w.native_id()) });
        w
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn Tray> {
    	let tray = tray::TestableTray::with_params(title, menu);
        self.trays.push(unsafe { TestableId::from_outer(tray.native_id()) });
        tray
    }
    fn remove_window(&mut self, id: Self::Id) {
    	self.windows.retain(|t| *t != id);
    }
    fn remove_tray(&mut self, id: Self::Id) {
    	self.trays.retain(|t| *t != id);
    }
    fn name<'a>(&'a self) -> Cow<'a, str> {
        Cow::Borrowed(self.name.as_str())
    }
    fn frame_sleep(&self) -> u32 {
        self.sleep
    }
    fn set_frame_sleep(&mut self, value: u32) {
        self.sleep = value;
    }
    fn start(&mut self) {
    	for window in self.windows.as_slice() {
    		let window = common::member_from_id::<window::Window>(window.clone().into()).unwrap();
    		window.inner_mut().inner_mut().inner_mut().inner_mut().draw();
    	}
        loop {
            let mut frame_callbacks = 0;
            let w = unsafe {&mut *self.root}.base_mut();
            while frame_callbacks < defaults::MAX_FRAME_CALLBACKS {
                match w.queue().try_recv() {
                    Ok(mut cmd) => {
                        if (cmd.as_mut())(unsafe { &mut *self.root } ) {
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
            if self.sleep > 0 {
                thread::sleep(time::Duration::from_millis(self.sleep as u64));
            }
            if self.windows.len() < 1 && self.trays.len() < 1 {
                break;
            }
        }
    }
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member> {
        for window in self.windows.as_mut_slice() {
            if let Some(window) = common::member_from_id::<window::Window>((*window).into()) {
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
                let found = controls::Container::find_control_mut(window, arg.clone()).map(|control| control.as_member_mut());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_mut_slice() {
        	if let Some(tray) = common::member_from_id::<tray::Tray>((*tray).into()) {
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
        }
        None
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member> {
        for window in self.windows.as_slice() {
            if let Some(window) = common::member_from_id::<window::Window>((*window).into()) {
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
                let found = controls::Container::find_control(window, arg.clone()).map(|control| control.as_member());
                if found.is_some() {
                    return found;
                }
            }
        }
        for tray in self.trays.as_slice() {
            if let Some(tray) = common::member_from_id::<tray::Tray>((*tray).into()) {
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
        }
        None
    }
    fn exit(&mut self, skip_on_close: bool) -> bool {
        for window in self.windows.as_mut_slice() {
            if !common::member_from_id::<window::Window>((*window).into()).unwrap().close(skip_on_close) {
                return false;
            }
        }
        for tray in self.trays.as_mut_slice() {
            if !common::member_from_id::<tray::Tray>((*tray).into()).unwrap().close(skip_on_close) {
                return false;
            }
        }
        true
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a> {
        Box::new(MemberIterator {
            inner: self,
            is_tray: false,
            index: 0,
            needs_window: true,
            needs_tray: true,
        })
    }
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a> {
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
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        (self.root as *mut MemberBase).into()
    }
}

impl Drop for TestableApplication {
    fn drop(&mut self) {
        self.windows.clear();
        self.trays.clear();
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
    type Item = &'a dyn (Member);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get(self.index).map(|tray| common::member_from_id::<tray::Tray>((*tray).into()).unwrap() as &dyn Member)
        } else if self.needs_window {
            self.inner.windows.get(self.index).map(|window| common::member_from_id::<window::Window>((*window).into()).unwrap() as &dyn Member)
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
    type Item = &'a mut dyn (Member);

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_tray && self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get_mut(self.index).map(|tray| common::member_from_id::<tray::Tray>((*tray).into()).unwrap() as &mut dyn Member)
        } else if self.needs_window {
            self.inner.windows.get_mut(self.index).map(|window| common::member_from_id::<window::Window>((*window).into()).unwrap() as &mut dyn Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}
