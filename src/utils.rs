use super::*;

#[inline]
pub unsafe fn base_to_impl<T>(this: &development::MemberBase) -> &T
    where T: controls::Member + Sized
{
    &*(this as *const _ as *const T)
}
#[inline]
pub unsafe fn base_to_impl_mut<T>(this: &mut development::MemberBase) -> &mut T
    where T: controls::Member + Sized
{
    &mut *(this as *mut _ as *mut T)
}

#[inline]
pub fn member_control_base<T: development::ControlInner>(this: &development::Member<development::Control<T>>) -> &development::MemberControlBase {
    unsafe { &*(this as *const _ as *const development::MemberControlBase) }
}
#[inline]
pub fn member_control_base_mut<T: development::ControlInner>(this: &mut development::Member<development::Control<T>>) -> &mut development::MemberControlBase {
    unsafe { &mut *(this as *mut _ as *mut development::MemberControlBase) }
}

#[inline]
pub unsafe fn member_control_base_unchecked(this: &development::MemberBase) -> &development::MemberControlBase {
    &*(this as *const _ as *const development::MemberControlBase)
}
#[inline]
pub unsafe fn member_control_base_mut_unchecked(this: &mut development::MemberBase) -> &mut development::MemberControlBase {
    &mut *(this as *mut _ as *mut development::MemberControlBase)
}
