use super::*;

#[inline]
pub unsafe fn base_to_impl<T>(this: &sdk::MemberBase) -> &T
where
    T: controls::Member + Sized,
{
    &*(this as *const _ as *const T)
}
#[inline]
pub unsafe fn base_to_impl_mut<T>(this: &mut sdk::MemberBase) -> &mut T
where
    T: controls::Member + Sized,
{
    &mut *(this as *mut _ as *mut T)
}

#[inline]
pub(crate) unsafe fn base_to_member<T>(this: &sdk::MemberBase) -> &dyn controls::Member
where
    T: controls::Member + Sized,
{
    base_to_impl::<T>(this)
}
#[inline]
pub(crate) unsafe fn base_to_member_mut<T>(this: &mut sdk::MemberBase) -> &mut dyn controls::Member
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
pub fn find_by_mut<'a>(control: &'a mut dyn controls::Control, arg: types::FindBy<'a>) -> Option<&'a mut dyn controls::Control> {
    match arg {
        types::FindBy::Id(id) => {
            if control.as_member_mut().id() == id {
                return Some(control);
            }
        },
        types::FindBy::Tag(tag) => {
            if let Some(mytag) = control.as_member_mut().tag() {
                if tag == mytag {
                    return Some(control);
                }
            }
        }
    }
    if let Some(c) = control.is_container_mut() {
        let ret = c.find_control_mut(arg.clone());
        if ret.is_some() {
            return ret;
        }
    }
    None
}
pub fn find_by<'a>(control: &'a dyn controls::Control, arg: types::FindBy<'a>) -> Option<&'a dyn controls::Control> {
    match arg {
        types::FindBy::Id(id) => {
            if control.as_member().id() == id {
                return Some(control);
            }
        },
        types::FindBy::Tag(tag) => {
            if let Some(mytag) = control.as_member().tag() {
                if tag == mytag {
                    return Some(control);
                }
            }
        }
    }
    if let Some(c) = control.is_container() {
        let ret = c.find_control(arg.clone());
        if ret.is_some() {
            return ret;
        }
    }
    None
}