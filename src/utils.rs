use super::*;
use std::mem;

#[inline]
pub unsafe fn base_to_impl<T>(this: &development::MemberBase) -> &T
    where T: controls::Member + Sized
{
    mem::transmute(this)
}
#[inline]
pub unsafe fn base_to_impl_mut<T>(this: &mut development::MemberBase) -> &mut T
    where T: controls::Member + Sized
{
    mem::transmute(this)
}

#[inline]
pub fn member_control_base<T: development::ControlInner>(this: &development::Member<development::Control<T>>) -> &development::MemberControlBase {
	unsafe { mem::transmute(this) }
}
#[inline]
pub fn member_control_base_mut<T: development::ControlInner>(this: &mut development::Member<development::Control<T>>) -> &mut development::MemberControlBase {
	unsafe { mem::transmute(this) }
}

#[inline]
pub unsafe fn member_control_base_unchecked(this: &development::MemberBase) -> &development::MemberControlBase {
	mem::transmute(this)
}
#[inline]
pub unsafe fn member_control_base_mut_unchecked(this: &mut development::MemberBase) -> &mut development::MemberControlBase {
	mem::transmute(this)
}
