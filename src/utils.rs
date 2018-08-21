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
