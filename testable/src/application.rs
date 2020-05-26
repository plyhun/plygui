use crate::common::{self, *};
use std::{thread, time, any::TypeId};

pub struct TestableApplication {
    pub(crate) root: *mut Application,
    name: String,
    sleep: u32,
}

pub type Application = AApplication<TestableApplication>;

impl<O: controls::Application> NewApplicationInner<O> for TestableApplication {
    fn with_uninit_params(u: &mut mem::MaybeUninit<O>, name: &str) -> Self {
        TestableApplication {
        	root: u as *mut _ as *mut Application,
            name: name.into(),
            sleep: 0,
        }
    }
}

impl TestableApplication {
    pub fn get(&self) -> &Application {
        unsafe { &*self.root }
    }
    pub fn get_mut(&mut self) -> &mut Application {
        unsafe { &mut *self.root }
    }
}

impl ApplicationInner for TestableApplication {
    fn with_name<S: AsRef<str>>(name: S) -> Box<dyn controls::Application> {
        let mut b: Box<mem::MaybeUninit<Application>> = Box::new_uninit();
        let ab = AApplication::with_inner(
            <Self as NewApplicationInner<Application>>::with_uninit_params(b.as_mut(), name.as_ref()),
        );
        unsafe {
	        b.as_mut_ptr().write(ab);
	        b.assume_init()
        }
    }
    fn add_root(&mut self, m: Box<dyn controls::Closeable>) -> &mut dyn controls::Member {
        let base = &mut self.get_mut().base; 
        
        let is_window = m.as_any().type_id() == TypeId::of::<crate::window::Window>();
        let is_tray = m.as_any().type_id() == TypeId::of::<crate::tray::Tray>();
        
        if is_window {
            let i = base.windows.len();
            base.windows.push(m.into_any().downcast::<crate::window::Window>().unwrap());
            return base.windows[i].as_mut().as_member_mut();
        }
        
        if is_tray {
            let i = base.trays.len();
            base.trays.push(m.into_any().downcast::<crate::tray::Tray>().unwrap());
            return base.trays[i].as_mut().as_member_mut();
        }
        
        panic!("Unsupported Closeable: {:?}", m.as_any().type_id());
    }
    fn close_root(&mut self, arg: types::FindBy, skip_callbacks: bool) -> bool {
        let base = &mut self.get_mut().base; 
        match arg {
            types::FindBy::Id(id) => {
                (0..base.windows.len()).into_iter().find(|i| if base.windows[*i].id() == id 
                    && base.windows[*i].as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(skip_callbacks) {
                        base.windows.remove(*i);
                        true
                    } else {
                        false
                }).is_some()
                || 
                (0..base.trays.len()).into_iter().find(|i| if base.trays[*i].id() == id 
                    && base.trays[*i].as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(skip_callbacks) {
                        base.trays.remove(*i);
                        true
                    } else {
                        false
                }).is_some()
            }
            types::FindBy::Tag(ref tag) => {
                (0..base.windows.len()).into_iter().find(|i| if base.windows[*i].tag().is_some() && base.windows[*i].tag().unwrap() == Cow::Borrowed(tag.into()) 
                    && base.windows[*i].as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(skip_callbacks) {
                        base.windows.remove(*i);
                        true
                    } else {
                        false
                }).is_some()
                || 
                (0..base.trays.len()).into_iter().find(|i| if base.trays[*i].tag().is_some() && base.trays[*i].tag().unwrap() == Cow::Borrowed(tag.into()) 
                    && base.trays[*i].as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(skip_callbacks) {
                        base.trays.remove(*i);
                        true
                    } else {
                        false
                }).is_some()
            }
        }
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
        {
        	let base = &mut self.get_mut().base; 
            for window in base.windows.as_mut_slice() {
        		window.as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().inner_mut().draw();
        	}
        }
        loop {
            let mut frame_callbacks = 0;
            let w = &mut unsafe {&mut *self.root}.base;
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
            let base = &mut self.get_mut().base; 
            if base.windows.len() < 1 && base.trays.len() < 1 {
                break;
            }
        }
    }
    fn find_member_mut<'a>(&'a mut self, arg: &'a types::FindBy) -> Option<&'a mut dyn controls::Member> {
        let base = &mut self.get_mut().base; 
        for window in base.windows.as_mut_slice() {
             match arg {
                types::FindBy::Id(id) => {
                    if window.id() == *id {
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
            let found = controls::Container::find_control_mut(window.as_mut(), arg).map(|control| control.as_member_mut());
            if found.is_some() {
                return found;
            }
        }
        for tray in base.trays.as_mut_slice() {
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
    fn find_member<'a>(&'a self, arg: &'a types::FindBy) -> Option<&'a dyn controls::Member> {
        let base = &self.get().base; 
        for window in base.windows.as_slice() {
            match arg {
                types::FindBy::Id(id) => {
                    if window.id() == *id {
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
            let found = controls::Container::find_control(window.as_ref(), arg).map(|control| control.as_member());
            if found.is_some() {
                return found;
            }
        }
        for tray in base.trays.as_slice() {
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
    fn exit(&mut self) {
        let base = &mut self.get_mut().base; 
        for mut window in base.windows.drain(..) {
            window.as_any_mut().downcast_mut::<crate::window::Window>().unwrap().inner_mut().inner_mut().inner_mut().inner_mut().close(true);
        }
        for mut tray in base.trays.drain(..) {
            tray.as_any_mut().downcast_mut::<crate::tray::Tray>().unwrap().inner_mut().close(true);
        }
    }
    fn roots<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        self.get().roots()
    }
    fn roots_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        self.get_mut().roots_mut()
    }
}

impl HasNativeIdInner for TestableApplication {
    type Id = common::TestableId;

    fn native_id(&self) -> Self::Id {
        (self.root as *mut MemberBase).into()
    }
}
