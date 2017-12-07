pub mod layout;

use super::{types, ids};

pub type FnIsControl = unsafe fn(&UiMemberBase) -> Option<&UiControlBase>;
pub type FnIsControlMut = unsafe fn(&mut UiMemberBase) -> Option<&mut UiControlBase>;
pub type FnMemberId = unsafe fn(&UiMemberBase) -> &'static str;
pub type FnSize = unsafe fn(&UiMemberBase) -> (u16, u16);

#[repr(C)]
pub struct UiControlBase {
    pub member_base: UiMemberBase,
    pub layout: layout::LayoutBase,
}

#[repr(C)]
pub struct UiMemberBase {
    pub id: ids::Id,
    pub visibility: types::Visibility,

    pub(crate) fn_member_id: FnMemberId,
    pub(crate) fn_is_control: FnIsControl,
    pub(crate) fn_is_control_mut: FnIsControlMut,
    pub(crate) fn_size: FnSize,
}

impl UiMemberBase {
    pub fn with_params(visibility: types::Visibility, fn_member_id: FnMemberId, fn_is_control: FnIsControl, fn_is_control_mut: FnIsControlMut, fn_size: FnSize) -> UiMemberBase {
        UiMemberBase {
            id: ids::Id::next(),
            visibility: visibility,
            fn_member_id: fn_member_id,
            fn_is_control: fn_is_control,
            fn_is_control_mut: fn_is_control_mut,
            fn_size: fn_size,
        }
    }
}
