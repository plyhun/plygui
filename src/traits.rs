use super::{layout, callbacks, types, ids, development};

pub trait UiApplication {
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, has_menu: bool) -> Box<UiWindow>;
    fn name<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn start(&mut self);
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut UiMember>;
    fn find_member_by_id(&self, id: ids::Id) -> Option<&UiMember>;
}

pub trait UiMember {
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);

    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;

    fn as_base(&self) -> &types::UiMemberBase;
    fn as_base_mut(&mut self) -> &mut types::UiMemberBase;

    fn is_control(&self) -> Option<&UiControl>;
    fn is_control_mut(&mut self) -> Option<&mut UiControl>;

    unsafe fn native_id(&self) -> usize;
}

pub trait UiHasOrientation {
    fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, layout::Orientation);
}

pub trait UiHasLayout: UiMember {
    fn layout_width(&self) -> layout::Size;
    fn layout_height(&self) -> layout::Size;
    fn layout_gravity(&self) -> layout::Gravity;
    fn layout_alignment(&self) -> layout::Alignment;
    fn layout_padding(&self) -> layout::BoundarySize;
    fn layout_margin(&self) -> layout::BoundarySize;

    fn set_layout_width(&mut self, layout::Size);
    fn set_layout_height(&mut self, layout::Size);
    fn set_layout_gravity(&mut self, layout::Gravity);
    fn set_layout_alignment(&mut self, layout::Alignment);
    fn set_layout_padding(&mut self, layout::BoundarySizeArgs);
    fn set_layout_margin(&mut self, layout::BoundarySizeArgs);

    fn as_member(&self) -> &UiMember;
    fn as_member_mut(&mut self) -> &mut UiMember;
}

pub trait UiControl: UiHasLayout + development::UiDrawable {
    fn is_container_mut(&mut self) -> Option<&mut UiContainer>;
    fn is_container(&self) -> Option<&UiContainer>;

    fn on_added_to_container(&mut self, &UiContainer, x: i32, y: i32);
    fn on_removed_from_container(&mut self, &UiContainer);

    fn parent(&self) -> Option<&types::UiMemberBase>;
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase>;
    fn root(&self) -> Option<&types::UiMemberBase>;
    fn root_mut(&mut self) -> Option<&mut types::UiMemberBase>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, &super::markup::Markup, &mut super::markup::MarkupRegistry);

    fn as_has_layout(&self) -> &UiHasLayout;
    fn as_has_layout_mut(&mut self) -> &mut UiHasLayout;

    /*fn as_drawable(&self) -> &development::UiDrawable;
	fn as_drawable_mut(&mut self) -> &mut development::UiDrawable;	*/
}

pub trait UiContainer: UiMember {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut UiControl>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&UiControl>;

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

    fn as_member(&self) -> &UiMember;
    fn as_member_mut(&mut self) -> &mut UiMember;
}

pub trait UiSingleContainer: UiContainer {
    fn set_child(&mut self, Option<Box<UiControl>>) -> Option<Box<UiControl>>;
    fn child(&self) -> Option<&UiControl>;
    fn child_mut(&mut self) -> Option<&mut UiControl>;

    fn as_container(&self) -> &UiContainer;
    fn as_container_mut(&mut self) -> &mut UiContainer;
}

pub trait UiMultiContainer: UiContainer {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<UiControl>) -> Option<Box<UiControl>>;
    fn remove_child_from(&mut self, index: usize) -> Option<Box<UiControl>>;
    fn child_at(&self, index: usize) -> Option<&Box<UiControl>>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<UiControl>>;

    fn as_container(&self) -> &UiContainer;
    fn as_container_mut(&mut self) -> &mut UiContainer;

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

pub trait UiHasLabel {
    fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn set_label(&mut self, &str);
}

pub trait UiClickable {
    fn on_click(&mut self, Option<callbacks::Click>);
}

pub trait UiWindow: UiSingleContainer + UiHasLabel {
    fn as_single_container(&self) -> &UiSingleContainer;
    fn as_single_container_mut(&mut self) -> &mut UiSingleContainer;
}

pub trait UiButton: UiControl + UiClickable + UiHasLabel {
    fn as_control(&self) -> &UiControl;
    fn as_control_mut(&mut self) -> &mut UiControl;
    fn as_clickable(&self) -> &UiClickable;
    fn as_clickable_mut(&mut self) -> &mut UiClickable;
    fn as_has_label(&self) -> &UiHasLabel;
    fn as_has_label_mut(&mut self) -> &mut UiHasLabel;
}

pub trait UiLinearLayout: UiMultiContainer + UiControl + UiHasOrientation {
    fn orientation(&self) -> layout::Orientation;
    fn set_orientation(&mut self, layout::Orientation);

    fn as_control(&self) -> &UiControl;
    fn as_control_mut(&mut self) -> &mut UiControl;
    fn as_multi_container(&self) -> &UiMultiContainer;
    fn as_multi_container_mut(&mut self) -> &mut UiMultiContainer;
    fn as_has_orientation(&self) -> &UiHasOrientation;
    fn as_has_orientation_mut(&mut self) -> &mut UiHasOrientation;
}

pub trait UiRelativeLayout: UiMultiContainer + UiControl {}
