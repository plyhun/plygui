use super::{layout, types, ids};

pub trait UiApplication {
    fn new_window(&mut self, title: &str, width: u16, height: u16, has_menu: bool) -> Box<UiWindow>;
    fn name(&self) -> &str;
    fn start(&mut self);
}

pub trait UiMember {
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<Box<FnMut(&mut UiMember, u16, u16)>>);

    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;
	fn id(&self) -> ids::Id;
    
    unsafe fn native_id(&self) -> usize;
    
    fn member_id(&self) -> &'static str;
    fn is_control(&self) -> Option<&UiControl>;
    fn is_control_mut(&mut self) -> Option<&mut UiControl>;
}

pub trait UiLayedOut: UiMember {
	fn layout_width(&self) -> layout::Size;
	fn layout_height(&self) -> layout::Size;
	fn layout_gravity(&self) -> layout::Gravity;
	fn layout_orientation(&self) -> layout::Orientation;
	fn layout_alignment(&self) -> layout::Alignment;
	
	fn set_layout_width(&mut self, layout::Size);
	fn set_layout_height(&mut self, layout::Size);
	fn set_layout_gravity(&mut self, layout::Gravity);
	fn set_layout_orientation(&mut self, layout::Orientation);
	fn set_layout_alignment(&mut self, layout::Alignment);    
}

pub trait UiControl: UiLayedOut {
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);

    fn is_container_mut(&mut self) -> Option<&mut UiContainer>;
    fn is_container(&self) -> Option<&UiContainer>;

	fn on_added_to_container(&mut self, &UiContainer, x: u16, y: u16);
    fn on_removed_from_container(&mut self, &UiContainer);
    
    fn parent(&self) -> Option<&types::UiMemberCommon>;
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberCommon>;
    fn root(&self) -> Option<&types::UiMemberCommon>;
    fn root_mut(&mut self) -> Option<&mut types::UiMemberCommon>;
}

pub trait UiContainer: UiMember {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut UiControl>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&UiControl>;

    fn is_multi_mut(&mut self) -> Option<&mut UiMultiContainer> { None }
    fn is_multi(&self) -> Option<&UiMultiContainer> { None }
    fn is_single_mut(&mut self) -> Option<&mut UiSingleContainer> { None }
    fn is_single(&self) -> Option<&UiSingleContainer> { None }
}

pub trait UiSingleContainer: UiContainer {
	fn set_child(&mut self, Option<Box<UiControl>>) -> Option<Box<UiControl>>;
    fn child(&self) -> Option<&UiControl>;
    fn child_mut(&mut self) -> Option<&mut UiControl>;
}

pub trait UiMultiContainer: UiContainer {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<UiControl>) -> Option<Box<UiControl>>;
    fn remove_child_from(&mut self, index: usize) -> Option<Box<UiControl>>;
    fn child_at(&self, index: usize) -> Option<&Box<UiControl>>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<UiControl>>;
    
    fn clear(&mut self) {
        let len = self.len();
        for index in (0..len).rev() {
            self.remove_child_from(index);
        }
    }
    fn push_child(&mut self, child: Box<UiControl>) {
        let len = self.len();
        self.set_child_to(len, child);
    }
    fn pop_child(&mut self) -> Option<Box<UiControl>> {
        let len = self.len();
        if len > 0 {
        	self.remove_child_from(len - 1)
        } else {
        	None
        }
    }
}

pub trait UiWindow: UiSingleContainer {}

pub trait UiButton: UiControl {
    //fn new(label: &str) -> Box<Self>;
    fn label(&self) -> &str;
    fn on_left_click(&mut self, Option<Box<FnMut(&mut UiButton)>>);
}

pub trait UiLinearLayout: UiMultiContainer + UiControl {
    fn orientation(&self) -> layout::Orientation;
    fn set_orientation(&mut self, layout::Orientation);
}

pub trait UiRelativeLayout: UiMultiContainer + UiControl {}