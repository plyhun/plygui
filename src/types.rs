use super::{development, ids, traits};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStartSize {
	Exact(u16, u16),
	Fullscreen,
}

#[repr(C)]
pub struct UiMemberBase(pub(crate) development::UiMemberCommon);

/*impl <'a> From<&'a UiMemberBase> for &'a development::UiMemberCommon {
	fn from(this: &'a UiMemberBase) -> &'a development::UiMemberCommon {
		&this.0
	}
}*/

impl AsRef<UiMemberBase> for development::UiMemberCommon {
	fn as_ref(&self) -> &UiMemberBase {
		unsafe { ::std::mem::transmute(self) }
	} 
}
impl AsMut<UiMemberBase> for development::UiMemberCommon {
	fn as_mut(&mut self) -> &mut UiMemberBase {
		unsafe { ::std::mem::transmute(self) }
	} 
}

impl UiMemberBase {
	pub fn id(&self) -> ids::Id {
		self.0.id
	}    
    pub fn member_id(&self) -> &'static str {
    	self.0.member_id()
    }
}

#[macro_export]
macro_rules! callback {
	($id: ident, $($typ:tt)+) => {
		pub struct $id(Box<$($typ)+>);
		impl <T> From<T> for $id where T: $($typ)+ + Sized {
			fn from(t: T) -> $id {
				$id(Box::new(t))
			}
		}
		
		/*impl AsRef<$($typ)+> for $id {
			fn as_ref(&self) -> &$($typ)+ {
				self.0.as_ref()
			}
		}*/
	}
}

callback!(ResizeCallback, FnMut(&mut traits::UiMember, u16, u16));
callback!(ClickCallback, FnMut(&mut traits::UiButton));

