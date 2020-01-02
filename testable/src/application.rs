use std::{thread, time, marker::PhantomData};

use crate::common::{self, *};
use crate::{window, tray};
use crate::plygui_api::controls::Member;

pub type Application = AApplication<TestableApplication>;

#[repr(C)]
pub struct TestableApplicationInner<O: controls::Application> {
    pub(crate) root: *mut Application,
    name: String,
    sleep: u32,
    windows: Vec<TestableId>,
    trays: Vec<TestableId>,
    _marker: PhantomData<O>
}

impl<O: controls::Application> ApplicationInner for TestableApplicationInner<O> {
    fn get() -> Box<dyn controls::Application> {
        please_override_this!()
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        let w = window::TestableWindow::with_params(title, size, menu);
        self.windows.push(unsafe { TestableId::from_outer(w.native_id()) });
        w
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
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
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
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
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
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

impl<O: controls::Application> HasNativeIdInner for TestableApplicationInner<O> {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        (self.root as *mut MemberBase).into()
    }
}

impl<O: controls::Application> Drop for TestableApplicationInner<O> {
    fn drop(&mut self) {
        self.windows.clear();
        self.trays.clear();
    }
}

pub struct TestableApplication {
    inner: TestableApplicationInner<Application>
}
impl HasInner for TestableApplication {
    type I = TestableApplicationInner<Application>;

    fn inner(&self) -> &Self::I { &self.inner }
    fn inner_mut(&mut self) -> &mut Self::I { &mut self.inner }
    fn into_inner(self) -> Self::I { panic!("Cannot unwrap an Application!") }
}
impl HasNativeIdInner for TestableApplication {
    type Id = common::TestableId;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl ApplicationInner for TestableApplication {
    fn get() -> Box<dyn controls::Application> {
        let mut w = Box::new(AApplication::with_inner(
            TestableApplication {
                inner: TestableApplicationInner {
                	root: ptr::null_mut(),
                    name: String::new(), //name.into(), // TODO later
                    sleep: 0,
                    windows: Vec::with_capacity(1),
                    trays: Vec::with_capacity(0),
                    _marker: PhantomData
                },
            }
        ));
        w.inner_mut().inner_mut().root = w.as_mut();
        w
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        self.inner_mut().new_window(title, size, menu)
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
    	self.inner_mut().new_tray(title, menu)
    }
    fn remove_window(&mut self, id: Self::Id) {
    	self.inner_mut().remove_window(id)
    }
    fn remove_tray(&mut self, id: Self::Id) {
    	self.inner_mut().remove_tray(id)
    }
    fn name<'a>(&'a self) -> Cow<'a, str> {
        self.inner().name()
    }
    fn frame_sleep(&self) -> u32 {
        self.inner().frame_sleep()
    }
    fn set_frame_sleep(&mut self, value: u32) {
        self.inner_mut().set_frame_sleep(value)
    }
    fn start(&mut self) {
    	self.inner_mut().start()
    }
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
        self.inner_mut().find_member_mut(arg)
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
        self.inner().find_member(arg)
    }
    fn exit(&mut self, skip_on_close: bool) -> bool {
        self.inner_mut().exit(skip_on_close)
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        self.inner().members()
    }
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        self.inner_mut().members_mut()
    }
}
impl controls::Application for Application {
    #[inline]
    default fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        self.inner_mut().new_window(title, size, menu)
    }
    #[inline]
    default fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        self.inner_mut().new_tray(title, menu)
    }
    #[inline]
    default fn name(&self) -> Cow<'_, str> {
        self.inner().name()
    }
    #[inline]
    default fn start(&mut self) {
        self.inner_mut().start()
    }
    #[inline]
    default fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member> {
        self.inner_mut().find_member_mut(arg)
    }
    #[inline]
    default fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member> {
        self.inner().find_member(arg)
    }
    #[inline]
    default fn exit(mut self: Box<Self>, skip_on_close: bool) -> bool {
        let exited = self.inner_mut().exit(skip_on_close);
        if exited {
            self.runtime_deinit();
        }
        exited
    }
    #[inline]
    default fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<callbacks::OnFrame> {
        let feeder = self.base_mut().sender().clone();
        self.inner_mut().on_frame_async_feeder(feeder.into())
    }
    #[inline]
    default fn on_frame(&mut self, cb: callbacks::OnFrame) {
        let mut feeder = self.base_mut().sender().clone().into();
        self.inner_mut().on_frame(&mut feeder, cb)
    }
    #[inline]
    default fn frame_sleep(&self) -> u32 {
        self.inner().frame_sleep()
    }
    #[inline]
    default fn set_frame_sleep(&mut self, value: u32) {
        self.inner_mut().set_frame_sleep(value)
    }
    #[inline]
    default fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a> {
        self.inner().members()
    }
    #[inline]
    default fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a> {
        self.inner_mut().members_mut()
    }
}

struct MemberIterator<'a, O: controls::Application> {
    inner: &'a TestableApplicationInner<O>,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a, O: controls::Application> Iterator for MemberIterator<'a, O> {
    type Item = &'a (dyn controls::Member);

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get(self.index).map(|tray| common::member_from_id::<tray::Tray>((*tray).into()).unwrap() as &dyn controls::Member)
        } else if self.needs_window {
            self.inner.windows.get(self.index).map(|window| common::member_from_id::<window::Window>((*window).into()).unwrap() as &dyn controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}

struct MemberIteratorMut<'a, O: controls::Application> {
    inner: &'a mut TestableApplicationInner<O>,
    needs_window: bool,
    needs_tray: bool,
    is_tray: bool,
    index: usize,
}
impl<'a, O: controls::Application> Iterator for MemberIteratorMut<'a, O> {
    type Item = &'a mut (dyn controls::Member);

    fn next(&mut self) -> Option<Self::Item> {
        if self.needs_tray && self.index >= self.inner.windows.len() {
            self.is_tray = true;
            self.index = 0;
        }
        let ret = if self.needs_tray && self.is_tray {
            self.inner.trays.get_mut(self.index).map(|tray| common::member_from_id::<tray::Tray>((*tray).into()).unwrap() as &mut dyn controls::Member)
        } else if self.needs_window {
            self.inner.windows.get_mut(self.index).map(|window| common::member_from_id::<window::Window>((*window).into()).unwrap() as &mut dyn controls::Member)
        } else {
            return None;
        };
        self.index += 1;
        ret
    }
}
