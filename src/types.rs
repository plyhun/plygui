use super::{development, ids};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowStartSize {
    Exact(u16, u16),
    Fullscreen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowMenu {
    None,
}

#[repr(C)]
pub struct UiMemberBase(pub(crate) development::UiMemberCommon);

impl AsRef<UiMemberBase> for development::UiMemberCommon {
    fn as_ref(&self) -> &UiMemberBase {
        unsafe { ::std::mem::transmute(self) }
    }
}
impl AsMut<UiMemberBase> for development::UiMemberCommon {
    fn as_mut(&mut self) -> &mut UiMemberBase {
        unsafe { ::std::mem::transmute(self) }
    }
}

impl UiMemberBase {
    pub fn id(&self) -> ids::Id {
        self.0.id
    }
    pub fn member_id(&self) -> &'static str {
        self.0.member_id()
    }
}
