use super::{types, ids, layout, traits, callbacks};

pub trait UiMemberExtension {
	fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);
    
    fn set_visibility(&mut self, visibility: types::Visibility);
    fn visibility(&self) -> types::Visibility;
    
    unsafe fn native_id(&self) -> usize;
}
pub trait UiControlExtension: UiMemberExtension + UiDrawable + UiChild {
	fn parent(&self) -> Option<&types::UiMemberBase>;
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase>;
    fn root(&self) -> Option<&types::UiMemberBase>;
    fn root_mut(&mut self) -> Option<&mut types::UiMemberBase>;
	    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, &super::markup::Markup, &mut super::markup::MarkupRegistry);
}
pub trait UiHasLayoutExtension: UiMemberExtension {
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
pub trait UiContainerExtension: UiMemberExtension {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl>;
}
pub trait UiSingleContainerExtension: UiContainerExtension {
	fn set_child(&mut self, Option<Box<traits::UiControl>>) -> Option<Box<traits::UiControl>>;
    fn child(&self) -> Option<&traits::UiControl>;
    fn child_mut(&mut self) -> Option<&mut traits::UiControl>;
}
pub trait UiMultiContainerExtension: UiContainerExtension {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<traits::UiControl>) -> Option<Box<traits::UiControl>>;
    fn remove_child_from(&mut self, index: usize) -> Option<Box<traits::UiControl>>;
    fn child_at(&self, index: usize) -> Option<&Box<traits::UiControl>>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<traits::UiControl>>;
    
	fn clear(&mut self) {
        let len = self.len();
        for index in (0..len).rev() {
            self.remove_child_from(index);
        }
    }
    fn push_child(&mut self, child: Box<traits::UiControl>) {
        let len = self.len();
        self.set_child_to(len, child);
    }
    fn pop_child(&mut self) -> Option<Box<traits::UiControl>> {
        let len = self.len();
        if len > 0 {
        	self.remove_child_from(len - 1)
        } else {
        	None
        }
    }
}
pub trait UiHasLabelExtension {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn set_label(&mut self, &str);
}
pub trait UiClickableExtension {
	fn on_click(&mut self, Option<callbacks::Click>);    
}
pub trait UiHasOrientationExtension {
	fn layout_orientation(&self) -> layout::Orientation;
	fn set_layout_orientation(&mut self, layout::Orientation);
}
pub trait UiWindowExtension: UiSingleContainerExtension + UiHasLabelExtension {
}

pub trait UiButtonExtension: UiControlExtension + UiClickableExtension + UiHasLabelExtension {	
}

pub trait UiLinearLayoutExtension: UiMultiContainerExtension + UiControlExtension + UiHasOrientationExtension {
}



#[repr(C)]
pub struct UiMemberCommon {
    pub id: ids::Id,
    pub visibility: types::Visibility,
    pub on_resize: Option<callbacks::Resize>,
}
impl Default for UiMemberCommon {
	fn default() -> UiMemberCommon {
		UiMemberCommon {
			id: ids::Id::next(),
			visibility: types::Visibility::Visible,
			on_resize: None
		}
	}
}
#[repr(C)]
pub struct UiMemberBase<T: UiMemberExtension + Sized> {
	common: UiMemberCommon,
	inner: T
}
impl <T: UiMemberExtension + Sized> traits::UiIsControl for UiMemberBase<T> {
	default fn is_control(&self) -> Option<&traits::UiControl> { None }
    default fn is_control_mut(&mut self) -> Option<&mut traits::UiControl> { None }
}
impl <T: UiMemberExtension + Sized> traits::UiMember for UiMemberBase<T> {
	fn size(&self) -> (u16, u16) { self.inner.size() }
    fn on_resize(&mut self, callback: Option<callbacks::Resize>) { self.common.on_resize = callback; }

    fn set_visibility(&mut self, visibility: types::Visibility) { self.common.visibility = visibility; }
    fn visibility(&self) -> types::Visibility { self.common.visibility }
    
    fn as_base(&self) -> &types::UiMemberBase { &self.common }
    fn as_base_mut(&mut self) -> &mut types::UiMemberBase { &mut self.common }	
    
    unsafe fn native_id(&self) -> usize { self.inner.native_id() }
}
impl <T: UiContainerExtension + Sized> traits::UiIsSingleContainer for UiMemberBase<T> {
	default fn is_single_mut(&mut self) -> Option<&mut traits::UiSingleContainer> { None }
    default fn is_single(&self) -> Option<&traits::UiSingleContainer> { None }
}
impl <T: UiContainerExtension + Sized> traits::UiIsMultiContainer for UiMemberBase<T> {
	default fn is_multi_mut(&mut self) -> Option<&mut traits::UiMultiContainer> { None }
    default fn is_multi(&self) -> Option<&traits::UiMultiContainer> { None }
}
impl <T: UiContainerExtension + Sized> traits::UiContainer for UiMemberBase<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }

    fn as_member(&self) -> &traits::UiMember { self }
	fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
} 
impl <T: UiHasLayoutExtension + Sized> traits::UiHasLayout for UiMemberBase<T> {
	fn layout_width(&self) -> layout::Size { self.inner.layout_width() }
	fn layout_height(&self) -> layout::Size { self.inner.layout_height() }
	fn layout_gravity(&self) -> layout::Gravity { self.inner.layout_gravity() }
	fn layout_alignment(&self) -> layout::Alignment { self.inner.layout_alignment() }
	fn layout_padding(&self) -> layout::BoundarySize { self.inner.layout_padding() }
	fn layout_margin(&self) -> layout::BoundarySize { self.inner.layout_margin() }
	
	fn set_layout_width(&mut self, width: layout::Size) { self.inner.set_layout_width(width) }
	fn set_layout_height(&mut self, height: layout::Size) { self.inner.set_layout_height(height) }
	fn set_layout_gravity(&mut self, gravity: layout::Gravity) { self.inner.set_layout_gravity(gravity) }
	fn set_layout_alignment(&mut self, alignment: layout::Alignment) { self.inner.set_layout_alignment(alignment) }    
	fn set_layout_padding(&mut self, padding: layout::BoundarySizeArgs) { self.inner.set_layout_padding(padding) }
	fn set_layout_margin(&mut self, margin: layout::BoundarySizeArgs) { self.inner.set_layout_margin(margin) }

    fn as_member(&self) -> &traits::UiMember { self }
	fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
}
impl <T: UiSingleContainerExtension + Sized> traits::UiSingleContainer for UiMemberBase<T> {
	fn set_child(&mut self, child: Option<Box<traits::UiControl>>) -> Option<Box<traits::UiControl>> { self.inner.set_child(child) }
    fn child(&self) -> Option<&traits::UiControl> { self.inner.child() }
    fn child_mut(&mut self) -> Option<&mut traits::UiControl> { self.inner.child_mut() }
    
	fn as_container(&self) -> &traits::UiContainer { self }
	fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
}
impl <T: UiMultiContainerExtension + Sized> traits::UiMultiContainer for UiMemberBase<T> {
	fn len(&self) -> usize { self.inner.len() }
    fn set_child_to(&mut self, index: usize, child: Box<traits::UiControl>) -> Option<Box<traits::UiControl>> { self.inner.set_child_to(index, child) }
    fn remove_child_from(&mut self, index: usize) -> Option<Box<traits::UiControl>> { self.inner.remove_child_from(index) }
    fn child_at(&self, index: usize) -> Option<&Box<traits::UiControl>> { self.inner.child_at(index) }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<traits::UiControl>> { { self.inner.child_at_mut(index) } }
    
	fn clear(&mut self) { self.inner.clear() }
    fn push_child(&mut self, child: Box<traits::UiControl>) { self.inner.push_child(child) }
    fn pop_child(&mut self) -> Option<Box<traits::UiControl>> { self.inner.pop_child() }
    
    fn as_container(&self) -> &traits::UiContainer { self }
	fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
}
impl <T: UiHasLabelExtension + UiMemberExtension + Sized> traits::UiHasLabel for UiMemberBase<T> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.set_label(label) }
}
impl <T: UiClickableExtension + UiMemberExtension + Sized> traits::UiClickable for UiMemberBase<T> {
	fn on_click(&mut self, callback: Option<callbacks::Click>) { self.inner.on_click(callback) }    
}
impl <T: UiHasOrientationExtension + UiMemberExtension + Sized> traits::UiHasOrientation for UiMemberBase<T> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.layout_orientation() }
	fn set_layout_orientation(&mut self, orientation: layout::Orientation) { self.inner.set_layout_orientation(orientation) }
}
impl <T: UiWindowExtension + Sized> traits::UiIsSingleContainer for UiMemberBase<T> {
	fn is_single_mut(&mut self) -> Option<&mut traits::UiSingleContainer> { Some(self) }
    fn is_single(&self) -> Option<&traits::UiSingleContainer> { Some(self) }
}
impl <T: UiWindowExtension + Sized> traits::UiWindow for UiMemberBase<T> {
	fn as_has_label(&self) -> &traits::UiHasLabel { self }
	fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
	fn as_single_container(&self) -> &traits::UiSingleContainer { self }
	fn as_single_container_mut(&mut self) -> &mut traits::UiSingleContainer { self }
}




impl <T: UiControlExtension + Sized> traits::UiIsControl for UiMemberBase<UiControlBase<T>> {
	default fn is_control(&self) -> Option<&traits::UiControl> { Some(self) }
    default fn is_control_mut(&mut self) -> Option<&mut traits::UiControl> { Some(self) }
}
impl <T: UiControlExtension + Sized> traits::UiIsContainer for UiMemberBase<UiControlBase<T>> {
	default fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { None }
    default fn is_container(&self) -> Option<&traits::UiContainer> { None }
}
impl <T: UiControlExtension + Sized> traits::UiControl for UiMemberBase<UiControlBase<T>> {
	fn parent(&self) -> Option<&types::UiMemberBase> { self.inner.inner.parent() }
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase> { self.inner.inner.parent_mut() }
    fn root(&self) -> Option<&types::UiMemberBase> { self.inner.inner.root() }
    fn root_mut(&mut self) -> Option<&mut types::UiMemberBase> { self.inner.inner.root_mut() }
	    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) { { self.inner.inner.fill_from_markup(markup, registry) } }
    
    fn as_has_layout(&self) -> &traits::UiHasLayout { self }
	fn as_has_layout_mut(&mut self) -> &mut traits::UiHasLayout { self }
}
impl <T: UiControlExtension + Sized> UiDrawable for UiMemberBase<UiControlBase<T>> {
	fn draw(&mut self, coords: Option<(i32, i32)>) { self.inner.inner.draw(coords) }
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool) { self.inner.inner.measure(w, h) }
}
impl <T: UiControlExtension + Sized> UiChild for UiMemberBase<UiControlBase<T>> {
	fn on_added_to_container(&mut self, parent: &traits::UiContainer, x: i32, y: i32) { self.inner.inner.on_added_to_container(parent, x, y) }
    fn on_removed_from_container(&mut self, parent: &traits::UiContainer) { self.inner.inner.on_removed_from_container(parent) }
}
impl <T: UiControlExtension + Sized> traits::UiHasLayout for UiMemberBase<UiControlBase<T>> {
	fn layout_width(&self) -> layout::Size { self.inner.common.layout.width }
	fn layout_height(&self) -> layout::Size { self.inner.common.layout.height }
	fn layout_gravity(&self) -> layout::Gravity { self.inner.common.layout.gravity }
	fn layout_alignment(&self) -> layout::Alignment { self.inner.common.layout.alignment }
	fn layout_padding(&self) -> layout::BoundarySize { self.inner.common.layout.padding }
	fn layout_margin(&self) -> layout::BoundarySize { self.inner.common.layout.margin }
	
	fn set_layout_width(&mut self, width: layout::Size) { self.inner.common.layout.width = width }
	fn set_layout_height(&mut self, height: layout::Size) { self.inner.common.layout.height = height }
	fn set_layout_gravity(&mut self, gravity: layout::Gravity) { self.inner.common.layout.gravity = gravity }
	fn set_layout_alignment(&mut self, alignment: layout::Alignment) { self.inner.common.layout.alignment = alignment }
	fn set_layout_padding(&mut self, padding: layout::BoundarySizeArgs) { self.inner.common.layout.padding = padding.into() }
	fn set_layout_margin(&mut self, margin: layout::BoundarySizeArgs) { self.inner.common.layout.margin = margin.into() }
	
	fn as_member(&self) -> &traits::UiMember { self }
	fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
}
impl <T: UiButtonExtension + Sized> traits::UiButton for UiMemberBase<UiControlBase<T>> {
	fn as_control(&self) -> &traits::UiControl { self }
	fn as_control_mut(&mut self) -> &mut traits::UiControl { self }
	fn as_clickable(&self) -> &traits::UiClickable { self }
	fn as_clickable_mut(&mut self) -> &mut traits::UiClickable { self }
	fn as_has_label(&self) -> &traits::UiHasLabel { self }
	fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
}
impl <T: UiLinearLayoutExtension + Sized> traits::UiIsContainer for UiMemberBase<UiControlBase<T>> {
	fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { Some(self) }
    fn is_container(&self) -> Option<&traits::UiContainer> { Some(self) }
}
impl <T: UiLinearLayoutExtension + Sized> traits::UiIsMultiContainer for UiMemberBase<UiControlBase<T>> {
	fn is_multi_mut(&mut self) -> Option<&mut traits::UiMultiContainer> { Some(self) }
    fn is_multi(&self) -> Option<&traits::UiMultiContainer> { Some(self) }
}
impl <T: UiLinearLayoutExtension + Sized> traits::UiLinearLayout for UiMemberBase<UiControlBase<T>> {
	fn as_control(&self) -> &traits::UiControl { self }
	fn as_control_mut(&mut self) -> &mut traits::UiControl { self }
	fn as_multi_container(&self) -> &traits::UiMultiContainer { self }
	fn as_multi_container_mut(&mut self) -> &mut traits::UiMultiContainer { self }
	fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
	fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
}



#[repr(C)]
pub struct UiControlCommon {
    pub layout: layout::Attributes,
}
impl Default for UiControlCommon {
	fn default() -> UiControlCommon {
		UiControlCommon {
			layout: Default::default()
		}
	}
}
#[repr(C)]
pub struct UiControlBase<T: UiControlExtension + Sized> {
	common: UiControlCommon,
	inner: T,
}
impl <T: UiControlExtension + Sized> UiMemberExtension for UiControlBase<T> {
	fn size(&self) -> (u16, u16) { self.inner.size() }
    fn on_resize(&mut self, callback: Option<callbacks::Resize>) { self.inner.on_resize(callback) }
    fn set_visibility(&mut self, visibility: types::Visibility) { self.inner.set_visibility(visibility) }
    fn visibility(&self) -> types::Visibility { self.inner.visibility() }
    
    unsafe fn native_id(&self) -> usize { self.inner.native_id() }
}
impl <T: UiControlExtension + UiContainerExtension + Sized> UiContainerExtension for UiControlBase<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }
}
impl <T: UiControlExtension + UiSingleContainerExtension + Sized> UiSingleContainerExtension for UiControlBase<T> {
	fn set_child(&mut self, child: Option<Box<traits::UiControl>>) -> Option<Box<traits::UiControl>> { self.inner.set_child(child) }
    fn child(&self) -> Option<&traits::UiControl> { self.inner.child() }
    fn child_mut(&mut self) -> Option<&mut traits::UiControl> { self.inner.child_mut() }
}
impl <T: UiControlExtension + UiMultiContainerExtension + Sized> UiMultiContainerExtension for UiControlBase<T> {
	fn len(&self) -> usize { self.inner.len() }
    fn set_child_to(&mut self, index: usize, child: Box<traits::UiControl>) -> Option<Box<traits::UiControl>> { self.inner.set_child_to(index, child) }
    fn remove_child_from(&mut self, index: usize) -> Option<Box<traits::UiControl>> { self.inner.remove_child_from(index) }
    fn child_at(&self, index: usize) -> Option<&Box<traits::UiControl>> { self.inner.child_at(index) }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut Box<traits::UiControl>> { { self.inner.child_at_mut(index) } }
    
	fn clear(&mut self) { self.inner.clear() }
    fn push_child(&mut self, child: Box<traits::UiControl>) { self.inner.push_child(child) }
    fn pop_child(&mut self) -> Option<Box<traits::UiControl>> { self.inner.pop_child() }
}
impl <T: UiControlExtension + UiHasLabelExtension + Sized> UiHasLabelExtension for UiControlBase<T> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.set_label(label) }
}
impl <T: UiControlExtension + UiClickableExtension + Sized> UiClickableExtension for UiControlBase<T> {
	fn on_click(&mut self, callback: Option<callbacks::Click>) { self.inner.on_click(callback) }    
}
impl <T: UiControlExtension + UiHasOrientationExtension + Sized> UiHasOrientationExtension for UiControlBase<T> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.layout_orientation() }
	fn set_layout_orientation(&mut self, orientation: layout::Orientation) { self.inner.set_layout_orientation(orientation) }
}



pub trait UiDrawable {
	fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
}
pub trait UiChild {
	fn on_added_to_container(&mut self, &traits::UiContainer, x: i32, y: i32);
    fn on_removed_from_container(&mut self, &traits::UiContainer);
}