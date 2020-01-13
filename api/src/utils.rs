use super::*;

#[inline]
pub unsafe fn base_to_impl<T>(this: &development::MemberBase) -> &T
where
    T: controls::Member + Sized,
{
    &*(this as *const _ as *const T)
}
#[inline]
pub unsafe fn base_to_impl_mut<T>(this: &mut development::MemberBase) -> &mut T
where
    T: controls::Member + Sized,
{
    &mut *(this as *mut _ as *mut T)
}

#[inline]
pub(crate) unsafe fn base_to_member<T>(this: &development::MemberBase) -> &dyn controls::Member
where
    T: controls::Member + Sized,
{
    base_to_impl::<T>(this)
}
#[inline]
pub(crate) unsafe fn base_to_member_mut<T>(this: &mut development::MemberBase) -> &mut dyn controls::Member
where
    T: controls::Member + Sized,
{
    base_to_impl_mut::<T>(this)
}

#[inline]
pub fn coord_to_size(a: i32) -> u16 {
    ::std::cmp::max(0, a) as u16
}

/*#[inline]
pub unsafe fn member_to_base(this: &controls::Member) -> &MemberBase
where
    T: controls::Member + Sized,
{
    &*(this as *const _ as *const T)
}
#[inline]
pub unsafe fn member_to_base_mut<T>(this: &mut controls::Member) -> &mut MemberBase
where
    T: controls::Member + Sized,
{
    &mut *(this as *mut _ as *mut T)
}
*/
