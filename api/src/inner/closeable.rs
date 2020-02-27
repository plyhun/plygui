use crate::callbacks::*;

use super::auto::{AsAny, HasInner, Abstract};
use super::member::{AMember, Member, MemberInner, MemberBase};
use super::application::Application;

define_abstract! {
    Closeable : Member {
        extends: {
            'static + AsAny
        },
        outer: {
            fn on_close(& mut self, callback : Option < OnClose >) ; 
            fn application(&self) -> &dyn Application;
            fn application_mut(&mut self) -> &mut dyn Application;
        },
        inner: {
            fn close (& mut self, skip_callbacks : bool) -> bool ; 
            fn on_close(& mut self, callback : Option < OnClose >) ;
            fn application<'a>(&'a self, base: &'a MemberBase) -> &'a dyn Application;
            fn application_mut<'a>(&'a mut self, base: &'a mut MemberBase) -> &'a mut dyn Application;
        }
        base: {
            application: usize,
        }
    }
}

pub struct OnClose(CallbackId, Box < dyn FnMut (& mut dyn Closeable,) -> bool >) ; 

impl Callback for OnClose {
    fn name (& self) -> & 'static str { stringify ! (OnClose) } 
    fn id (& self) -> CallbackId { self . 0 }
} 
impl < T > From < T > for OnClose where T : FnMut (& mut dyn Closeable) -> bool + Sized + 'static {
    fn from (t : T) -> OnClose { OnClose (CallbackId :: next (), Box :: new (t)) }
} 
impl AsRef < dyn FnMut (& mut dyn Closeable,) -> bool > for OnClose {
    fn as_ref (& self) -> &(dyn FnMut (& mut dyn Closeable) -> bool + 'static) { self . 1 . as_ref () }
} 
impl AsMut < dyn FnMut (& mut dyn Closeable,) -> bool > for OnClose {
    fn as_mut (& mut self) -> & mut (dyn FnMut (& mut dyn Closeable) -> bool + 'static) { self . 1 . as_mut () }
} 
impl From < OnClose > for (CallbackId, Box < dyn FnMut (& mut dyn Closeable) -> bool >) { 
    fn from (a : OnClose) -> Self { (a . 0, a . 1) } 
} 
impl From <(CallbackId, Box < dyn FnMut (& mut dyn Closeable) -> bool >) > for OnClose {
    fn from(a : (CallbackId, Box < dyn FnMut (& mut dyn Closeable) -> bool >)) -> Self { OnClose (a . 0, a . 1) }
} 
impl :: std :: fmt :: Debug for OnClose {
    fn fmt (& self, f : & mut :: std :: fmt :: Formatter) -> :: std :: fmt :: Result { write ! (f, "{}({})", self . name (), self . id ()) }
} 
impl :: std :: cmp :: PartialEq for OnClose {
    fn eq (& self, other : & OnClose) -> bool { self . id () . eq (& other . id ()) }
} 

impl<II: CloseableInner, T: HasInner<I = II> + Abstract + 'static> CloseableInner for T {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner_mut().close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<OnClose>) {
        self.inner_mut().on_close(callback)
    }
    fn application<'a>(&'a self, base: &'a MemberBase) -> &'a dyn Application {
        self.inner().application(base)
    }
    fn application_mut<'a>(&'a mut self, base: &'a mut MemberBase) -> &'a mut dyn Application {
        self.inner_mut().application_mut(base)
    }
}

impl<T: CloseableInner> Closeable for AMember<T> {
    fn on_close(&mut self, callback: Option<OnClose>) {
        self.inner.on_close(callback)
    }
    fn as_closeable(&self) -> &dyn Closeable {
        self
    }
    fn as_closeable_mut(&mut self) -> &mut dyn Closeable {
        self
    }
    fn into_closeable(self: Box<Self>) -> Box<dyn Closeable> {
        self
    }
    fn application(&self) -> &dyn Application {
        self.inner.application(&self.base)
    }
    fn application_mut(&mut self) -> &mut dyn Application {
        self.inner.application_mut(&mut self.base)
    }
}
impl<T: CloseableInner> ACloseable<T> {
    pub fn with_inner(inner: T, application: usize) -> Self {
        Self {
            base: CloseableBase { application },
            inner,
        }
    }
    pub fn application_impl<A: Application>(&self) -> &A {
        unsafe { ::std::mem::transmute(self.base.application) }
    }
    pub fn application_impl_mut<A: Application>(&mut self) -> &mut A {
        unsafe { ::std::mem::transmute(self.base.application) }
    }
}
