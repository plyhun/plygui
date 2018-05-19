use super::{layout, callbacks, types, ids, development};

use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    fn into_any(self: Box<Self>) -> Box<Any>; 
}

pub trait UiMember: AsAny + development::seal::Sealed {
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);

    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;

    fn id(&self) -> ids::Id;
    unsafe fn native_id(&self) -> usize;
    
    fn is_control(&self) -> Option<&UiControl>;
    fn is_control_mut(&mut self) -> Option<&mut UiControl>;
    fn is_container(&self) -> Option<&UiContainer>;
    fn is_container_mut(&mut self) -> Option<&mut UiContainer>;
    
    fn as_member(&self) -> &UiMember;
    fn as_member_mut(&mut self) -> &mut UiMember;
    fn into_member(self: Box<Self>) -> Box<UiMember>;
}

pub trait UiHasOrientation: AsAny + development::seal::Sealed {
    fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, layout::Orientation);
    
    fn as_has_orientation(&self) -> &UiHasOrientation;
    fn as_has_orientation_mut(&mut self) -> &mut UiHasOrientation;
    fn into_has_orientation(self: Box<Self>) -> Box<UiHasOrientation>;
}

pub trait UiHasLayout: UiMember {
    fn layout_width(&self) -> layout::Size;
    fn layout_height(&self) -> layout::Size;
    fn layout_alignment(&self) -> layout::Alignment;
    fn layout_padding(&self) -> layout::BoundarySize;
    fn layout_margin(&self) -> layout::BoundarySize;

    fn set_layout_width(&mut self, layout::Size);
    fn set_layout_height(&mut self, layout::Size);
    fn set_layout_alignment(&mut self, layout::Alignment);
    fn set_layout_padding(&mut self, layout::BoundarySizeArgs);
    fn set_layout_margin(&mut self, layout::BoundarySizeArgs);
    
    fn as_has_layout(&self) -> &UiHasLayout;
    fn as_has_layout_mut(&mut self) -> &mut UiHasLayout;
    fn into_has_layout(self: Box<Self>) -> Box<UiHasLayout>;
}

pub trait UiControl: UiHasLayout + development::OuterDrawable {
    fn on_added_to_container(&mut self, &UiContainer, x: i32, y: i32);
    fn on_removed_from_container(&mut self, &UiContainer);

    fn parent(&self) -> Option<&UiMember>;
    fn parent_mut(&mut self) -> Option<&mut UiMember>;
    fn root(&self) -> Option<&UiMember>;
    fn root_mut(&mut self) -> Option<&mut UiMember>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, &super::markup::Markup, &mut super::markup::MarkupRegistry);
    
    fn as_control(&self) -> &UiControl;
    fn as_control_mut(&mut self) -> &mut UiControl;
    fn into_control(self: Box<Self>) -> Box<UiControl>;
}

pub trait UiContainer: UiMember {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut UiControl>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&UiControl>;
    
    fn gravity(&self) -> (layout::Gravity, layout::Gravity);
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity);

    fn draw_area_size(&self) -> (u16, u16) {
    	let mut size = self.size();
    	if let Some(c) = self.is_control() {
    		use std::cmp::max;
    		
    		let hl = c.as_has_layout();
    		let (lp,tp,rp,bp) = hl.layout_padding().into();
    		let (lm,tm,rm,bm) = hl.layout_margin().into();
    		size.0 = max(0, size.0 as i32 - (lp + rp + lm + rm)) as u16;
    		size.1 = max(0, size.1 as i32 - (tp + bp + tm + bm)) as u16;
    	}
    	size
    }

    fn is_multi_mut(&mut self) -> Option<&mut UiMultiContainer> {
        None
    }
    fn is_multi(&self) -> Option<&UiMultiContainer> {
        None
    }
    fn is_single_mut(&mut self) -> Option<&mut UiSingleContainer> {
        None
    }
    fn is_single(&self) -> Option<&UiSingleContainer> {
        None
    }

    fn as_container(&self) -> &UiContainer;
    fn as_container_mut(&mut self) -> &mut UiContainer;
    fn into_container(self: Box<Self>) -> Box<UiContainer>;
}

pub trait UiSingleContainer: UiContainer {
    fn set_child(&mut self, Option<Box<UiControl>>) -> Option<Box<UiControl>>;
    fn child(&self) -> Option<&UiControl>;
    fn child_mut(&mut self) -> Option<&mut UiControl>;
    
    fn as_single_container(&self) -> &UiSingleContainer;
    fn as_single_container_mut(&mut self) -> &mut UiSingleContainer;
    fn into_single_container(self: Box<Self>) -> Box<UiSingleContainer>;
}

pub trait UiMultiContainer: UiContainer {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<UiControl>) -> Option<Box<UiControl>>;
    fn remove_child_from(&mut self, index: usize) -> Option<Box<UiControl>>;
    fn child_at(&self, index: usize) -> Option<&UiControl>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut UiControl>;

    fn is_empty(&self) -> bool {
        self.len() < 1
    }
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
    
    fn as_multi_container(&self) -> &UiMultiContainer;
    fn as_multi_container_mut(&mut self) -> &mut UiMultiContainer;
    fn into_multi_container(self: Box<Self>) -> Box<UiMultiContainer>;
}

pub trait UiHasLabel: AsAny + development::seal::Sealed {
    fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn set_label(&mut self, &str);
    
    fn as_has_label(&self) -> &UiHasLabel;
    fn as_has_label_mut(&mut self) -> &mut UiHasLabel;
    fn into_has_label(self: Box<Self>) -> Box<UiHasLabel>;
}

pub trait UiClickable: AsAny + development::seal::Sealed {
    fn on_click(&mut self, Option<callbacks::Click>);
    
    fn as_clickable(&self) -> &UiClickable;
    fn as_clickable_mut(&mut self) -> &mut UiClickable;
    fn into_clickable(self: Box<Self>) -> Box<UiClickable>;
}

pub trait UiApplication: AsAny + development::seal::Sealed {
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<UiWindow>;
    fn name<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn start(&mut self);
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut UiMember>;
    fn find_member_by_id(&self, id: ids::Id) -> Option<&UiMember>;
}
impl development::Final for UiApplication {}

pub trait UiWindow: UiSingleContainer + UiHasLabel {}
impl development::Final for UiWindow {}

pub trait UiButton: UiControl + UiClickable + UiHasLabel {}
impl development::Final for UiButton {}

pub trait UiLinearLayout: UiMultiContainer + UiControl + UiHasOrientation {}
impl development::Final for UiLinearLayout {}

pub trait UiFrame: UiSingleContainer + UiControl + UiHasLabel {}
impl development::Final for UiFrame {}
