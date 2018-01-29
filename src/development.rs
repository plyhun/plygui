use super::{types, ids, layout, traits, callbacks};

pub trait UiMemberExtension {
	fn size(&self) -> (u16, u16);
    fn on_resize(&mut self, Option<callbacks::Resize>);
    
    unsafe fn native_id(&self) -> usize;
}
pub trait UiControlExtension: UiMemberExtension + UiDrawable {
	fn parent(&self) -> Option<&types::UiMemberBase>;
    fn parent_mut(&mut self) -> Option<&mut types::UiMemberBase>;
    fn root(&self) -> Option<&types::UiMemberBase>;
    fn root_mut(&mut self) -> Option<&mut types::UiMemberBase>;
	    
    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, &super::markup::Markup, &mut super::markup::MarkupRegistry);
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


#[repr(C)]
pub struct UiMemberCommon {
    pub id: ids::Id,
    pub visibility: types::Visibility,
    pub on_resize: Option<callbacks::Resize>,
}
#[repr(C)]
pub struct UiMemberBase<T: UiMemberExtension + Sized> {
	common: UiMemberCommon,
	inner: T
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
	fn on_added_to_container(&mut self, parent: &traits::UiContainer, x: i32, y: i32) { self.inner.inner.on_added_to_container(parent, x, y) }
    fn on_removed_from_container(&mut self, parent: &traits::UiContainer) { self.inner.inner.on_removed_from_container(parent) }
    fn draw(&mut self, coords: Option<(i32, i32)>) { self.inner.inner.draw(coords) }
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool) { self.inner.inner.measure(w, h) }
}
impl <T: UiContainerExtension + Sized> traits::UiContainer for UiMemberBase<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }

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


#[repr(C)]
pub struct UiControlCommon {
    pub layout: layout::Attributes,
}
#[repr(C)]
pub struct UiControlBase<T: UiControlExtension + Sized> {
	common: UiControlCommon,
	inner: T,
}
impl <T: UiControlExtension + Sized> UiMemberExtension for UiControlBase<T> {
	fn size(&self) -> (u16, u16) { self.inner.size() }
    fn on_resize(&mut self, callback: Option<callbacks::Resize>) { self.inner.on_resize(callback) }
    
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



pub trait UiDrawable {
	fn on_added_to_container(&mut self, &traits::UiContainer, x: i32, y: i32);
    fn on_removed_from_container(&mut self, &traits::UiContainer);
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
}
