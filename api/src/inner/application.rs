use super::native_id::{HasNativeId, HasNativeIdInner};
use super::member::{MemberInner, AMember};

use crate::{callbacks, controls, ids, layout, runtime, types};

use std::any::Any;
use std::borrow::Cow;
use std::cell::UnsafeCell;
use std::rc::Rc;
use std::sync::mpsc;

#[cfg(feature = "type_check")]
use std::any::TypeId;

pub trait Application: HasNativeId + AsAny + development::seal::Sealed {
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window>;
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn Tray>;
    fn name(&self) -> ::std::borrow::Cow<'_, str>;
    fn start(&mut self);
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member>;
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member>;
    fn exit(self: Box<Self>, skip_on_close: bool) -> bool;
    fn on_frame(&mut self, cb: callbacks::OnFrame);
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<callbacks::OnFrame>;

    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a>; //E0562 :(
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a>; //E0562 :(
}

pub trait ApplicationInner: HasNativeIdInner + 'static {
    fn get() -> Box<Application<Self>> where Self: Sized;
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window>;
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray>;

    fn remove_window(&mut self, id: Self::Id);
    fn remove_tray(&mut self, id: Self::Id);

    fn name(&self) -> Cow<'_, str>;
    fn start(&mut self);

    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member>;
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member>;

    fn exit(&mut self, skip_on_close: bool) -> bool;

    fn on_frame_async_feeder(&mut self, feeder: callbacks::AsyncFeeder<callbacks::OnFrame>) -> callbacks::AsyncFeeder<callbacks::OnFrame> {
        feeder
    }
    fn on_frame(&mut self, feeder: &mut callbacks::AsyncFeeder<callbacks::OnFrame>, cb: callbacks::OnFrame) {
        let _ = feeder.feed(cb);
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &(dyn controls::Member)> + 'a>;
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut (dyn controls::Member)> + 'a>;
}
pub struct AApplication<T: ApplicationInner> {
    inner: Rc<UnsafeCell<ApplicationInnerWrapper<T>>>,
}
pub struct ApplicationBase {
    queue: mpsc::Receiver<callbacks::OnFrame>,
    sender: mpsc::Sender<callbacks::OnFrame>,
}
pub struct ApplicationInnerWrapper<T: ApplicationInner> {
    base: ApplicationBase,
    inner: T,
}
impl ApplicationBase {
    pub fn sender(&mut self) -> &mut mpsc::Sender<callbacks::OnFrame> {
        &mut self.sender
    }
    pub fn queue(&mut self) -> &mut mpsc::Receiver<callbacks::OnFrame> {
        &mut self.queue
    }
}
impl<T: ApplicationInner> HasBase for AApplication<T> {
    type Base = ApplicationBase;

    fn base(&self) -> &Self::Base {
        unsafe { &(&*self.inner.get()).base }
    }
    fn base_mut(&mut self) -> &mut Self::Base {
        unsafe { &mut (&mut *self.inner.get()).base }
    }
}
impl<T: ApplicationInner> controls::HasNativeId for AApplication<T> {
    #[inline]
    unsafe fn native_id(&self) -> usize {
        self.as_inner().native_id().into()
    }
}
impl<T: ApplicationInner> controls::Application for AApplication<T> {
    #[inline]
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        self.as_inner_mut().new_window(title, size, menu)
    }
    #[inline]
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        self.as_inner_mut().new_tray(title, menu)
    }
    #[inline]
    fn name(&self) -> Cow<'_, str> {
        self.as_inner().name()
    }
    #[inline]
    fn start(&mut self) {
        self.as_inner_mut().start()
    }
    #[inline]
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
        self.as_inner_mut().find_member_mut(arg)
    }
    #[inline]
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
        self.as_inner().find_member(arg)
    }
    #[inline]
    fn exit(mut self: Box<Self>, skip_on_close: bool) -> bool {
        let exited = self.as_inner_mut().exit(skip_on_close);
        if exited {
            runtime::deinit(&self.inner);
        }
        exited
    }
    #[inline]
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<callbacks::OnFrame> {
        let feeder = self.base_mut().sender().clone();
        self.as_inner_mut().on_frame_async_feeder(feeder.into())
    }
    #[inline]
    fn on_frame(&mut self, cb: callbacks::OnFrame) {
        let mut feeder = self.base_mut().sender().clone().into();
        self.as_inner_mut().on_frame(&mut feeder, cb)
    }
    #[inline]
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        self.as_inner().members()
    }
    #[inline]
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        self.as_inner_mut().members_mut()
    }
}
impl<T: ApplicationInner> controls::AsAny for AApplication<T> {
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
    type Inner = T;
    type Params = ();

    #[inline]
    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        let (tx, rx) = mpsc::channel();
        Application {
            inner: Rc::new(UnsafeCell::new(ApplicationInnerWrapper {
                base: ApplicationBase { sender: tx, queue: rx },
                inner: inner,
            })),
        }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        unsafe { &(&*self.inner.get()).inner }
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        unsafe { &mut (&mut *self.inner.get()).inner }
    }
    #[inline]
    fn into_inner(self) -> Self::Inner {
        panic!("Never unwrap an Application");
    }
}
impl<T: ApplicationInner> AApplication<T> {
    #[inline]
    pub fn get() -> types::ApplicationResult {
        let (inner, ready) = runtime::get::<T>();
        if let Some(inner) = inner {
            types::ApplicationResult::Existing(Box::new(Application { inner }))
        } else if ready {
            types::ApplicationResult::ErrorNonUiThread
        } else {
            let app = T::get();
            runtime::init(app.inner.clone());
            types::ApplicationResult::New(app)
        }
    }
}
impl<T: ApplicationInner> seal::Sealed for AApplication<T> {}