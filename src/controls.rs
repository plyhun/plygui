use super::{layout, callbacks, types, ids, development};

use std::any::Any;
#[cfg(feature = "type_check")]
use std::any::TypeId;

pub trait AsAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn into_any(self: Box<Self>) -> Box<dyn Any>;
}

pub trait Member: AsAny + development::seal::Sealed {
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);

    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;

    fn id(&self) -> ids::Id;
    unsafe fn native_id(&self) -> usize;
    #[cfg(feature = "type_check")]
    unsafe fn type_id(&self) -> TypeId;

    fn is_control(&self) -> Option<&dyn Control>;
    fn is_control_mut(&mut self) -> Option<&mut dyn Control>;
    fn is_container(&self) -> Option<&dyn Container>;
    fn is_container_mut(&mut self) -> Option<&mut dyn Container>;

    fn as_member(&self) -> &dyn Member;
    fn as_member_mut(&mut self) -> &mut dyn Member;
    fn into_member(self: Box<Self>) -> Box<dyn Member>;
}

pub trait HasOrientation: AsAny + development::seal::Sealed {
    fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, layout::Orientation);

    fn as_has_orientation(&self) -> &dyn HasOrientation;
    fn as_has_orientation_mut(&mut self) -> &mut dyn HasOrientation;
    fn into_has_orientation(self: Box<Self>) -> Box<dyn HasOrientation>;
}

pub trait HasLayout: Member {
    fn layout_width(&self) -> layout::Size;
    fn layout_height(&self) -> layout::Size;
    
    fn layout_margin(&self) -> layout::BoundarySize;

    fn set_layout_width(&mut self, layout::Size);
    fn set_layout_height(&mut self, layout::Size);
    
    fn as_has_layout(&self) -> &dyn HasLayout;
    fn as_has_layout_mut(&mut self) -> &mut dyn HasLayout;
    fn into_has_layout(self: Box<Self>) -> Box<dyn HasLayout>;
}

pub trait Control: HasLayout + development::OuterDrawable {
    fn on_added_to_container(&mut self, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16);
    fn on_removed_from_container(&mut self, parent: &dyn Container);

    fn parent(&self) -> Option<&dyn Member>;
    fn parent_mut(&mut self) -> Option<&mut dyn Member>;
    fn root(&self) -> Option<&dyn Member>;
    fn root_mut(&mut self) -> Option<&mut dyn Member>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &dyn super::markup::Markup, registry: &mut dyn super::markup::MarkupRegistry);

    fn as_control(&self) -> &dyn Control;
    fn as_control_mut(&mut self) -> &mut dyn Control;
    fn into_control(self: Box<Self>) -> Box<dyn Control>;
}

pub trait Container: Member {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn Control>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&dyn Control>;

    fn is_multi_mut(&mut self) -> Option<&mut dyn MultiContainer> {
        None
    }
    fn is_multi(&self) -> Option<&dyn MultiContainer> {
        None
    }
    fn is_single_mut(&mut self) -> Option<&mut dyn SingleContainer> {
        None
    }
    fn is_single(&self) -> Option<&dyn SingleContainer> {
        None
    }

    fn as_container(&self) -> &dyn Container;
    fn as_container_mut(&mut self) -> &mut dyn Container;
    fn into_container(self: Box<Self>) -> Box<dyn Container>;
}

pub trait SingleContainer: Container {
    fn set_child(&mut self, Option<Box<dyn Control>>) -> Option<Box<dyn Control>>;
    fn child(&self) -> Option<&dyn Control>;
    fn child_mut(&mut self) -> Option<&mut dyn Control>;

    fn as_single_container(&self) -> &dyn SingleContainer;
    fn as_single_container_mut(&mut self) -> &mut dyn SingleContainer;
    fn into_single_container(self: Box<Self>) -> Box<dyn SingleContainer>;
}

pub trait MultiContainer: Container {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<dyn Control>) -> Option<Box<dyn Control>>;
    fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn Control>>;
    fn child_at(&self, index: usize) -> Option<&dyn Control>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn Control>;

    fn is_empty(&self) -> bool {
        self.len() < 1
    }
    fn clear(&mut self) {
        let len = self.len();
        for index in (0..len).rev() {
            self.remove_child_from(index);
        }
    }
    fn push_child(&mut self, child: Box<dyn Control>) {
        let len = self.len();
        self.set_child_to(len, child);
    }
    fn pop_child(&mut self) -> Option<Box<dyn Control>> {
        let len = self.len();
        if len > 0 {
            self.remove_child_from(len - 1)
        } else {
            None
        }
    }

    fn as_multi_container(&self) -> &dyn MultiContainer;
    fn as_multi_container_mut(&mut self) -> &mut dyn MultiContainer;
    fn into_multi_container(self: Box<Self>) -> Box<dyn MultiContainer>;
}

pub trait HasLabel: AsAny + development::seal::Sealed {
    fn label(&self) -> ::std::borrow::Cow<str>;
    fn set_label(&mut self, &str);

    fn as_has_label(&self) -> &dyn HasLabel;
    fn as_has_label_mut(&mut self) -> &mut dyn HasLabel;
    fn into_has_label(self: Box<Self>) -> Box<dyn HasLabel>;
}

pub trait Clickable: AsAny + development::seal::Sealed {
    fn on_click(&mut self, Option<callbacks::Click>);

    fn as_clickable(&self) -> &dyn Clickable;
    fn as_clickable_mut(&mut self) -> &mut dyn Clickable;
    fn into_clickable(self: Box<Self>) -> Box<dyn Clickable>;
}

pub trait Application: AsAny + development::seal::Sealed {
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<dyn Window>;
    fn name(&self) -> ::std::borrow::Cow<str>;
    fn start(&mut self);
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut dyn Member>;
    fn find_member_by_id(&self, id: ids::Id) -> Option<&dyn Member>;
}
//impl <T: Application> development::Final for T {}

pub trait Window: SingleContainer + HasLabel {}
//impl <T: Window> development::Final for T {}

pub trait Button: Control + Clickable + HasLabel {}
//impl <T: Button> development::Final for T {}

pub trait LinearLayout: MultiContainer + Control + HasOrientation {}
//impl <T: LinearLayout> development::Final for T {}

pub trait Frame: SingleContainer + Control + HasLabel {}
//impl <T: Frame> development::Final for T {}

pub trait Splitted: MultiContainer + Control + HasOrientation {
    fn first(&self) -> &dyn Control;
    fn second(&self) -> &dyn Control;
    fn first_mut(&mut self) -> &mut dyn Control;
    fn second_mut(&mut self) -> &mut dyn Control;

    fn set_splitter(&mut self, pos: f32);
    fn splitter(&self) -> f32;
}
//impl <T: Splitted> development::Final for T {}
