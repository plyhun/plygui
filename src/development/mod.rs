pub mod layout;

use super::{types, ids};

#[repr(C)]
pub struct UiControlBase {
	pub member_base: UiMemberBase,
	pub layout: layout::LayoutBase,
}

#[repr(C)]
pub struct UiMemberBase {
	pub id: ids::Id,
	pub visibility: types::Visibility,
    
    // Keep in sync with UiMember !!
    pub is_control: bool, 
    pub member_id: &'static str,
}

impl UiMemberBase {
	pub fn with_params(member_id: &'static str, is_control: bool, visibility: types::Visibility) -> UiMemberBase {
		UiMemberBase {
			id: ids::Id::next(),
			is_control: is_control,
			visibility: visibility,
			member_id: member_id,
		}
	}
}