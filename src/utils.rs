use super::*;
use std::mem;

pub unsafe fn base_to_impl<'a, T>(this: &'a development::UiMemberCommon) -> &'a T where T: traits::UiMember + Sized {
	mem::transmute(this)
}
pub unsafe fn base_to_impl_mut<'a, T>(this: &'a mut development::UiMemberCommon) -> &'a mut T where T: traits::UiMember + Sized {
	mem::transmute(this)
}

/*pub fn common_to_impl<'a, T>(this: &'a types::UiMemberBase) -> &'a T where T: traits::UiMember + Sized {
	unsafe { base_to_impl(&this.0) }
}
pub fn common_to_impl_mut<'a, T>(this: &'a mut types::UiMemberBase) -> &'a mut T where T: traits::UiMember + Sized {
	unsafe { base_to_impl_mut(&mut this.0) }
}*/


/*
pub fn common_builtin_as_member(this: &UiMemberCommon) -> &UiMember {
	match this.member_id() {
		members::MEMBER_ID_WINDOW => common_to_impl<>(this)
	}
}*/