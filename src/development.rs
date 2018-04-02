use super::{types, ids, layout};

pub struct UiMemberFunctions {
    pub fn_member_id: unsafe fn(&UiMemberCommon) -> &'static str,
    pub fn_is_control: unsafe fn(&UiMemberCommon) -> Option<&UiControlCommon>,
    pub fn_is_control_mut: unsafe fn(&mut UiMemberCommon) -> Option<&mut UiControlCommon>,
    pub fn_size: unsafe fn(&UiMemberCommon) -> (u16, u16),
}

#[repr(C)]
pub struct UiControlCommon {
    pub member_base: UiMemberCommon,
    pub layout: layout::Attributes,
}

#[repr(C)]
pub struct UiMemberCommon {
    pub id: ids::Id,
    pub visibility: types::Visibility,

    functions: UiMemberFunctions,
}

impl UiMemberCommon {
    pub fn with_params(visibility: types::Visibility, functions: UiMemberFunctions) -> UiMemberCommon {
        UiMemberCommon {
            id: ids::Id::next(),
            visibility,
            functions,
        }
    }

    pub fn member_id(&self) -> &'static str {
        unsafe { (self.functions.fn_member_id)(self) }
    }

    pub fn is_control(&self) -> Option<&UiControlCommon> {
        unsafe { (self.functions.fn_is_control)(self) }
    }
    pub fn is_control_mut(&mut self) -> Option<&mut UiControlCommon> {
        unsafe { (self.functions.fn_is_control_mut)(self) }
    }
    pub fn size(&self) -> (u16, u16) {
        unsafe { (self.functions.fn_size)(self) }
    }
}

impl AsRef<UiMemberCommon> for types::UiMemberBase {
    fn as_ref(&self) -> &UiMemberCommon {
        unsafe { ::std::mem::transmute(self) }
    }
}
impl AsMut<UiMemberCommon> for types::UiMemberBase {
    fn as_mut(&mut self) -> &mut UiMemberCommon {
        unsafe { ::std::mem::transmute(self) }
    }
}

pub trait UiDrawable {
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
}
