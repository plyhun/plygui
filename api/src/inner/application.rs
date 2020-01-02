use super::auto::{AsAny, HasInner, OnFrame, Abstract};
use super::has_native_id::{HasNativeId, HasNativeIdInner};
use super::member::Member;
use super::tray::Tray;
use super::window::Window;

use crate::{callbacks, runtime, types};

use std::any::Any;
use std::borrow::Cow;
use std::cell::UnsafeCell;
use std::rc::Rc;
use std::sync::mpsc;

pub trait Application: HasNativeId + AsAny + super::seal::Sealed {
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window>;
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn Tray>;
    fn name(&self) -> ::std::borrow::Cow<'_, str>;
    fn start(&mut self);
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member>;
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member>;
    fn exit(self: Box<Self>, skip_on_close: bool) -> bool;
    fn on_frame(&mut self, cb: OnFrame);
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<OnFrame>;

    fn frame_sleep(&self) -> u32;
    fn set_frame_sleep(&mut self, value: u32);
    
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a>; //E0562 :(
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a>; //E0562 :(
}
pub trait NewApplication {
    fn get() -> Box<dyn Application>;
}
pub trait ApplicationInner: HasNativeIdInner + 'static {
    fn get() -> Box<dyn Application>
    where
        Self: Sized;
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window>;
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn Tray>;

    fn remove_window(&mut self, id: Self::Id);
    fn remove_tray(&mut self, id: Self::Id);

    fn name(&self) -> Cow<'_, str>;
    fn start(&mut self);

    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member>;
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member>;

    fn exit(&mut self, skip_on_close: bool) -> bool;

    fn on_frame_async_feeder(&mut self, feeder: callbacks::AsyncFeeder<OnFrame>) -> callbacks::AsyncFeeder<OnFrame> {
        feeder
    }
    fn on_frame(&mut self, feeder: &mut callbacks::AsyncFeeder<OnFrame>, cb: OnFrame) {
        let _ = feeder.feed(cb);
    }
    fn frame_sleep(&self) -> u32;
    fn set_frame_sleep(&mut self, value: u32);
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &(dyn Member)> + 'a>;
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut (dyn Member)> + 'a>;
}
#[fundamental]
pub struct AApplication<T: ApplicationInner> {
    inner: Rc<UnsafeCell<ApplicationInnerWrapper<T>>>,
}
pub struct ApplicationBase {
    queue: mpsc::Receiver<OnFrame>,
    sender: mpsc::Sender<OnFrame>,
}
pub struct ApplicationInnerWrapper<T: ApplicationInner> {
    base: ApplicationBase,
    inner: T,
}
impl<T: ApplicationInner> NewApplication for AApplication<T> {
    #[inline]
    fn get() -> Box<dyn Application> {
        T::get()
    }
}
impl<T: ApplicationInner> AApplication<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        let (tx, rx) = mpsc::channel();
        AApplication {
            inner: Rc::new(UnsafeCell::new(ApplicationInnerWrapper {
                base: ApplicationBase { sender: tx, queue: rx },
                inner: inner,
            })),
        }
    }
    #[inline]
    pub fn runtime_deinit(&mut self) {
        runtime::deinit(&self.inner);
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
    default fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window> {
        self.inner_mut().new_window(title, size, menu)
    }
    #[inline]
    default fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn Tray> {
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
    default fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<OnFrame> {
        let feeder = self.base_mut().sender().clone();
        self.inner_mut().on_frame_async_feeder(feeder.into())
    }
    #[inline]
    default fn on_frame(&mut self, cb: OnFrame) {
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

impl<T: ApplicationInner> HasInner for AApplication<T> {
    type I = T;

    #[inline]
    fn inner(&self) -> &Self::I {
        unsafe { &(&*self.inner.get()).inner }
    }
    #[inline]
    fn inner_mut(&mut self) -> &mut Self::I {
        unsafe { &mut (&mut *self.inner.get()).inner }
    }
    #[inline]
    fn into_inner(self) -> Self::I {
        panic!("Never unwrap an Application");
    }
}
impl<T: ApplicationInner> AApplication<T> {
    #[inline]
    pub fn get() -> types::ApplicationResult {
        let (inner, ready) = runtime::get::<T>();
        if let Some(inner) = inner {
            types::ApplicationResult::Existing(Box::new(AApplication { inner }))
        } else if ready {
            types::ApplicationResult::ErrorNonUiThread
        } else {
            let app = T::get();
            runtime::init(app.as_any().downcast_ref::<Self>().unwrap().inner.clone());
            types::ApplicationResult::New(app)
        }
    }
    #[inline]
    pub fn base(&self) -> &ApplicationBase {
        unsafe { &(&*self.inner.get()).base }
    }
    #[inline]
    pub fn base_mut(&mut self) -> &mut ApplicationBase {
        unsafe { &mut (&mut *self.inner.get()).base }
    }
}
impl<II: ApplicationInner, T: HasInner<I = II> + Abstract + 'static> ApplicationInner for T {
    fn get() -> Box<dyn Application> {
        <<Self as HasInner>::I as ApplicationInner>::get()
    }
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window> {
        self.inner_mut().new_window(title, size, menu)
    }
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn Tray> {
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
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member> {
        self.inner_mut().find_member_mut(arg)
    }
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member> {
        self.inner().find_member(arg)
    }
    fn exit(&mut self, skip_on_close: bool) -> bool {
        self.inner_mut().exit(skip_on_close)
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a> {
        self.inner().members()
    }
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a> {
        self.inner_mut().members_mut()
    }
}
impl<T: ApplicationInner> super::seal::Sealed for AApplication<T> {}
