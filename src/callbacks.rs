use super::controls;

pub trait Callback {
    fn id(&self) -> &'static str;
}

#[macro_export]
macro_rules! callback {
	($id: ident, $($typ:tt)+) => {
		pub struct $id(Box<dyn $($typ)+>);
		
		impl Callback for $id {
			fn id(&self) -> &'static str {
				stringify!($id)
			}
		}
		
		impl <T> From<T> for $id where T: $($typ)+ + Sized + 'static {
			fn from(t: T) -> $id {
				$id(Box::new(t))
			}
		}
		impl AsRef<dyn $($typ)+> for $id {
			fn as_ref(&self) -> &(dyn $($typ)+  + 'static) {
				self.0.as_ref()
			}
		}
		impl AsMut<dyn $($typ)+> for $id {
			fn as_mut(&mut self) -> &mut (dyn $($typ)+ + 'static) {
				self.0.as_mut()
			}
		}
	}
}

callback!(Resize, FnMut(&mut dyn controls::Member, u16, u16));
callback!(Click, FnMut(&mut dyn controls::Button));
