use super::*;

pub struct MemberMock {
	
}

impl development::MemberInner for MemberMock {
	type Id = MockId;
	
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);

    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;

    fn id(&self) -> ids::Id;
    unsafe fn native_id(&self) -> Self::Id;
}

fn test_sanity() {
	
}