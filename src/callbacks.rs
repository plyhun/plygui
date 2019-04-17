use super::{controls, types};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::{SendError, Sender};

static GLOBAL_COUNT: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CallbackId(usize);

impl CallbackId {
    pub fn next() -> CallbackId {
        CallbackId(atomic_next())
    }
}

fn atomic_next() -> usize {
    GLOBAL_COUNT.fetch_add(1, Ordering::SeqCst)
}

pub trait Callback {
    fn name(&self) -> &'static str;
    fn id(&self) -> CallbackId;
}

#[derive(Debug, Clone)]
pub struct AsyncFeeder<T: Callback> {
    sender: Sender<T>,
}
impl<T: Callback> AsyncFeeder<T> {
    pub fn feed(&mut self, data: T) -> Result<(), SendError<T>> {
        self.sender.send(data)
    }
}
impl<T: Callback> From<Sender<T>> for AsyncFeeder<T> {
    fn from(s: Sender<T>) -> Self {
        AsyncFeeder { sender: s }
    }
}
unsafe impl <T: Callback> Send for AsyncFeeder<T> {}
unsafe impl <T: Callback> Sync for AsyncFeeder<T> {}

macro_rules! callback {
	($id: ident, $($typ:tt)+) => {
		pub struct $id(CallbackId, Box<dyn $($typ)+>);

		impl Callback for $id {
			fn name(&self) -> &'static str {
				stringify!($id)
			}
			fn id(&self) -> CallbackId {
				self.0
			}
		}

		impl <T> From<T> for $id where T: $($typ)+ + Sized + 'static {
			fn from(t: T) -> $id {
				$id(CallbackId::next(), Box::new(t))
			}
		}
		impl AsRef<dyn $($typ)+> for $id {
			fn as_ref(&self) -> &(dyn $($typ)+  + 'static) {
				self.1.as_ref()
			}
		}
		impl AsMut<dyn $($typ)+> for $id {
			fn as_mut(&mut self) -> &mut (dyn $($typ)+ + 'static) {
				self.1.as_mut()
			}
		}
		impl From<$id> for (CallbackId, Box<dyn $($typ)+>) {
		    fn from(a: $id) -> Self {
		        (a.0, a.1)
		    }
		}
		impl From<(CallbackId, Box<dyn $($typ)+>)> for $id {
		    fn from(a: (CallbackId, Box<dyn $($typ)+>)) -> Self {
		        $id(a.0, a.1)
		    }
		}

		impl ::std::fmt::Debug for $id {
			fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
				write!(f, "{}({})", self.name(), self.id().0)
			}
		}
		impl ::std::cmp::PartialEq for $id {
			fn eq(&self, other: &$id) -> bool {
				self.id().eq(&other.id())
			}
		}
	}
}

callback!(OnSize, FnMut(&mut dyn controls::HasSize, u16, u16));
callback!(OnVisibility, FnMut(&mut dyn controls::HasVisibility, types::Visibility));
callback!(OnClick, FnMut(&mut dyn controls::Clickable));
callback!(OnFrame, FnMut(&mut dyn controls::Window) -> bool);
callback!(Action, FnMut(&mut dyn controls::Member) -> bool);
