use super::*;
use std::mem;

pub unsafe fn base_to_impl<T>(this: &development::UiMemberCommon) -> &T
    where T: traits::UiMember + Sized
{
    mem::transmute(this)
}
pub unsafe fn base_to_impl_mut<T>(this: &mut development::UiMemberCommon) -> &mut T
    where T: traits::UiMember + Sized
{
    mem::transmute(this)
}

pub fn common_to_impl<T>(this: &types::UiMemberBase) -> &T
    where T: traits::UiMember + Sized
{
    unsafe { base_to_impl(&this.0) }
}
pub fn common_to_impl_mut<T>(this: &mut types::UiMemberBase) -> &mut T
    where T: traits::UiMember + Sized
{
    unsafe { base_to_impl_mut(&mut this.0) }
}
