use super::{types, ids, layout, callbacks, traits, utils};

use std::fmt::Debug;
use std::hash::Hash;
use std::any::Any;

pub trait NativeId: Debug + Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Hash + Into<usize> {}

// ==========================================================================================================

pub trait HasInner: Sized + 'static {
	type Inner: Sized;
	type Params: Sized;
	
	fn new(inner: Self::Inner, params: Self::Params) -> Self;
	fn as_inner(&self) -> &Self::Inner;
	fn as_inner_mut(&mut self) -> &mut Self::Inner;
}

#[repr(C)]
pub struct MemberBase {
    pub id: ids::Id,
    pub visibility: types::Visibility,
    pub functions: MemberFunctions,
    
    pub handler_resize: Option<callbacks::Resize>,
}
#[repr(C)]
pub struct Member<T: MemberInner> {
	base: MemberBase,
    inner: T,
}
#[repr(C)]
pub struct MemberFunctions {
	_as_any: unsafe fn (&MemberBase) -> &Any,
    _as_any_mut : unsafe fn (&mut MemberBase) -> &mut Any,
    _as_member: unsafe fn (&MemberBase) -> &traits::UiMember,
    _as_member_mut : unsafe fn (&mut MemberBase) -> &mut traits::UiMember,
}
impl MemberFunctions {
	pub fn new(
			_as_any: unsafe fn (&MemberBase) -> &Any, 
			_as_any_mut: unsafe fn (&mut MemberBase) -> &mut Any,
			_as_member: unsafe fn (&MemberBase) -> &traits::UiMember,
		    _as_member_mut : unsafe fn (&mut MemberBase) -> &mut traits::UiMember,
	) -> Self {
		MemberFunctions {_as_any,_as_any_mut,_as_member, _as_member_mut }
	}
}

impl MemberBase {
	pub fn with_functions(functions: MemberFunctions) -> Self {
		MemberBase {
			id: ids::Id::next(),
			visibility: types::Visibility::Visible,
			functions: functions,
			
			handler_resize: None,
		}
	}
	pub fn as_any(&self) -> &Any { unsafe { (self.functions._as_any)(self) } }
    pub fn as_any_mut(&mut self) -> &mut Any { unsafe { (self.functions._as_any_mut)(self) } }
    pub fn as_member(&self) -> &traits::UiMember { unsafe { (self.functions._as_member)(self) } }
    pub fn as_member_mut(&mut self) -> &mut traits::UiMember { unsafe { (self.functions._as_member_mut)(self) } }
}
impl <T: MemberInner> Member<T> {
	pub fn base(&self) -> &MemberBase { &self.base }
	pub fn base_mut(&mut self) -> &mut MemberBase { &mut self.base }
}

pub trait MemberInner: Sized + 'static {
	type Id: NativeId + Sized;
	
    fn size(&self, base: &MemberBase) -> (u16, u16);
    
    fn on_set_visibility(&mut self, base: &mut MemberBase);
    
    unsafe fn native_id(&self) -> Self::Id;
}
impl <T: MemberInner> traits::UiMember for Member<T> {
    fn size(&self) -> (u16, u16) { self.inner.size(&self.base) }
    fn on_resize(&mut self, cb: Option<callbacks::Resize>) { 
	    self.base.handler_resize = cb;	
    }

    fn set_visibility(&mut self, visibility: types::Visibility) { 
    	if self.base.visibility != visibility {
    		self.base.visibility = visibility;
    		self.inner.on_set_visibility(&mut self.base);
    	}
    }
    fn visibility(&self) -> types::Visibility { self.base.visibility }

    fn id(&self) -> ids::Id { self.base.id }
    unsafe fn native_id(&self) -> usize { self.inner.native_id().into() }
    
    default fn is_control(&self) -> Option<&traits::UiControl> { None }
    default fn is_control_mut(&mut self) -> Option<&mut traits::UiControl> { None }
    default fn is_container(&self) -> Option<&traits::UiContainer> { None }
    default fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { None }
    
    fn as_member(&self) -> &traits::UiMember { self }
    fn as_member_mut(&mut self) -> &mut traits::UiMember { self }
    fn into_member(self: Box<Self>) -> Box<traits::UiMember> { self }
}
impl <T: MemberInner> traits::AsAny for Member<T> {
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }
    fn into_any(self: Box<Self>) -> Box<Any> { self }
}
impl <T: MemberInner> seal::Sealed for Member<T> {}

// ===============================================================================================================

pub trait HasLayoutInner: MemberInner {
	fn on_layout_changed(&mut self, base: &mut layout::Attributes);
}

impl <T: ControlInner> traits::UiHasLayout for Member<Control<T>> {
	fn layout_width(&self) -> layout::Size { self.inner.base.layout.width }
    fn layout_height(&self) -> layout::Size { self.inner.base.layout.height }
    fn layout_alignment(&self) -> layout::Alignment { self.inner.base.layout.alignment }
    fn layout_padding(&self) -> layout::BoundarySize { self.inner.base.layout.padding }
    fn layout_margin(&self) -> layout::BoundarySize { self.inner.base.layout.margin }

    fn set_layout_width(&mut self, value: layout::Size) { self.inner.base.layout.width = value; self.inner.inner.on_layout_changed(&mut self.inner.base.layout); }
    fn set_layout_height(&mut self, value: layout::Size) { self.inner.base.layout.height = value; self.inner.inner.on_layout_changed(&mut self.inner.base.layout); }
    fn set_layout_alignment(&mut self, value: layout::Alignment) { self.inner.base.layout.alignment = value; self.inner.inner.on_layout_changed(&mut self.inner.base.layout); }
    fn set_layout_padding(&mut self, value: layout::BoundarySizeArgs) { self.inner.base.layout.padding = value.into(); self.inner.inner.on_layout_changed(&mut self.inner.base.layout); }
    fn set_layout_margin(&mut self, value: layout::BoundarySizeArgs) { self.inner.base.layout.margin = value.into(); self.inner.inner.on_layout_changed(&mut self.inner.base.layout); }
    
    fn as_has_layout(&self) -> &traits::UiHasLayout { self }
    fn as_has_layout_mut(&mut self) -> &mut traits::UiHasLayout { self }
    fn into_has_layout(self: Box<Self>) -> Box<traits::UiHasLayout> { self }
}

// ===============================================================================================================

pub trait Drawable: Sized + 'static {
    fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>);
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool);
    fn invalidate(&mut self, base: &mut MemberControlBase);
}

// ===============================================================================================================

pub trait ControlInner: HasLayoutInner + Drawable {
	fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &traits::UiContainer, x: i32, y: i32);
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &traits::UiContainer);
    
    fn parent(&self) -> Option<&traits::UiMember>;
    fn parent_mut(&mut self) -> Option<&mut traits::UiMember>;
    fn root(&self) -> Option<&traits::UiMember>;
    fn root_mut(&mut self) -> Option<&mut traits::UiMember>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut MemberControlBase, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry);
}

#[repr(C)]
pub struct MemberControlBase {
    pub member: MemberBase,
    pub control: ControlBase,
}

#[repr(C)]
pub struct ControlBase {
    pub layout: layout::Attributes,
}
#[repr(C)]
pub struct Control<T: ControlInner> {
	base: ControlBase,
	inner: T
}

impl Default for ControlBase {
	fn default() -> Self {
		ControlBase {
			layout: layout::Attributes::default(),
		}
	}
}
impl <T: ControlInner> Control<T> {
	pub fn base(&self) -> &ControlBase { &self.base }
	pub fn base_mut(&mut self) -> &mut ControlBase { &mut self.base }
}

impl <T: ControlInner> MemberInner for Control<T> {
	type Id = T::Id;
	
	fn size(&self, base: &MemberBase) -> (u16, u16) { self.inner.size(base) }
    
    fn on_set_visibility(&mut self, base: &mut MemberBase) { self.inner.on_set_visibility(base) }
    
    unsafe fn native_id(&self) -> Self::Id { self.inner.native_id() }
}
impl <T: ControlInner> HasLayoutInner for Control<T> {
	fn on_layout_changed(&mut self, base: &mut layout::Attributes) { self.inner.on_layout_changed(base) }
}
impl <T: ControlInner> OuterDrawable for Member<Control<T>> {
	fn draw(&mut self, coords: Option<(i32, i32)>) { 
		self.inner.inner.draw(
			unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
			coords
		) 
	}
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool) { 
    	self.inner.inner.measure(
    		unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
			w, h
    	) 
    }
    fn invalidate(&mut self) {
    	self.inner.inner.invalidate(
			unsafe { utils::member_control_base_mut_unchecked(&mut self.base) }
		) 
    }    
    fn as_drawable(&self) -> &OuterDrawable { self }
    fn as_drawable_mut(&mut self) -> &mut OuterDrawable { self }
    fn into_drawable(self: Box<Self>) -> Box<OuterDrawable> { self }
}
impl <T: ControlInner> traits::UiControl for Member<Control<T>> {
	fn on_added_to_container(&mut self, parent: &traits::UiContainer, x: i32, y: i32) { 
		self.inner.inner.on_added_to_container(
			unsafe { utils::member_control_base_mut_unchecked(&mut self.base) }, 
			parent, 
			x, y
		) 
	}
    fn on_removed_from_container(&mut self, parent: &traits::UiContainer) { 
    	self.inner.inner.on_removed_from_container(
    		unsafe { utils::member_control_base_mut_unchecked(&mut self.base) }, 
    		parent
    	) 
    }

    fn parent(&self) -> Option<&traits::UiMember> { self.inner.inner.parent() }
    fn parent_mut(&mut self) -> Option<&mut traits::UiMember> { self.inner.inner.parent_mut() }
    fn root(&self) -> Option<&traits::UiMember> { self.inner.inner.root() }
    fn root_mut(&mut self) -> Option<&mut traits::UiMember> { self.inner.inner.root_mut() }

    #[cfg(feature = "markup")]
    default fn fill_from_markup(&mut self, _markup: &super::markup::Markup, _registry: &mut super::markup::MarkupRegistry) { unimplemented!() } 
    
    fn as_control(&self) -> &traits::UiControl { self }
    fn as_control_mut(&mut self) -> &mut traits::UiControl { self }
    fn into_control(self: Box<Self>) -> Box<traits::UiControl> { self }
}
impl <T: ControlInner> traits::UiMember for Member<Control<T>> {
    fn is_control(&self) -> Option<&traits::UiControl> { Some(self) }
    fn is_control_mut(&mut self) -> Option<&mut traits::UiControl> { Some(self) }
}
impl <T: ControlInner> Member<Control<T>> {
	pub fn control(&self) -> &Control<T> { &self.inner }
	pub fn control_mut(&mut self) -> &mut Control<T> { &mut self.inner }
}

// ===============================================================================================================

pub trait ContainerInner: MemberInner {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl>;
    
    fn gravity(&self) -> (layout::Gravity, layout::Gravity);
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity);
}
impl <T: ContainerInner> traits::UiContainer for Member<T> {
	default fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    default fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }
    
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) { self.inner.gravity() }
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity) { self.inner.set_gravity(&mut self.base, w, h) }
    
    fn as_container(&self) -> &traits::UiContainer { self }
    fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
    fn into_container(self: Box<Self>) -> Box<traits::UiContainer> { self }
}

// ===============================================================================================================

pub trait SingleContainerInner: ContainerInner {
	fn set_child(&mut self, Option<Box<traits::UiControl>>) -> Option<Box<traits::UiControl>>;
    fn child(&self) -> Option<&traits::UiControl>;
    fn child_mut(&mut self) -> Option<&mut traits::UiControl>;
}

pub struct SingleContainer<T: SingleContainerInner> {
	inner: T
}

impl <T: SingleContainerInner> MemberInner for SingleContainer<T> {
	type Id = T::Id;
	
	fn size(&self, base: &MemberBase) -> (u16, u16) { self.inner.size(base) }
    
    fn on_set_visibility(&mut self, base: &mut MemberBase) { self.inner.on_set_visibility(base) }
    
    unsafe fn native_id(&self) -> Self::Id { self.inner.native_id() }
}
impl <T: SingleContainerInner + ContainerInner> ContainerInner for SingleContainer<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }
    
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) { self.inner.gravity() }
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity) { self.inner.set_gravity(base, w, h) }
}
impl <T: SingleContainerInner + ControlInner + Drawable> Drawable for SingleContainer<T> {
	fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) { self.inner.draw(base, coords) }
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool) { self.inner.measure(base, w, h) }
    fn invalidate(&mut self, base: &mut MemberControlBase) { self.inner.invalidate(base) }
}
impl <T: SingleContainerInner + ControlInner> HasLayoutInner for SingleContainer<T> {
	fn on_layout_changed(&mut self, base: &mut layout::Attributes) { self.inner.on_layout_changed(base) }
}
impl <T: SingleContainerInner + ControlInner> ControlInner for SingleContainer<T> {
	fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &traits::UiContainer, x: i32, y: i32) { self.inner.on_added_to_container(base, parent, x, y) }
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &traits::UiContainer) { self.inner.on_removed_from_container(base, parent) }

    fn parent(&self) -> Option<&traits::UiMember> { self.inner.parent() }
    fn parent_mut(&mut self) -> Option<&mut traits::UiMember> { self.inner.parent_mut() }
    fn root(&self) -> Option<&traits::UiMember> { self.inner.root() }
    fn root_mut(&mut self) -> Option<&mut traits::UiMember> { self.inner.root_mut() }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) { self.inner.fill_from_markup(markup, registry) }
}
impl <T: SingleContainerInner> traits::UiSingleContainer for Member<SingleContainer<T>> {
	fn set_child(&mut self, child: Option<Box<traits::UiControl>>) -> Option<Box<traits::UiControl>> { self.inner.inner.set_child(child) }
    fn child(&self) -> Option<&traits::UiControl> { self.inner.inner.child() }
    fn child_mut(&mut self) -> Option<&mut traits::UiControl> { self.inner.inner.child_mut() }
    
    fn as_single_container(&self) -> &traits::UiSingleContainer { self }
    fn as_single_container_mut(&mut self) -> &mut traits::UiSingleContainer { self }
    fn into_single_container(self: Box<Self>) -> Box<traits::UiSingleContainer> { self }
}
impl <T: SingleContainerInner> traits::UiMember for Member<SingleContainer<T>> {
    fn is_container(&self) -> Option<&traits::UiContainer> { Some(self) }
    fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { Some(self) }
}
impl <T: SingleContainerInner> traits::UiContainer for Member<SingleContainer<T>> {
	fn is_single_mut(&mut self) -> Option<&mut traits::UiSingleContainer> { Some(self) }
    fn is_single(&self) -> Option<&traits::UiSingleContainer> { Some(self) }
}
impl <T: SingleContainerInner + ControlInner> traits::UiSingleContainer for Member<Control<SingleContainer<T>>> {
	fn set_child(&mut self, child: Option<Box<traits::UiControl>>) -> Option<Box<traits::UiControl>> { self.inner.inner.inner.set_child(child) }
    fn child(&self) -> Option<&traits::UiControl> { self.inner.inner.inner.child() }
    fn child_mut(&mut self) -> Option<&mut traits::UiControl> { self.inner.inner.inner.child_mut() }
    
    fn as_single_container(&self) -> &traits::UiSingleContainer { self }
    fn as_single_container_mut(&mut self) -> &mut traits::UiSingleContainer { self }
    fn into_single_container(self: Box<Self>) -> Box<traits::UiSingleContainer> { self }
}
impl <T: SingleContainerInner + ControlInner> traits::UiMember for Member<Control<SingleContainer<T>>> {
    fn is_container(&self) -> Option<&traits::UiContainer> { Some(self) }
    fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { Some(self) }
}
impl <T: SingleContainerInner + ControlInner> traits::UiContainer for Member<Control<SingleContainer<T>>> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { 
		if self.base.id == id {
			Some(self)
		} else {
			self.inner.inner.find_control_by_id_mut(id)
		} 
	}
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { 
    	if self.base.id == id {
			Some(self)
		} else {
			self.inner.inner.find_control_by_id(id) 
		}
    }
    
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) { self.inner.inner.gravity() }
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity) { self.inner.inner.set_gravity(&mut self.base, w, h) }
    
    fn is_single_mut(&mut self) -> Option<&mut traits::UiSingleContainer> { Some(self) }
    fn is_single(&self) -> Option<&traits::UiSingleContainer> { Some(self) }
    
    fn as_container(&self) -> &traits::UiContainer { self }
    fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
    fn into_container(self: Box<Self>) -> Box<traits::UiContainer> { self }
}  

// ===============================================================================================================

pub trait MultiContainerInner: ContainerInner {
	fn len(&self) -> usize;
    fn set_child_to(&mut self, index: usize, Box<traits::UiControl>) -> Option<Box<traits::UiControl>>;
    fn remove_child_from(&mut self, index: usize) -> Option<Box<traits::UiControl>>;
    fn child_at(&self, index: usize) -> Option<&traits::UiControl>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut traits::UiControl>;
}

pub struct MultiContainer<T: MultiContainerInner> {
	inner: T
}

impl <T: MultiContainerInner> MemberInner for MultiContainer<T> {
	type Id = T::Id;
	
	fn size(&self, base: &MemberBase) -> (u16, u16) { self.inner.size(base) }
    
    fn on_set_visibility(&mut self, base: &mut MemberBase) { self.inner.on_set_visibility(base) }
    
    unsafe fn native_id(&self) -> Self::Id { self.inner.native_id() }
}
impl <T: MultiContainerInner + ContainerInner> ContainerInner for MultiContainer<T> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { self.inner.find_control_by_id_mut(id) }
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { self.inner.find_control_by_id(id) }
    
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) { self.inner.gravity() }
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity) { self.inner.set_gravity(base, w, h) }
}
impl <T: MultiContainerInner + ControlInner + Drawable> Drawable for MultiContainer<T> {
	fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) { self.inner.draw(base, coords) }
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool) { self.inner.measure(base, w, h) }
    fn invalidate(&mut self, base: &mut MemberControlBase) { self.inner.invalidate(base) }
}
impl <T: MultiContainerInner + ControlInner> HasLayoutInner for MultiContainer<T> {
	fn on_layout_changed(&mut self, base: &mut layout::Attributes) { self.inner.on_layout_changed(base) }
}
impl <T: MultiContainerInner + ControlInner> ControlInner for MultiContainer<T> {
	fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &traits::UiContainer, x: i32, y: i32) { self.inner.on_added_to_container(base, parent, x, y) }
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &traits::UiContainer) { self.inner.on_removed_from_container(base, parent) }

    fn parent(&self) -> Option<&traits::UiMember> { self.inner.parent() }
    fn parent_mut(&mut self) -> Option<&mut traits::UiMember> { self.inner.parent_mut() }
    fn root(&self) -> Option<&traits::UiMember> { self.inner.root() }
    fn root_mut(&mut self) -> Option<&mut traits::UiMember> { self.inner.root_mut() }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) { self.inner.fill_from_markup(markup, registry) }
}
impl <T: MultiContainerInner> traits::UiMultiContainer for Member<MultiContainer<T>> {
	fn len(&self) -> usize { self.inner.inner.len() }
    fn set_child_to(&mut self, index: usize, child: Box<traits::UiControl>) -> Option<Box<traits::UiControl>> { self.inner.inner.set_child_to(index, child) }
    fn remove_child_from(&mut self, index: usize) -> Option<Box<traits::UiControl>> { self.inner.inner.remove_child_from(index) }
    fn child_at(&self, index: usize) -> Option<&traits::UiControl> { self.inner.inner.child_at(index) }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut traits::UiControl> { self.inner.inner.child_at_mut(index) }
    
    fn as_multi_container(&self) -> &traits::UiMultiContainer { self }
    fn as_multi_container_mut(&mut self) -> &mut traits::UiMultiContainer { self }
    fn into_multi_container(self: Box<Self>) -> Box<traits::UiMultiContainer> { self }
}
impl <T: MultiContainerInner> traits::UiMember for Member<MultiContainer<T>> {
    fn is_container(&self) -> Option<&traits::UiContainer> { Some(self) }
    fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { Some(self) }
}
impl <T: MultiContainerInner> traits::UiContainer for Member<MultiContainer<T>> {
	fn is_multi_mut(&mut self) -> Option<&mut traits::UiMultiContainer> { Some(self) }
    fn is_multi(&self) -> Option<&traits::UiMultiContainer> { Some(self) }
}
impl <T: MultiContainerInner + ControlInner> traits::UiContainer for Member<Control<MultiContainer<T>>> {
	fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiControl> { 
		if self.base.id == id {
			Some(self)
		} else {
			self.inner.inner.find_control_by_id_mut(id)
		} 
	}
    fn find_control_by_id(&self, id: ids::Id) -> Option<&traits::UiControl> { 
    	if self.base.id == id {
			Some(self)
		} else {
			self.inner.inner.find_control_by_id(id) 
		}
    }
    
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) { self.inner.inner.gravity() }
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity) { self.inner.inner.set_gravity(&mut self.base, w, h) }
    
    fn is_multi_mut(&mut self) -> Option<&mut traits::UiMultiContainer> { Some(self) }
    fn is_multi(&self) -> Option<&traits::UiMultiContainer> { Some(self) }
    
    fn as_container(&self) -> &traits::UiContainer { self }
    fn as_container_mut(&mut self) -> &mut traits::UiContainer { self }
    fn into_container(self: Box<Self>) -> Box<traits::UiContainer> { self }
}
impl <T: MultiContainerInner + ControlInner> traits::UiMember for Member<Control<MultiContainer<T>>> {
    fn is_container(&self) -> Option<&traits::UiContainer> { Some(self) }
    fn is_container_mut(&mut self) -> Option<&mut traits::UiContainer> { Some(self) }
}
impl <T: MultiContainerInner + ControlInner> traits::UiMultiContainer for Member<Control<MultiContainer<T>>> {
	fn len(&self) -> usize { self.inner.inner.inner.len() }
    fn set_child_to(&mut self, index: usize, child: Box<traits::UiControl>) -> Option<Box<traits::UiControl>> { self.inner.inner.inner.set_child_to(index, child) }
    fn remove_child_from(&mut self, index: usize) -> Option<Box<traits::UiControl>> { self.inner.inner.inner.remove_child_from(index) }
    fn child_at(&self, index: usize) -> Option<&traits::UiControl> { self.inner.inner.inner.child_at(index) }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut traits::UiControl> { self.inner.inner.inner.child_at_mut(index) }
    
    fn as_multi_container(&self) -> &traits::UiMultiContainer { self }
    fn as_multi_container_mut(&mut self) -> &mut traits::UiMultiContainer { self }
    fn into_multi_container(self: Box<Self>) -> Box<traits::UiMultiContainer> { self }
} 

// ===============================================================================================================

pub trait HasLabelInner {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn set_label(&mut self, label: &str);
}
impl <T: HasLabelInner + MemberInner> traits::UiHasLabel for Member<T> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.set_label(label) }
    
    fn as_has_label(&self) -> &traits::UiHasLabel { self }
    fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
    fn into_has_label(self: Box<Self>) -> Box<traits::UiHasLabel> { self }
}
impl <T: HasLabelInner + ControlInner> traits::UiHasLabel for Member<Control<T>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.set_label(label) }
    
    fn as_has_label(&self) -> &traits::UiHasLabel { self }
    fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
    fn into_has_label(self: Box<Self>) -> Box<traits::UiHasLabel> { self }
}
impl <T: HasLabelInner + SingleContainerInner> traits::UiHasLabel for Member<SingleContainer<T>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.set_label(label) }
    
    fn as_has_label(&self) -> &traits::UiHasLabel { self }
    fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
    fn into_has_label(self: Box<Self>) -> Box<traits::UiHasLabel> { self }
}
impl <T: HasLabelInner + MultiContainerInner> traits::UiHasLabel for Member<MultiContainer<T>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.set_label(label) }
    
    fn as_has_label(&self) -> &traits::UiHasLabel { self }
    fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
    fn into_has_label(self: Box<Self>) -> Box<traits::UiHasLabel> { self }
}
impl <T: HasLabelInner + ControlInner + SingleContainerInner> traits::UiHasLabel for Member<Control<SingleContainer<T>>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.inner.set_label(label) }
    
    fn as_has_label(&self) -> &traits::UiHasLabel { self }
    fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
    fn into_has_label(self: Box<Self>) -> Box<traits::UiHasLabel> { self }
}
impl <T: HasLabelInner + ControlInner + MultiContainerInner> traits::UiHasLabel for Member<Control<MultiContainer<T>>> {
	fn label<'a>(&'a self) -> ::std::borrow::Cow<'a, str> { self.inner.inner.inner.label() }
    fn set_label(&mut self, label: &str) { self.inner.inner.inner.set_label(label) }
    
    fn as_has_label(&self) -> &traits::UiHasLabel { self }
    fn as_has_label_mut(&mut self) -> &mut traits::UiHasLabel { self }
    fn into_has_label(self: Box<Self>) -> Box<traits::UiHasLabel> { self }
}

// ===============================================================================================================

pub trait ClickableInner {
	fn on_click(&mut self, cb: Option<callbacks::Click>);
}
impl <T: ClickableInner + MemberInner> traits::UiClickable for Member<T> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.on_click(cb) }
	
	fn as_clickable(&self) -> &traits::UiClickable { self }
    fn as_clickable_mut(&mut self) -> &mut traits::UiClickable { self }
    fn into_clickable(self: Box<Self>) -> Box<traits::UiClickable> { self }
}
impl <T: ClickableInner + ControlInner> traits::UiClickable for Member<Control<T>> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.inner.on_click(cb) }
	
	fn as_clickable(&self) -> &traits::UiClickable { self }
    fn as_clickable_mut(&mut self) -> &mut traits::UiClickable { self }
    fn into_clickable(self: Box<Self>) -> Box<traits::UiClickable> { self }
}
impl <T: ClickableInner + ControlInner + SingleContainerInner> traits::UiClickable for Member<Control<SingleContainer<T>>> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.inner.inner.on_click(cb) }
	
	fn as_clickable(&self) -> &traits::UiClickable { self }
    fn as_clickable_mut(&mut self) -> &mut traits::UiClickable { self }
    fn into_clickable(self: Box<Self>) -> Box<traits::UiClickable> { self }
}
impl <T: ClickableInner + ControlInner + MultiContainerInner> traits::UiClickable for Member<Control<MultiContainer<T>>> {
	fn on_click(&mut self, cb: Option<callbacks::Click>) {  self.inner.inner.inner.on_click(cb) }
	
	fn as_clickable(&self) -> &traits::UiClickable { self }
    fn as_clickable_mut(&mut self) -> &mut traits::UiClickable { self }
    fn into_clickable(self: Box<Self>) -> Box<traits::UiClickable> { self }
}

// ===============================================================================================================

pub trait HasOrientationInner {
	fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, layout::Orientation);
}
impl <T: HasOrientationInner + MemberInner> traits::UiHasOrientation for Member<T> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.set_layout_orientation(value) }
    
    fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
    fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
    fn into_has_orientation(self: Box<Self>) -> Box<traits::UiHasOrientation> { self }
}
impl <T: HasOrientationInner + ControlInner> traits::UiHasOrientation for Member<Control<T>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.set_layout_orientation(value) }
    
    fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
    fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
    fn into_has_orientation(self: Box<Self>) -> Box<traits::UiHasOrientation> { self }
}
impl <T: HasOrientationInner + SingleContainerInner> traits::UiHasOrientation for Member<SingleContainer<T>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.set_layout_orientation(value) }
    
    fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
    fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
    fn into_has_orientation(self: Box<Self>) -> Box<traits::UiHasOrientation> { self }
}
impl <T: HasOrientationInner + MultiContainerInner> traits::UiHasOrientation for Member<MultiContainer<T>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.set_layout_orientation(value) }
    
    fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
    fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
    fn into_has_orientation(self: Box<Self>) -> Box<traits::UiHasOrientation> { self }
}
impl <T: HasOrientationInner + SingleContainerInner + ControlInner> traits::UiHasOrientation for Member<Control<SingleContainer<T>>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.inner.set_layout_orientation(value) }
    
    fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
    fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
    fn into_has_orientation(self: Box<Self>) -> Box<traits::UiHasOrientation> { self }
}
impl <T: HasOrientationInner + MultiContainerInner + ControlInner> traits::UiHasOrientation for Member<Control<MultiContainer<T>>> {
	fn layout_orientation(&self) -> layout::Orientation { self.inner.inner.inner.layout_orientation() }
    fn set_layout_orientation(&mut self, value: layout::Orientation) { self.inner.inner.inner.set_layout_orientation(value) }
    
    fn as_has_orientation(&self) -> &traits::UiHasOrientation { self }
    fn as_has_orientation_mut(&mut self) -> &mut traits::UiHasOrientation { self }
    fn into_has_orientation(self: Box<Self>) -> Box<traits::UiHasOrientation> { self }
}

// ===============================================================================================================

pub trait ApplicationInner: Sized + 'static {
	fn with_name(name: &str) -> Box<traits::UiApplication>;
	fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<traits::UiWindow>;
    fn name<'a>(&'a self) -> ::std::borrow::Cow<'a, str>;
    fn start(&mut self);
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiMember>;
    fn find_member_by_id(&self, id: ids::Id) -> Option<&traits::UiMember>;
}
pub struct Application<T: ApplicationInner> {
	inner: T
}
impl <T: ApplicationInner> traits::UiApplication for Application<T> {
	fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<traits::UiWindow> {
		self.inner.new_window(title, size, menu)
	}
    fn name<'a>(&'a self) -> ::std::borrow::Cow<'a, str> {
	    self.inner.name()
    }
    fn start(&mut self) { self.inner.start() }
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut traits::UiMember> { self.inner.find_member_by_id_mut(id) }
    fn find_member_by_id(&self, id: ids::Id) -> Option<&traits::UiMember> { self.inner.find_member_by_id(id) }
}
impl <T: ApplicationInner> traits::AsAny for Application<T> {
    fn as_any(&self) -> &Any { self }
    fn as_any_mut(&mut self) -> &mut Any { self }
    fn into_any(self: Box<Self>) -> Box<Any> { self }
}

impl <T: ApplicationInner> HasInner for Application<T> {
	type Inner = T;
	type Params = ();
	
	fn new(inner: Self::Inner, _: Self::Params) -> Self { Application { inner } }
	fn as_inner(&self) -> &Self::Inner { &self.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner }
}
impl <T: ApplicationInner> Application<T> {
	pub fn with_name(name: &str) -> Box<traits::UiApplication> {
		T::with_name(name)
	}
}
impl <T: ApplicationInner> seal::Sealed for Application<T> {}

// ===============================================================================================================

pub trait WindowInner: HasLabelInner + SingleContainerInner {
	fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::WindowMenu) -> Box<traits::UiWindow>;
}

impl <T: WindowInner> traits::UiWindow for Member<SingleContainer<T>> {}

impl <T: WindowInner> HasInner for Member<SingleContainer<T>> {
	type Inner = T;
	type Params = MemberFunctions;
	
	fn new(inner: Self::Inner, params: Self::Params) -> Self { Member { inner: SingleContainer { inner }, base: MemberBase::with_functions(params) } }
	fn as_inner(&self) -> &Self::Inner { &self.inner.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner.inner }
}
impl <T: WindowInner> Member<SingleContainer<T>> {
	pub fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::WindowMenu) -> Box<traits::UiWindow> {
		T::with_params(title, window_size, menu)
	}
}

// ===============================================================================================================

pub trait ButtonInner: ControlInner + ClickableInner + HasLabelInner {
	fn with_label(label: &str) -> Box<traits::UiButton>;
}

impl <T: ButtonInner> traits::UiButton for Member<Control<T>> {}

impl <T: ButtonInner> HasInner for Member<Control<T>> {
	type Inner = T;
	type Params = MemberFunctions;
	
	fn new(inner: Self::Inner, params: Self::Params) -> Self { Member { inner: Control { inner, base: Default::default() }, base: MemberBase::with_functions(params) } }
	fn as_inner(&self) -> &Self::Inner { &self.inner.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner.inner }
}
impl <T: ButtonInner> Member<Control<T>> {
	pub fn with_label(label: &str) -> Box<traits::UiButton> {
		T::with_label(label)
	}
}

// ===============================================================================================================

pub trait LinearLayoutInner: ControlInner + MultiContainerInner + HasOrientationInner {
	fn with_orientation(orientation: layout::Orientation) -> Box<traits::UiLinearLayout>;
}

impl <T: LinearLayoutInner> traits::UiLinearLayout for Member<Control<MultiContainer<T>>> {}

impl <T: LinearLayoutInner> HasInner for Member<Control<MultiContainer<T>>> {
	type Inner = T;
	type Params = MemberFunctions;
	
	fn new(inner: Self::Inner, params: Self::Params) -> Self { Member { inner: Control { inner: MultiContainer { inner }, base: Default::default() }, base: MemberBase::with_functions(params) } }
	fn as_inner(&self) -> &Self::Inner { &self.inner.inner.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner.inner.inner }
}
impl <T: LinearLayoutInner> Member<Control<MultiContainer<T>>> {
	pub fn with_orientation(orientation: layout::Orientation) -> Box<traits::UiLinearLayout> {
		T::with_orientation(orientation)
	}
}

// ===============================================================================================================

pub trait FrameInner: ControlInner + SingleContainerInner + HasLabelInner {
	fn with_label(label: &str) -> Box<traits::UiFrame>;
}

impl <T: FrameInner> traits::UiFrame for Member<Control<SingleContainer<T>>> {}

impl <T: FrameInner> HasInner for Member<Control<SingleContainer<T>>> {
	type Inner = T;
	type Params = MemberFunctions;
	
	fn new(inner: Self::Inner, params: Self::Params) -> Self { Member { inner: Control { inner: SingleContainer { inner }, base: Default::default() }, base: MemberBase::with_functions(params) } }
	fn as_inner(&self) -> &Self::Inner { &self.inner.inner.inner }
	fn as_inner_mut(&mut self) -> &mut Self::Inner { &mut self.inner.inner.inner }
}
impl <T: FrameInner> Member<Control<SingleContainer<T>>> {
	pub fn with_label(label: &str) -> Box<traits::UiFrame> {
		T::with_label(label)
	}
}

// ===============================================================================================================

pub trait Final {}

pub trait OuterDrawable {
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
    fn invalidate(&mut self);
        
    fn as_drawable(&self) -> &OuterDrawable;
    fn as_drawable_mut(&mut self) -> &mut OuterDrawable;
    fn into_drawable(self: Box<Self>) -> Box<OuterDrawable>;
}

pub(crate) mod seal {
	pub trait Sealed {}
}