use super::{development, ids};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

#[repr(C)]
pub struct UiMemberCommon(development::UiMemberBase);

impl UiMemberCommon {
	pub fn id(&self) -> ids::Id {
		self.0.id
	}    
    pub fn member_id(&self) -> &'static str {
    	unsafe { (self.0.fn_member_id)(&self.0) }
    }
}