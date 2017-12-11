pub mod layout;

use super::{types, ids};

pub struct UiMemberFunctions {
    pub fn_member_id: unsafe fn(&UiMemberBase) -> &'static str,
    pub fn_is_control: unsafe fn(&UiMemberBase) -> Option<&UiControlBase>,
    pub fn_is_control_mut: unsafe fn(&mut UiMemberBase) -> Option<&mut UiControlBase>,
    pub fn_size: unsafe fn(&UiMemberBase) -> (u16, u16),	
    /*pub fn_draw: unsafe fn(&mut UiMemberBase, coords: Option<(i32, i32)>),
    pub fn_measure: unsafe fn(&mut UiMemberBase, w: u16, h: u16) -> (u16, u16, bool),*/
}

#[repr(C)]
pub struct UiControlBase {
    pub member_base: UiMemberBase,
    pub layout: layout::LayoutBase,
}

#[repr(C)]
pub struct UiMemberBase {
    pub id: ids::Id,
    pub visibility: types::Visibility,

	functions: UiMemberFunctions,
}

impl UiMemberBase {
    pub fn with_params(visibility: types::Visibility, functions: UiMemberFunctions) -> UiMemberBase {
        UiMemberBase {
            id: ids::Id::next(),
            visibility: visibility,
            functions: functions,
        }
    }
    
    pub fn member_id(&self) -> &'static str {
    	unsafe { (self.functions.fn_member_id)(self) }
    }
    
    pub fn is_control(&self) -> Option<&UiControlBase> {
    	unsafe { (self.functions.fn_is_control)(self) }
    }
    pub fn is_control_mut(&mut self) -> Option<&mut UiControlBase> {
    	unsafe { (self.functions.fn_is_control_mut)(self) }
    }
    pub fn size(&self) -> (u16, u16) {
    	unsafe { (self.functions.fn_size)(self) }
    }
    /*pub fn draw(&mut self, coords: Option<(i32, i32)>) {
    	unsafe { (self.functions.fn_draw)(self, coords) }
    }
    pub fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool) {
    	unsafe { (self.functions.fn_measure)(self, w, h) }
    }*/
}

pub trait UiDrawable {
	fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
}
