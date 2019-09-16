pub use crate::auto::{AsAny, Clickable, Closeable, HasImage, HasLabel, HasNativeId, HasProgress, HasSize, HasVisibility, MaybeContainer, MaybeControl, MaybeHasSize, MaybeHasVisibility, MaybeMember};
use crate::{callbacks, development, ids, layout, types};

#[cfg(feature = "type_check")]
use std::any::TypeId;
use std::borrow::Cow;

// ===============================================================================================================

pub trait Member: HasNativeId + MaybeControl + MaybeContainer + MaybeHasSize + MaybeHasVisibility + AsAny + development::seal::Sealed {
    fn id(&self) -> ids::Id;
    fn tag(&self) -> Option<Cow<str>>;
    fn set_tag(&mut self, tag: Option<Cow<str>>);

    #[cfg(feature = "type_check")]
    unsafe fn type_id(&self) -> TypeId;

    fn as_member(&self) -> &dyn Member;
    fn as_member_mut(&mut self) -> &mut dyn Member;
    fn into_member(self: Box<Self>) -> Box<dyn Member>;
}

pub trait HasOrientation: Member + development::seal::Sealed {
    fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, orientation: layout::Orientation);

    fn as_has_orientation(&self) -> &dyn HasOrientation;
    fn as_has_orientation_mut(&mut self) -> &mut dyn HasOrientation;
    fn into_has_orientation(self: Box<Self>) -> Box<dyn HasOrientation>;
}

pub trait HasLayout: Member {
    fn layout_width(&self) -> layout::Size;
    fn layout_height(&self) -> layout::Size;

    fn layout_margin(&self) -> layout::BoundarySize;

    fn set_layout_width(&mut self, width: layout::Size);
    fn set_layout_height(&mut self, height: layout::Size);

    fn as_has_layout(&self) -> &dyn HasLayout;
    fn as_has_layout_mut(&mut self) -> &mut dyn HasLayout;
    fn into_has_layout(self: Box<Self>) -> Box<dyn HasLayout>;
}

pub trait Control: HasSize + HasVisibility + HasLayout + development::OuterDrawable {
    fn on_added_to_container(&mut self, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16);
    fn on_removed_from_container(&mut self, parent: &dyn Container);

    fn parent(&self) -> Option<&dyn Member>;
    fn parent_mut(&mut self) -> Option<&mut dyn Member>;
    fn root(&self) -> Option<&dyn Member>;
    fn root_mut(&mut self) -> Option<&mut dyn Member>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry);

    fn as_control(&self) -> &dyn Control;
    fn as_control_mut(&mut self) -> &mut dyn Control;
    fn into_control(self: Box<Self>) -> Box<dyn Control>;
}

pub trait Container: Member {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Control>;
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn Control>;

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
    fn set_child(&mut self, child: Option<Box<dyn Control>>) -> Option<Box<dyn Control>>;
    fn child(&self) -> Option<&dyn Control>;
    fn child_mut(&mut self) -> Option<&mut dyn Control>;

    fn as_single_container(&self) -> &dyn SingleContainer;
    fn as_single_container_mut(&mut self) -> &mut dyn SingleContainer;
    fn into_single_container(self: Box<Self>) -> Box<dyn SingleContainer>;
}

pub trait MultiContainer: Container {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>>;
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

pub trait Application: HasNativeId + AsAny + development::seal::Sealed {
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn Window>;
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn Tray>;
    fn name(&self) -> ::std::borrow::Cow<'_, str>;
    fn start(&mut self);
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn Member>;
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn Member>;
    fn exit(self: Box<Self>, skip_on_close: bool) -> bool;
    fn on_frame(&mut self, cb: callbacks::OnFrame);
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<callbacks::OnFrame>;

    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn Member)> + 'a>; //E0562 :(
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn Member)> + 'a>; //E0562 :(
}
//impl <T: Application> development::Final for T {}

pub trait Window: HasSize + HasVisibility + SingleContainer + HasLabel + Closeable {}
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

pub trait Text: Control + HasLabel {}
//impl <T: Text> development::Final for T {}

pub trait Message: Member + HasLabel {
    fn severity(&self) -> types::MessageSeverity;
    fn start(self: Box<Self>) -> Result<String, ()>;
}
//impl <T: Message> development::Final for T {}

pub trait Tray: Member + HasLabel + HasImage + Closeable {}
//impl <T: Tray> development::Final for T {}

pub trait Image: Control {
    fn set_scale(&mut self, policy: types::ImageScalePolicy);
    fn scale(&self) -> types::ImageScalePolicy;
}
//impl <T: Image> development::Final for T {}

pub trait ProgressBar: Control + HasProgress {}
//impl <T: ProgressBar> development::Final for T {}

pub trait Table: Control + MultiContainer {
    fn row_len(&self) -> usize;
    fn column_len(&self) -> usize;
    fn table_child_at(&self, row: usize, col: usize) -> Option<&dyn Control>;
    fn table_child_at_mut(&mut self, row: usize, col: usize) -> Option<&mut dyn Control>;
    
    fn set_table_child_to(&mut self, row: usize, col: usize, child: Box<dyn Control>) -> Option<Box<dyn Control>>;
    fn remove_table_child_from(&mut self, row: usize, col: usize) -> Option<Box<dyn Control>>;
    
    fn add_row(&mut self) -> usize;
    fn add_column(&mut self) -> usize;
    fn insert_row(&mut self, row: usize) -> usize;
    fn insert_column(&mut self, col: usize) -> usize;
    fn delete_row(&mut self, row: usize) -> usize;
    fn delete_column(&mut self, col: usize) -> usize;
}
//impl <T: Table> development::Final for T {}

