use super::{layout, callbacks, types, ids, development};

use std::any::Any;

pub trait AsAny {
    fn as_any(&self) -> &Any;
    fn as_any_mut(&mut self) -> &mut Any;
    fn into_any(self: Box<Self>) -> Box<Any>;
}

pub trait Member: AsAny + development::seal::Sealed {
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);

    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;

    fn id(&self) -> ids::Id;
    unsafe fn native_id(&self) -> usize;

    fn is_control(&self) -> Option<&Control>;
    fn is_control_mut(&mut self) -> Option<&mut Control>;
    fn is_container(&self) -> Option<&Container>;
    fn is_container_mut(&mut self) -> Option<&mut Container>;

    fn as_member(&self) -> &Member;
    fn as_member_mut(&mut self) -> &mut Member;
    fn into_member(self: Box<Self>) -> Box<Member>;
}

pub trait HasOrientation: AsAny + development::seal::Sealed {
    fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, layout::Orientation);

    fn as_has_orientation(&self) -> &HasOrientation;
    fn as_has_orientation_mut(&mut self) -> &mut HasOrientation;
    fn into_has_orientation(self: Box<Self>) -> Box<HasOrientation>;
}

pub trait HasLayout: Member {
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

    fn as_has_layout(&self) -> &HasLayout;
    fn as_has_layout_mut(&mut self) -> &mut HasLayout;
    fn into_has_layout(self: Box<Self>) -> Box<HasLayout>;
}

pub trait Control: HasLayout + development::OuterDrawable {
    fn on_added_to_container(&mut self, &Container, x: i32, y: i32);
    fn on_removed_from_container(&mut self, &Container);

    fn parent(&self) -> Option<&Member>;
    fn parent_mut(&mut self) -> Option<&mut Member>;
    fn root(&self) -> Option<&Member>;
    fn root_mut(&mut self) -> Option<&mut Member>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry);

    fn as_control(&self) -> &Control;
    fn as_control_mut(&mut self) -> &mut Control;
    fn into_control(self: Box<Self>) -> Box<Control>;
}

pub trait Container: Member {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut Control>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&Control>;

    fn gravity(&self) -> (layout::Gravity, layout::Gravity);
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity);

    fn draw_area_size(&self) -> (u16, u16) {
        let mut size = self.size();

        if let Some(c) = self.is_control() {
            use std::cmp::max;

            let hl = c.as_has_layout();
            let (lp, tp, rp, bp) = hl.layout_padding().into();
            let (lm, tm, rm, bm) = hl.layout_margin().into();
            size.0 = max(0, size.0 as i32 - (lp + rp + lm + rm)) as u16;
            size.1 = max(0, size.1 as i32 - (tp + bp + tm + bm)) as u16;
        }
        size
    }

    fn is_multi_mut(&mut self) -> Option<&mut MultiContainer> {
        None
    }
    fn is_multi(&self) -> Option<&MultiContainer> {
        None
    }
    fn is_single_mut(&mut self) -> Option<&mut SingleContainer> {
        None
    }
    fn is_single(&self) -> Option<&SingleContainer> {
        None
    }

    fn as_container(&self) -> &Container;
    fn as_container_mut(&mut self) -> &mut Container;
    fn into_container(self: Box<Self>) -> Box<Container>;
}

pub trait SingleContainer: Container {
    fn set_child(&mut self, Option<Box<Control>>) -> Option<Box<Control>>;
    fn child(&self) -> Option<&Control>;
    fn child_mut(&mut self) -> Option<&mut Control>;

    fn as_single_container(&self) -> &SingleContainer;
    fn as_single_container_mut(&mut self) -> &mut SingleContainer;
    fn into_single_container(self: Box<Self>) -> Box<SingleContainer>;
}

pub trait MultiContainer: Container {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<Control>) -> Option<Box<Control>>;
    fn remove_child_from(&mut self, index: usize) -> Option<Box<Control>>;
    fn child_at(&self, index: usize) -> Option<&Control>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Control>;

    fn is_empty(&self) -> bool {
        self.len() < 1
    }
    fn clear(&mut self) {
        let len = self.len();
        for index in (0..len).rev() {
            self.remove_child_from(index);
        }
    }
    fn push_child(&mut self, child: Box<Control>) {
        let len = self.len();
        self.set_child_to(len, child);
    }
    fn pop_child(&mut self) -> Option<Box<Control>> {
        let len = self.len();
        if len > 0 {
            self.remove_child_from(len - 1)
        } else {
            None
        }
    }

    fn as_multi_container(&self) -> &MultiContainer;
    fn as_multi_container_mut(&mut self) -> &mut MultiContainer;
    fn into_multi_container(self: Box<Self>) -> Box<MultiContainer>;
}

pub trait HasLabel: AsAny + development::seal::Sealed {
    fn label(&self) -> ::std::borrow::Cow<str>;
    fn set_label(&mut self, &str);

    fn as_has_label(&self) -> &HasLabel;
    fn as_has_label_mut(&mut self) -> &mut HasLabel;
    fn into_has_label(self: Box<Self>) -> Box<HasLabel>;
}

pub trait Clickable: AsAny + development::seal::Sealed {
    fn on_click(&mut self, Option<callbacks::Click>);

    fn as_clickable(&self) -> &Clickable;
    fn as_clickable_mut(&mut self) -> &mut Clickable;
    fn into_clickable(self: Box<Self>) -> Box<Clickable>;
}

pub trait Application: AsAny + development::seal::Sealed {
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<Window>;
    fn name(&self) -> ::std::borrow::Cow<str>;
    fn start(&mut self);
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut Member>;
    fn find_member_by_id(&self, id: ids::Id) -> Option<&Member>;
}
impl development::Final for Application {}

pub trait Window: SingleContainer + HasLabel {}
impl development::Final for Window {}

pub trait Button: Control + Clickable + HasLabel {}
impl development::Final for Button {}

pub trait LinearLayout: MultiContainer + Control + HasOrientation {}
impl development::Final for LinearLayout {}

pub trait Frame: SingleContainer + Control + HasLabel {}
impl development::Final for Frame {}

pub trait Splitted: MultiContainer + Control + HasOrientation {
    fn first(&self) -> &Control;
    fn second(&self) -> &Control;
    fn first_mut(&mut self) -> &mut Control;
    fn second_mut(&mut self) -> &mut Control;

    fn set_splitter(&mut self, pos: f32);
    fn splitter(&self) -> f32;
}
impl development::Final for Splitted {}
