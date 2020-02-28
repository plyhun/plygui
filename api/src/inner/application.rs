use super::auto::{AsAny, HasInner, OnFrame, Abstract};
use super::has_native_id::{HasNativeId, HasNativeIdInner};
use super::member::Member;
use super::closeable::Closeable;
use super::seal::Sealed;
use super::window::{NewWindow, Window};
use super::tray::{NewTray, Tray};

use crate::{callbacks, types, ids};

use std::any::Any;
use std::borrow::Cow;
use std::sync::mpsc;

define! {
    Application: HasNativeId {
        base: {
            pub windows: Vec<Box<dyn Window>>,
            pub trays: Vec<Box<dyn Tray>>,
            queue: mpsc::Receiver<OnFrame>,
            sender: mpsc::Sender<OnFrame>,
        },
        extends: {
            AsAny + Sealed
        }
        outer: {
            fn name(&self) -> ::std::borrow::Cow<'_, str>;
            fn start(&mut self);
            fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member>;
            fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member>;
            fn exit(self: Box<Self>);
            fn prepare_exit(&mut self);
            fn on_frame(&mut self, cb: OnFrame);
            fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<OnFrame>;
            
            fn add_root(&mut self, m: Box<dyn Closeable>) -> &mut dyn Member;
            fn close_root(&mut self, arg: types::FindBy, skip_callbacks: bool) -> bool;
        
            fn frame_sleep(&self) -> u32;
            fn set_frame_sleep(&mut self, value: u32);
            
            fn roots<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a>; //E0562 :(
            fn roots_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a>; //E0562 :(
        },
        inner: {
            fn add_root(&mut self, m: Box<dyn Closeable>) -> &mut dyn Member;
            fn close_root(&mut self, arg: types::FindBy, skip_callbacks: bool) -> bool;
        
            fn name(&self) -> Cow<'_, str>;
            fn start(&mut self);
        
            fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member>;
            fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member>;
        
            fn exit(&mut self);
        
            fn on_frame_async_feeder(&mut self, feeder: callbacks::AsyncFeeder<OnFrame>) -> callbacks::AsyncFeeder<OnFrame> {
                feeder
            }
            fn on_frame(&mut self, feeder: &mut callbacks::AsyncFeeder<OnFrame>, cb: OnFrame) {
                let _ = feeder.feed(cb);
            }
            fn frame_sleep(&self) -> u32;
            fn set_frame_sleep(&mut self, value: u32);
            fn roots<'a>(&'a self) -> Box<dyn Iterator<Item = &(dyn Member)> + 'a>;
            fn roots_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut (dyn Member)> + 'a>;
        }
        constructor: {
            fn with_name<S: AsRef<str>>(name: S) -> Box<dyn Application>;
        }
        inner_constructor_params: {
            name: &str
        }
    }
}

pub trait CloseableSpawner {
    fn new_window<W: NewWindow>(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> ids::Id;
    fn new_tray<T: NewTray>(&mut self, title: &str, icon: image::DynamicImage, menu: types::Menu) -> ids::Id;
}

impl Drop for ApplicationBase {
    fn drop(&mut self) {
        self.windows.clear();
        self.trays.clear();
    }
}
impl<T: ApplicationInner> AApplication<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        let (tx, rx) = mpsc::channel();
        AApplication {
            base: ApplicationBase { windows: Vec::with_capacity(1), trays: Vec::with_capacity(0), sender: tx, queue: rx },
            inner: inner,
        }
    }
}
impl ApplicationBase {
    pub fn sender(&mut self) -> &mut mpsc::Sender<OnFrame> {
        &mut self.sender
    }
    pub fn queue(&mut self) -> &mut mpsc::Receiver<OnFrame> {
        &mut self.queue
    }
}
impl<T: ApplicationInner> HasNativeId for AApplication<T> {
    #[inline]
    unsafe fn native_id(&self) -> usize {
        self.inner().native_id().into()
    }
}
impl<T: ApplicationInner> Application for AApplication<T> {
    #[inline]
    fn name(&self) -> Cow<'_, str> {
        self.inner().name()
    }
    #[inline]
    fn start(&mut self) {
        self.inner_mut().start()
    }
    #[inline]
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member> {
        self.inner_mut().find_member_mut(arg)
    }
    #[inline]
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member> {
        self.inner().find_member(arg)
    }
    #[inline]
    fn exit(mut self: Box<Self>) {
        self.prepare_exit();
    }
    #[inline]
    fn prepare_exit(&mut self) {
        self.inner_mut().exit();
    }
    #[inline]
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<OnFrame> {
        let feeder = self.base_mut().sender().clone();
        self.inner_mut().on_frame_async_feeder(feeder.into())
    }
    #[inline]
    fn on_frame(&mut self, cb: OnFrame) {
        let mut feeder = self.base_mut().sender().clone().into();
        self.inner_mut().on_frame(&mut feeder, cb)
    }
    #[inline]
    fn frame_sleep(&self) -> u32 {
        self.inner().frame_sleep()
    }
    #[inline]
    fn set_frame_sleep(&mut self, value: u32) {
        self.inner_mut().set_frame_sleep(value)
    }
    #[inline]
    fn roots<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a> {
        self.inner().roots()
    }
    #[inline]
    fn roots_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a> {
        self.inner_mut().roots_mut()
    }
    #[inline]
    fn add_root(&mut self, m: Box<dyn Closeable>) -> &mut dyn Member {
        self.inner_mut().add_root(m)
    }
    #[inline]
    fn close_root(&mut self, arg: types::FindBy, skip_callbacks: bool) -> bool {
        self.inner_mut().close_root(arg, skip_callbacks)
    }
    #[inline]
    fn as_application(&self) -> &(dyn Application + 'static) { self }
    
    #[inline]
    fn as_application_mut(&mut self) -> &mut (dyn Application + 'static) { self }
    
    #[inline]
    fn into_application(self: Box<Self>) -> Box<(dyn Application + 'static)> { self }
}
impl<T: ApplicationInner> AsAny for AApplication<T> {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl<T: ApplicationInner> AApplication<T> {
    #[inline]
    pub fn base(&self) -> &ApplicationBase {
        &self.base
    }
    #[inline]
    pub fn base_mut(&mut self) -> &mut ApplicationBase {
        &mut self.base
    }
}
impl<II: ApplicationInner, T: HasInner<I = II> + Abstract + 'static> ApplicationInner for T {
    fn with_name<S: AsRef<str>>(name: S) -> Box<dyn Application> {
        <<Self as HasInner>::I as ApplicationInner>::with_name(name)
    }
    fn add_root(&mut self, m: Box<dyn Closeable>) -> &mut dyn Member {
        self.inner_mut().add_root(m)
    }
    fn close_root(&mut self, arg: types::FindBy, skip_callbacks: bool) -> bool {
        self.inner_mut().close_root(arg, skip_callbacks)
    }

    fn name(&self) -> Cow<str> {
        self.inner().name()
    }
    fn start(&mut self) {
        self.inner_mut().start()
    }

    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member> {
        self.inner_mut().find_member_mut(arg)
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member> {
        self.inner().find_member(arg)
    }

    fn exit(&mut self) {
        self.inner_mut().exit()
    }

    fn frame_sleep(&self) -> u32 {
        self.inner().frame_sleep()
    }
    fn set_frame_sleep(&mut self, value: u32) {
        self.inner_mut().set_frame_sleep(value)
    }
    fn roots<'a>(&'a self) -> Box<dyn Iterator<Item = &(dyn Member)> + 'a> {
        self.inner().roots()
    }
    fn roots_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut (dyn Member)> + 'a> {
        self.inner_mut().roots_mut()
    }
}
impl<T: ApplicationInner> NewApplication for AApplication<T> {
    #[inline]
    fn with_name<S: AsRef<str>>(name: S) -> Box<dyn Application> {
        T::with_name(name)
    }
}
impl<'a> CloseableSpawner for &'a mut dyn Application {
    fn new_window<W: NewWindow>(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> ids::Id {
        let window = W::with_params(*self, title, size, menu);
        self.add_root(window.into_closeable()).id()
    }
    fn new_tray<T: NewTray>(&mut self, title: &str, icon: image::DynamicImage, menu: types::Menu) -> ids::Id {
        let tray = T::with_params(*self, title, icon, menu);
        self.add_root(tray.into_closeable()).id()
    }
}
impl CloseableSpawner for Box<dyn Application> {
    fn new_window<W: NewWindow>(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> ids::Id {
        self.as_mut().new_window::<W>(title, size, menu)
    }
    fn new_tray<T: NewTray>(&mut self, title: &str, icon: image::DynamicImage, menu: types::Menu) -> ids::Id {
        self.as_mut().new_tray::<T>(title, icon, menu)
    }
}
impl<T: ApplicationInner> super::seal::Sealed for AApplication<T> {}
