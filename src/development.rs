//pub mod layout;

use super::{types, ids, layout, callbacks, traits};

use std::fmt::Debug;
use std::hash::Hash;
use std::any::Any;

pub trait NativeId: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Into<usize> {}

// ==========================================================================================================

pub trait HasInner {
	type Inner: Sized;
	
	fn with_inner(inner: Self::Inner) -> Self;
	fn as_inner(&self) -> &Self::Inner;
	fn as_inner_mut(&mut self) -> &mut Self::Inner;
}

pub struct Member<T: MemberInner + Sized> {
    inner: T,
}
pub trait MemberInner {
	type Id: NativeId + Sized;
	
    fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);

    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;

    fn id(&self) -> ids::Id;
    unsafe fn native_id(&self) -> Self::Id;
}
impl <T: MemberInner + Sized + 'static> traits::UiMember for Member<T> {
    fn size(&self) -> (u16, u16) { self.inner.size() }
    fn on_resize(&mut self, cb: Option<callbacks::Resize>) { self.inner.on_resize(cb) }

    fn set_visibility(&mut self, visibility: types::Visibility) { self.inner.set_visibility(visibility) }
    fn visibility(&self) -> types::Visibility { self.inner.visibility() }

    fn id(&self) -> ids::Id { self.inner.id() }
    unsafe fn native_id(&self) -> usize { self.inner.native_id().into() }
    
    default fn is_control(&self) -> Option<&traits::UiControl> { None }
    default fn is_control_mut(&mut self) -> Option<&mut traits::UiControl> { None }
}
impl <T: MemberInner + Sized + 'static> traits::AsAny for Member<T> {
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }
    fn into_any(self: Box<Self>) -> types::Dbox<Any> { Box::new(self) }
}
impl <T: MemberInner + Sized + 'static> seal::Sealed for Member<T> {}

// ===============================================================================================================

pub trait HasLayoutInner: MemberInner {
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
}

impl <T: HasLayoutInner + Sized + 'static> traits::UiHasLayout for Member<T> {
	fn layout_width(&self) -> layout::Size { self.inner.layout_width() }
    fn layout_height(&self) -> layout::Size { self.inner.layout_height() }
    fn layout_gravity(&self) -> layout::Gravity  { self.inner.layout_gravity() }
    fn layout_alignment(&self) -> layout::Alignment { self.inner.layout_alignment() }
    fn layout_padding(&self) -> layout::BoundarySize { self.inner.layout_padding() }
    fn layout_margin(&self) -> layout::BoundarySize { self.inner.layout_margin() }

    fn set_layout_width(&mut self, value: layout::Size) { self.inner.set_layout_width(value) }
    fn set_layout_height(&mut self, value: layout::Size) { self.inner.set_layout_height(value) }
    fn set_layout_gravity(&mut self, value: layout::Gravity) { self.inner.set_layout_gravity(value) }
    fn set_layout_alignment(&mut self, value: layout::Alignment) { self.inner.set_layout_alignment(value) }
    fn set_layout_padding(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_padding(value) }
    fn set_layout_margin(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_margin(value) }

    fn as_member(&self) -> &traits::UiMember { self }
    fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
}

// ===============================================================================================================

pub struct Control<T: ControlInner + Sized> {
	inner: T
}

pub trait ControlInner: HasLayoutInner {
	fn on_added_to_container(&mut self, parent: &traits::UiContainer, x: i32, y: i32);
    fn on_removed_from_container(&mut self, parent: &traits::UiContainer);

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry);
}
impl <T: ControlInner + Sized> MemberInner for Control<T> {
	type Id = T::Id;
	
	fn size(&self) -> (u16, u16) { self.inner.size() }
    fn on_resize(&mut self, cb: Option<callbacks::Resize>) { self.inner.on_resize(cb) }

    fn set_visibility(&mut self, visibility: types::Visibility) { self.inner.set_visibility(visibility) }
    fn visibility(&self) -> types::Visibility { self.inner.visibility() }

    fn id(&self) -> ids::Id { self.inner.id() }
    unsafe fn native_id(&self) -> Self::Id { self.inner.native_id() }
}
impl <T: ControlInner + Sized> HasLayoutInner for Control<T> {
	fn layout_width(&self) -> layout::Size { self.inner.layout_width() }
    fn layout_height(&self) -> layout::Size { self.inner.layout_height() }
    fn layout_gravity(&self) -> layout::Gravity  { self.inner.layout_gravity() }
    fn layout_alignment(&self) -> layout::Alignment { self.inner.layout_alignment() }
    fn layout_padding(&self) -> layout::BoundarySize { self.inner.layout_padding() }
    fn layout_margin(&self) -> layout::BoundarySize { self.inner.layout_margin() }

    fn set_layout_width(&mut self, value: layout::Size) { self.inner.set_layout_width(value) }
    fn set_layout_height(&mut self, value: layout::Size) { self.inner.set_layout_height(value) }
    fn set_layout_gravity(&mut self, value: layout::Gravity) { self.inner.set_layout_gravity(value) }
    fn set_layout_alignment(&mut self, value: layout::Alignment) { self.inner.set_layout_alignment(value) }
    fn set_layout_padding(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_padding(value) }
    fn set_layout_margin(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_margin(value) }
}
impl <T: ControlInner + Sized + 'static> traits::UiControl for Member<Control<T>> {
	fn on_added_to_container(&mut self, parent: &traits::UiContainer, x: i32, y: i32) { self.inner.inner.on_added_to_container(parent, x, y) }
    fn on_removed_from_container(&mut self, parent: &traits::UiContainer) { self.inner.inner.on_removed_from_container(parent) }

    default fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { None }
    default fn is_container(&self) -> Option<&traits::UiContainer> { None }

    default fn parent(&self) -> Option<&traits::UiMember> { None }
    default fn parent_mut(&mut self) -> Option<&mut traits::UiMember> { None }
    default fn root(&self) -> Option<&traits::UiMember> { None }
    default fn root_mut(&mut self) -> Option<&mut traits::UiMember> { None }

    #[cfg(feature = "markup")]
    default fn fill_from_markup(&mut self, _markup: &super::markup::Markup, _registry: &mut super::markup::MarkupRegistry) { unimplemented!() } 

    fn as_has_layout(&self) -> &traits::UiHasLayout { self }
    fn as_has_layout_mut(&mut self) -> &mut traits::UiHasLayout { self }
}
impl <T: ControlInner + Sized> UiDrawable for Member<Control<T>> {
	default fn draw(&mut self, _coords: Option<(i32, i32)>) { unimplemented!() }
    default fn measure(&mut self, _w: u16, _h: u16) -> (u16, u16, bool) { unimplemented!() }
}
impl <T: ControlInner + Sized + 'static> traits::UiMember for Member<Control<T>> {
    fn is_control(&self) -> Option<&traits::UiControl> { Some(self) }
    fn is_control_mut(&mut self) -> Option<&mut traits::UiControl> { Some(self) }
}

// ===============================================================================================================

pub trait ContainerInner: MemberInner {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl>;
}
impl <T: ContainerInner + Sized + 'static> traits::UiContainer for Member<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }
    
    fn as_member(&self) -> &traits::UiMember { self }
    fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
}

// ===============================================================================================================

pub trait SingleContainerInner: ContainerInner {
	fn set_child(&mut self, Option<types::Dbox<traits::UiControl>>) -> Option<types::Dbox<traits::UiControl>>;
    fn child(&self) -> Option<&traits::UiControl>;
    fn child_mut(&mut self) -> Option<&mut traits::UiControl>;
}

pub struct SingleContainer<T: SingleContainerInner + Sized + 'static> {
	inner: T
}

impl <T: SingleContainerInner + Sized + 'static> MemberInner for SingleContainer<T> {
	type Id = T::Id;
	
	fn size(&self) -> (u16, u16) { self.inner.size() }
    fn on_resize(&mut self, cb: Option<callbacks::Resize>) { self.inner.on_resize(cb) }

    fn set_visibility(&mut self, visibility: types::Visibility) { self.inner.set_visibility(visibility) }
    fn visibility(&self) -> types::Visibility { self.inner.visibility() }

    fn id(&self) -> ids::Id { self.inner.id() }
    unsafe fn native_id(&self) -> Self::Id { self.inner.native_id() }
}
impl <T: SingleContainerInner + ContainerInner + Sized + 'static> ContainerInner for SingleContainer<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }
}
impl <T: SingleContainerInner + HasLayoutInner + Sized + 'static> HasLayoutInner for SingleContainer<T> {
	fn layout_width(&self) -> layout::Size { self.inner.layout_width() }
    fn layout_height(&self) -> layout::Size { self.inner.layout_height() }
    fn layout_gravity(&self) -> layout::Gravity  { self.inner.layout_gravity() }
    fn layout_alignment(&self) -> layout::Alignment { self.inner.layout_alignment() }
    fn layout_padding(&self) -> layout::BoundarySize { self.inner.layout_padding() }
    fn layout_margin(&self) -> layout::BoundarySize { self.inner.layout_margin() }

    fn set_layout_width(&mut self, value: layout::Size) { self.inner.set_layout_width(value) }
    fn set_layout_height(&mut self, value: layout::Size) { self.inner.set_layout_height(value) }
    fn set_layout_gravity(&mut self, value: layout::Gravity) { self.inner.set_layout_gravity(value) }
    fn set_layout_alignment(&mut self, value: layout::Alignment) { self.inner.set_layout_alignment(value) }
    fn set_layout_padding(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_padding(value) }
    fn set_layout_margin(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_margin(value) }
}
impl <T: SingleContainerInner + ControlInner + Sized + 'static> ControlInner for SingleContainer<T> {
	fn on_added_to_container(&mut self, parent: &traits::UiContainer, x: i32, y: i32) { self.inner.on_added_to_container(parent, x, y) }
    fn on_removed_from_container(&mut self, parent: &traits::UiContainer) { self.inner.on_removed_from_container(parent) }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) { self.inner.fill_from_markup(markup, registry) }
}
impl <T: SingleContainerInner + Sized + 'static> traits::UiSingleContainer for Member<SingleContainer<T>> {
	fn set_child(&mut self, child: Option<types::Dbox<traits::UiControl>>) -> Option<types::Dbox<traits::UiControl>> { self.inner.inner.set_child(child) }
    fn child(&self) -> Option<&traits::UiControl> { self.inner.inner.child() }
    fn child_mut(&mut self) -> Option<&mut traits::UiControl> { self.inner.inner.child_mut() }

    fn as_container(&self) -> &traits::UiContainer { self }
    fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
}
impl <T: SingleContainerInner + Sized + 'static> traits::UiContainer for Member<SingleContainer<T>> {
	fn is_single_mut(&mut self) -> Option<&mut traits::UiSingleContainer> { Some(self) }
    fn is_single(&self) -> Option<&traits::UiSingleContainer> { Some(self) }
}
impl <T: SingleContainerInner + ControlInner + Sized + 'static> traits::UiSingleContainer for Member<Control<SingleContainer<T>>> {
	fn set_child(&mut self, child: Option<types::Dbox<traits::UiControl>>) -> Option<types::Dbox<traits::UiControl>> { self.inner.inner.inner.set_child(child) }
    fn child(&self) -> Option<&traits::UiControl> { self.inner.inner.inner.child() }
    fn child_mut(&mut self) -> Option<&mut traits::UiControl> { self.inner.inner.inner.child_mut() }

    fn as_container(&self) -> &traits::UiContainer { self }
    fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
}

impl <T: SingleContainerInner + ControlInner + Sized + 'static> traits::UiContainer for Member<Control<SingleContainer<T>>> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.inner.find_control_by_id(id) }
    
    fn as_member(&self) -> &traits::UiMember { self }
    fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
    
    fn is_single_mut(&mut self) -> Option<&mut traits::UiSingleContainer> { Some(self) }
    fn is_single(&self) -> Option<&traits::UiSingleContainer> { Some(self) }
}

// ===============================================================================================================

pub trait MultiContainerInner: ContainerInner {
	fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, types::Dbox<traits::UiControl>) -> Option<types::Dbox<traits::UiControl>>;
    fn remove_child_from(&mut self, index: usize) -> Option<types::Dbox<traits::UiControl>>;
    fn child_at(&self, index: usize) -> Option<&traits::UiControl>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut traits::UiControl>;
}

pub struct MultiContainer<T: MultiContainerInner + Sized + 'static> {
	inner: T
}

impl <T: MultiContainerInner + Sized + 'static> MemberInner for MultiContainer<T> {
	type Id = T::Id;
	
	fn size(&self) -> (u16, u16) { self.inner.size() }
    fn on_resize(&mut self, cb: Option<callbacks::Resize>) { self.inner.on_resize(cb) }

    fn set_visibility(&mut self, visibility: types::Visibility) { self.inner.set_visibility(visibility) }
    fn visibility(&self) -> types::Visibility { self.inner.visibility() }

    fn id(&self) -> ids::Id { self.inner.id() }
    unsafe fn native_id(&self) -> Self::Id { self.inner.native_id() }
}
impl <T: MultiContainerInner + ContainerInner + Sized + 'static> ContainerInner for MultiContainer<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }
}
impl <T: MultiContainerInner + HasLayoutInner + Sized + 'static> HasLayoutInner for MultiContainer<T> {
	fn layout_width(&self) -> layout::Size { self.inner.layout_width() }
    fn layout_height(&self) -> layout::Size { self.inner.layout_height() }
    fn layout_gravity(&self) -> layout::Gravity  { self.inner.layout_gravity() }
    fn layout_alignment(&self) -> layout::Alignment { self.inner.layout_alignment() }
    fn layout_padding(&self) -> layout::BoundarySize { self.inner.layout_padding() }
    fn layout_margin(&self) -> layout::BoundarySize { self.inner.layout_margin() }

    fn set_layout_width(&mut self, value: layout::Size) { self.inner.set_layout_width(value) }
    fn set_layout_height(&mut self, value: layout::Size) { self.inner.set_layout_height(value) }
    fn set_layout_gravity(&mut self, value: layout::Gravity) { self.inner.set_layout_gravity(value) }
    fn set_layout_alignment(&mut self, value: layout::Alignment) { self.inner.set_layout_alignment(value) }
    fn set_layout_padding(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_padding(value) }
    fn set_layout_margin(&mut self, value: layout::BoundarySizeArgs) { self.inner.set_layout_margin(value) }
}
impl <T: MultiContainerInner + ControlInner + Sized + 'static> ControlInner for MultiContainer<T> {
	fn on_added_to_container(&mut self, parent: &traits::UiContainer, x: i32, y: i32) { self.inner.on_added_to_container(parent, x, y) }
    fn on_removed_from_container(&mut self, parent: &traits::UiContainer) { self.inner.on_removed_from_container(parent) }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) { self.inner.fill_from_markup(markup, registry) }
}
impl <T: MultiContainerInner + Sized + 'static> traits::UiMultiContainer for Member<MultiContainer<T>> {
	fn len(&self) -> usize { self.inner.inner.len() }
    fn set_child_to(&mut self, index: usize, child: types::Dbox<traits::UiControl>) -> Option<types::Dbox<traits::UiControl>> { self.inner.inner.set_child_to(index, child) }
    fn remove_child_from(&mut self, index: usize) -> Option<types::Dbox<traits::UiControl>> { self.inner.inner.remove_child_from(index) }
    fn child_at(&self, index: usize) -> Option<&traits::UiControl> { self.inner.inner.child_at(index) }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut traits::UiControl> { self.inner.inner.child_at_mut(index) }

    fn as_container(&self) -> &traits::UiContainer { self }
    fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
}

impl <T: MultiContainerInner + Sized + 'static> traits::UiContainer for Member<MultiContainer<T>> {
	fn is_multi_mut(&mut self) -> Option<&mut traits::UiMultiContainer> { Some(self) }
    fn is_multi(&self) -> Option<&traits::UiMultiContainer> { Some(self) }
}
impl <T: MultiContainerInner + ControlInner + Sized + 'static> traits::UiContainer for Member<Control<MultiContainer<T>>> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.inner.find_control_by_id(id) }
    
    fn as_member(&self) -> &traits::UiMember { self }
    fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
    
    fn is_multi_mut(&mut self) -> Option<&mut traits::UiMultiContainer> { Some(self) }
    fn is_multi(&self) -> Option<&traits::UiMultiContainer> { Some(self) }
}
impl <T: MultiContainerInner + ControlInner + Sized + 'static> traits::UiMultiContainer for Member<Control<MultiContainer<T>>> {
	fn len(&self) -> usize { self.inner.inner.inner.len() }
    fn set_child_to(&mut self, index: usize, child: types::Dbox<traits::UiControl>) -> Option<types::Dbox<traits::UiControl>> { self.inner.inner.inner.set_child_to(index, child) }
    fn remove_child_from(&mut self, index: usize) -> Option<types::Dbox<traits::UiControl>> { self.inner.inner.inner.remove_child_from(index) }
    fn child_at(&self, index: usize) -> Option<&traits::UiControl> { self.inner.inner.inner.child_at(index) }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut traits::UiControl> { self.inner.inner.inner.child_at_mut(index) }

    fn as_container(&self) -> &traits::UiContainer { self }
    fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
}
impl <T: MultiContainerInner + Sized + 'static> Member<MultiContainer<T>> {
	pub fn new(inner: T) -> Member<MultiContainer<T>> {
		Member { inner: MultiContainer { inner: inner } }
	}
}
impl <T: MultiContainerInner + ControlInner + Sized + 'static> Member<Control<MultiContainer<T>>> {
	pub fn new(inner: T) -> Member<Control<MultiContainer<T>>> {
		Member { inner: Control { inner: MultiContainer { inner: inner } } }
	}
}

// ===============================================================================================================

pub trait HasLabelInner {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn set_label(&mut self, &str);
}
impl <T: HasLabelInner + MemberInner + Sized + 'static> traits::UiHasLabel for Member<T> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.set_label(label) }
}
impl <T: HasLabelInner + ControlInner + Sized + 'static> traits::UiHasLabel for Member<Control<T>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.set_label(label) }
}
impl <T: HasLabelInner + SingleContainerInner + Sized + 'static> traits::UiHasLabel for Member<SingleContainer<T>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.set_label(label) }
}
impl <T: HasLabelInner + MultiContainerInner + Sized + 'static> traits::UiHasLabel for Member<MultiContainer<T>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.set_label(label) }
}
impl <T: HasLabelInner + ControlInner + SingleContainerInner + Sized + 'static> traits::UiHasLabel for Member<Control<SingleContainer<T>>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.inner.set_label(label) }
}
impl <T: HasLabelInner + ControlInner + MultiContainerInner + Sized + 'static> traits::UiHasLabel for Member<Control<MultiContainer<T>>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.inner.set_label(label) }
}

// ===============================================================================================================

pub trait ClickableInner {
	fn on_click(&mut self, cb: Option<callbacks::Click>);
}
impl <T: ClickableInner + MemberInner + Sized + 'static> traits::UiClickable for Member<T> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.on_click(cb) }
}
impl <T: ClickableInner + ControlInner + Sized + 'static> traits::UiClickable for Member<Control<T>> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.inner.on_click(cb) }
}
impl <T: ClickableInner + ControlInner + SingleContainerInner + Sized + 'static> traits::UiClickable for Member<Control<SingleContainer<T>>> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.inner.inner.on_click(cb) }
}
impl <T: ClickableInner + ControlInner + MultiContainerInner + Sized + 'static> traits::UiClickable for Member<Control<MultiContainer<T>>> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.inner.inner.on_click(cb) }
}

// ===============================================================================================================

pub trait ApplicationInner {
	fn new_window(&mut self, title: &str, size: types::WindowStartSize, has_menu: bool) -> types::Dbox<traits::UiWindow>;
    fn name<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn start(&mut self);
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiMember>;
    fn find_member_by_id(&self, id: ids::Id) -> Option<&traits::UiMember>;
}
pub struct Application<T: ApplicationInner + Sized + 'static> {
	inner: T
}
impl <T: ApplicationInner + Sized + 'static> traits::UiApplication for Application<T> {
	fn new_window(&mut self, title: &str, size: types::WindowStartSize, has_menu: bool) -> types::Dbox<traits::UiWindow> {
		self.inner.new_window(title, size, has_menu)
	}
    fn name<'a>(&'a self) -> ::std::borrow::Cow<'a, str> {
	    self.inner.name()
    }
    fn start(&mut self) { self.inner.start() }
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiMember> { self.inner.find_member_by_id_mut(id) }
    fn find_member_by_id(&self, id: ids::Id) -> Option<&traits::UiMember> { self.inner.find_member_by_id(id) }
}
impl <T: ApplicationInner + Sized + 'static> traits::AsAny for Application<T> {
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }
    fn into_any(self: Box<Self>) -> types::Dbox<Any> { Box::new(self) }
}

impl <T: ApplicationInner + Sized> HasInner for Application<T> {
	type Inner = T;
	
	fn with_inner(inner: Self::Inner) -> Self { Application { inner } }
	fn as_inner(&self) -> &Self::Inner { &self.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner }
}
impl <T: ApplicationInner + Sized + 'static> seal::Sealed for Application<T> {}

// ===============================================================================================================

pub trait HasOrientationInner {
	fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, layout::Orientation);
}
impl <T: HasOrientationInner + MemberInner + Sized + 'static> traits::UiHasOrientation for Member<T> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.set_layout_orientation(value) }
}
impl <T: HasOrientationInner + ControlInner + Sized + 'static> traits::UiHasOrientation for Member<Control<T>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.set_layout_orientation(value) }
}
impl <T: HasOrientationInner + SingleContainerInner + Sized + 'static> traits::UiHasOrientation for Member<SingleContainer<T>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.set_layout_orientation(value) }
}
impl <T: HasOrientationInner + MultiContainerInner + Sized + 'static> traits::UiHasOrientation for Member<MultiContainer<T>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.set_layout_orientation(value) }
}
impl <T: HasOrientationInner + SingleContainerInner + ControlInner + Sized + 'static> traits::UiHasOrientation for Member<Control<SingleContainer<T>>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.inner.set_layout_orientation(value) }
}
impl <T: HasOrientationInner + MultiContainerInner + ControlInner + Sized + 'static> traits::UiHasOrientation for Member<Control<MultiContainer<T>>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.inner.set_layout_orientation(value) }
}

// ===============================================================================================================

pub trait WindowInner: HasLabelInner + SingleContainerInner {}

impl <T: WindowInner + Sized + 'static> traits::UiWindow for Member<SingleContainer<T>> {
	fn as_single_container(&self) -> &traits::UiSingleContainer { self }
    fn as_single_container_mut(&mut self) -> &mut traits::UiSingleContainer { self }
}
impl <T: WindowInner + Sized> HasInner for Member<SingleContainer<T>> {
	type Inner = T;
	
	fn with_inner(inner: Self::Inner) -> Self { Member { inner: SingleContainer { inner } } }
	fn as_inner(&self) -> &Self::Inner { &self.inner.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner.inner }
}

// ===============================================================================================================

pub trait ButtonInner: ControlInner + ClickableInner + HasLabelInner {}

impl <T: ButtonInner + Sized + 'static> traits::UiButton for Member<Control<T>> {
	fn as_control(&self) -> &traits::UiControl { self }
    fn as_control_mut(&mut self) -> &mut traits::UiControl { self }
    fn as_clickable(&self) -> &traits::UiClickable { self }
    fn as_clickable_mut(&mut self) -> &mut traits::UiClickable { self }
    fn as_has_label(&self) -> &traits::UiHasLabel { self }
    fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
}
impl <T: ButtonInner + Sized> HasInner for Member<Control<T>> {
	type Inner = T;
	
	fn with_inner(inner: Self::Inner) -> Self { Member { inner: Control { inner } } }
	fn as_inner(&self) -> &Self::Inner { &self.inner.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner.inner }
}

// ===============================================================================================================

pub trait LinearLayoutInner: ControlInner + MultiContainerInner + HasOrientationInner {}

impl <T: LinearLayoutInner + Sized + 'static> traits::UiLinearLayout for Member<Control<MultiContainer<T>>> {
	fn as_control(&self) -> &traits::UiControl { self }
    fn as_control_mut(&mut self) -> &mut traits::UiControl { self }
    fn as_multi_container(&self) -> &traits::UiMultiContainer { self }
    fn as_multi_container_mut(&mut self) -> &mut traits::UiMultiContainer { self }
    fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
    fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
}
impl <T: LinearLayoutInner + Sized> HasInner for Member<Control<MultiContainer<T>>> {
	type Inner = T;
	
	fn with_inner(inner: Self::Inner) -> Self { Member { inner: Control { inner: MultiContainer { inner } } } }
	fn as_inner(&self) -> &Self::Inner { &self.inner.inner.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner.inner.inner }
}

// ===============================================================================================================

pub trait UiDrawable {
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
}

pub(crate) mod seal {
	pub trait Sealed {}
}