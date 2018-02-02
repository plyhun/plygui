use super::traits;

pub trait Callback {
    fn id(&self) -> &'static str;
}

#[macro_export]
macro_rules! callback {
	($id: ident, $($typ:tt)+) => {
		pub struct $id(Box<$($typ)+>);
		
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
		impl AsRef<$($typ)+> for $id {
			fn as_ref(&self) -> &($($typ)+ + 'static) {
				self.0.as_ref()
			}
		}
		impl AsMut<$($typ)+> for $id {
			fn as_mut(&mut self) -> &mut ($($typ)+ + 'static) {
				self.0.as_mut()
			}
		}
	}
}

callback!(Resize, FnMut(&mut traits::UiMember, u16, u16));
callback!(Click, FnMut(&mut traits::UiButton));
