use super::{types, ids, layout, callbacks, controls, utils};

use std::fmt::Debug;
use std::hash::Hash;
use std::any::Any;

pub trait NativeId
    : Debug + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Into<usize> {
}

// ==========================================================================================================

pub trait HasInner: Sized + 'static {
    type Inner: Sized;
    type Params: Sized;

    fn with_inner(inner: Self::Inner, params: Self::Params) -> Self;
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
    _as_any: unsafe fn(&MemberBase) -> &Any,
    _as_any_mut: unsafe fn(&mut MemberBase) -> &mut Any,
    _as_member: unsafe fn(&MemberBase) -> &controls::Member,
    _as_member_mut: unsafe fn(&mut MemberBase) -> &mut controls::Member,
}
impl MemberFunctions {
    #[inline]
    pub fn new(_as_any: unsafe fn(&MemberBase) -> &Any, _as_any_mut: unsafe fn(&mut MemberBase) -> &mut Any, _as_member: unsafe fn(&MemberBase) -> &controls::Member, _as_member_mut: unsafe fn(&mut MemberBase) -> &mut controls::Member) -> Self {
        MemberFunctions {
            _as_any,
            _as_any_mut,
            _as_member,
            _as_member_mut,
        }
    }
}

impl MemberBase {
    #[inline]
    pub fn with_functions(functions: MemberFunctions) -> Self {
        MemberBase {
            id: ids::Id::next(),
            visibility: types::Visibility::Visible,
            functions: functions,

            handler_resize: None,
        }
    }
    #[inline]
    pub fn as_any(&self) -> &Any {
        unsafe { (self.functions._as_any)(self) }
    }
    #[inline]
    pub fn as_any_mut(&mut self) -> &mut Any {
        unsafe { (self.functions._as_any_mut)(self) }
    }
    #[inline]
    pub fn as_member(&self) -> &controls::Member {
        unsafe { (self.functions._as_member)(self) }
    }
    #[inline]
    pub fn as_member_mut(&mut self) -> &mut controls::Member {
        unsafe { (self.functions._as_member_mut)(self) }
    }
}
impl<T: MemberInner> Member<T> {
    #[inline]
    pub fn base(&self) -> &MemberBase {
        &self.base
    }
    #[inline]
    pub fn base_mut(&mut self) -> &mut MemberBase {
        &mut self.base
    }
}

pub trait MemberInner: Sized + 'static {
    type Id: NativeId + Sized;

    fn size(&self) -> (u16, u16);

    fn on_set_visibility(&mut self, base: &mut MemberBase);

    unsafe fn native_id(&self) -> Self::Id;
}
impl<T: MemberInner> controls::Member for Member<T> {
    #[inline]
    fn size(&self) -> (u16, u16) {
        self.inner.size()
    }
    #[inline]
    fn on_resize(&mut self, cb: Option<callbacks::Resize>) {
        self.base.handler_resize = cb;
    }

    #[inline]
    fn set_visibility(&mut self, visibility: types::Visibility) {
        if self.base.visibility != visibility {
            self.base.visibility = visibility;
            self.inner.on_set_visibility(&mut self.base);
        }
    }
    #[inline]
    fn visibility(&self) -> types::Visibility {
        self.base.visibility
    }

    #[inline]
    fn id(&self) -> ids::Id {
        self.base.id
    }
    #[inline]
    unsafe fn native_id(&self) -> usize {
        self.inner.native_id().into()
    }

    #[inline]
    default fn is_control(&self) -> Option<&controls::Control> {
        None
    }
    #[inline]
    default fn is_control_mut(&mut self) -> Option<&mut controls::Control> {
        None
    }
    #[inline]
    default fn is_container(&self) -> Option<&controls::Container> {
        None
    }
    #[inline]
    default fn is_container_mut(&mut self) -> Option<&mut controls::Container> {
        None
    }

    #[inline]
    fn as_member(&self) -> &controls::Member {
        self
    }
    #[inline]
    fn as_member_mut(&mut self) -> &mut controls::Member {
        self
    }
    #[inline]
    fn into_member(self: Box<Self>) -> Box<controls::Member> {
        self
    }
}
impl<T: MemberInner> controls::AsAny for Member<T> {
    #[inline]
    fn as_any(&self) -> &Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
    #[inline]
    fn into_any(self: Box<Self>) -> Box<Any> {
        self
    }
}
impl<T: MemberInner> HasInner for Member<T> {
    type Inner = T;
    type Params = MemberFunctions;

    #[inline]
    fn with_inner(inner: Self::Inner, params: Self::Params) -> Self {
        Member {
            inner: inner,
            base: MemberBase::with_functions(params),
        }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
impl<T: MemberInner> seal::Sealed for Member<T> {}

// ===============================================================================================================

pub trait HasLayoutInner: MemberInner {
    fn on_layout_changed(&mut self, base: &mut MemberBase);
}

impl<T: ControlInner> controls::HasLayout for Member<Control<T>> {
    #[inline]
    fn layout_width(&self) -> layout::Size {
        self.inner.base.layout.width
    }
    #[inline]
    fn layout_height(&self) -> layout::Size {
        self.inner.base.layout.height
    }
    #[inline]
    fn layout_alignment(&self) -> layout::Alignment {
        self.inner.base.layout.alignment
    }
    #[inline]
    fn layout_padding(&self) -> layout::BoundarySize {
        self.inner.base.layout.padding
    }
    #[inline]
    fn layout_margin(&self) -> layout::BoundarySize {
        self.inner.base.layout.margin
    }

    #[inline]
    fn set_layout_width(&mut self, value: layout::Size) {
        self.inner.base.layout.width = value;
        self.inner.inner.on_layout_changed(&mut self.base);
    }
    #[inline]
    fn set_layout_height(&mut self, value: layout::Size) {
        self.inner.base.layout.height = value;
        self.inner.inner.on_layout_changed(&mut self.base);
    }
    #[inline]
    fn set_layout_alignment(&mut self, value: layout::Alignment) {
        self.inner.base.layout.alignment = value;
        self.inner.inner.on_layout_changed(&mut self.base);
    }
    #[inline]
    fn set_layout_padding(&mut self, value: layout::BoundarySizeArgs) {
        self.inner.base.layout.padding = value.into();
        self.inner.inner.on_layout_changed(&mut self.base);
    }
    #[inline]
    fn set_layout_margin(&mut self, value: layout::BoundarySizeArgs) {
        self.inner.base.layout.margin = value.into();
        self.inner.inner.on_layout_changed(&mut self.base);
    }

    #[inline]
    fn as_has_layout(&self) -> &controls::HasLayout {
        self
    }
    #[inline]
    fn as_has_layout_mut(&mut self) -> &mut controls::HasLayout {
        self
    }
    #[inline]
    fn into_has_layout(self: Box<Self>) -> Box<controls::HasLayout> {
        self
    }
}

// ===============================================================================================================

pub trait Drawable: Sized + 'static {
    fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>);
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool);
    fn invalidate(&mut self, base: &mut MemberControlBase);
}

// ===============================================================================================================

pub trait ControlInner: HasLayoutInner + Drawable {
    fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32);
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container);

    fn parent(&self) -> Option<&controls::Member>;
    fn parent_mut(&mut self) -> Option<&mut controls::Member>;
    fn root(&self) -> Option<&controls::Member>;
    fn root_mut(&mut self) -> Option<&mut controls::Member>;

    fn cast_base_mut<'this, 'a: 'this>(&'this self, base: &'a mut MemberBase) -> &'a mut MemberControlBase {
        unsafe { utils::member_control_base_mut_unchecked(base) }
    }
    fn cast_base<'this, 'a: 'this>(&'this self, base: &'a MemberBase) -> &'a MemberControlBase {
        unsafe { utils::member_control_base_unchecked(base) }
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut MemberControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry);
}

#[repr(C)]
pub struct MemberControlBase {
    pub member: MemberBase,
    pub control: ControlBase,
}

#[repr(C)]
pub struct ControlBase {
    pub layout: layout::Attributes,
    pub skip_draw: bool,
}
#[repr(C)]
pub struct Control<T: ControlInner> {
    base: ControlBase,
    inner: T,
}

impl Default for ControlBase {
    fn default() -> Self {
        ControlBase {
            layout: layout::Attributes::default(),
            skip_draw: false,
        }
    }
}
impl<T: ControlInner> Control<T> {
    #[inline]
    pub fn base(&self) -> &ControlBase {
        &self.base
    }
    #[inline]
    pub fn base_mut(&mut self) -> &mut ControlBase {
        &mut self.base
    }
}

impl<T: ControlInner> MemberInner for Control<T> {
    type Id = T::Id;

    #[inline]
    fn size(&self) -> (u16, u16) {
        self.inner.size()
    }
    #[inline]
    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.inner.on_set_visibility(base)
    }
    #[inline]
    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: ControlInner> HasInner for Control<T> {
    type Inner = T;
    type Params = ();

    #[inline]
    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        Control {
            inner: inner,
            base: Default::default(),
        }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
impl<T: ControlInner> HasLayoutInner for Control<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
}
impl<T: ControlInner> OuterDrawable for Member<Control<T>> {
    #[inline]
    fn draw(&mut self, coords: Option<(i32, i32)>) {
        if !self.is_skip_draw() {
            self.inner
                .inner
                .draw(unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
                      coords)
        }
    }
    #[inline]
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner
            .inner
            .measure(unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
                     w,
                     h)
    }
    #[inline]
    fn invalidate(&mut self) {
        self.inner
            .inner
            .invalidate(unsafe { utils::member_control_base_mut_unchecked(&mut self.base) })
    }
    #[inline]
    fn set_skip_draw(&mut self, skip: bool) {
        self.inner.base.skip_draw = skip;
    }
    #[inline]
    fn is_skip_draw(&self) -> bool {
        self.inner.base.skip_draw
    }
    #[inline]
    fn as_drawable(&self) -> &OuterDrawable {
        self
    }
    #[inline]
    fn as_drawable_mut(&mut self) -> &mut OuterDrawable {
        self
    }
    #[inline]
    fn into_drawable(self: Box<Self>) -> Box<OuterDrawable> {
        self
    }
}
impl<T: ControlInner> controls::Control for Member<Control<T>> {
    #[inline]
    fn on_added_to_container(&mut self, parent: &controls::Container, x: i32, y: i32) {
        self.inner
            .inner
            .on_added_to_container(unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
                                   parent,
                                   x,
                                   y)
    }
    #[inline]
    fn on_removed_from_container(&mut self, parent: &controls::Container) {
        self.inner
            .inner
            .on_removed_from_container(unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
                                       parent)
    }

    #[inline]
    fn parent(&self) -> Option<&controls::Member> {
        self.inner.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.inner.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&controls::Member> {
        self.inner.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.inner.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    default fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        self.inner
            .inner
            .fill_from_markup(unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
                              markup,
                              registry)
    }

    #[inline]
    fn as_control(&self) -> &controls::Control {
        self
    }
    #[inline]
    fn as_control_mut(&mut self) -> &mut controls::Control {
        self
    }
    #[inline]
    fn into_control(self: Box<Self>) -> Box<controls::Control> {
        self
    }
}
impl<T: ControlInner> controls::Member for Member<Control<T>> {
    #[inline]
    fn is_control(&self) -> Option<&controls::Control> {
        Some(self)
    }
    #[inline]
    fn is_control_mut(&mut self) -> Option<&mut controls::Control> {
        Some(self)
    }
}
impl<T: ControlInner> Member<Control<T>> {
    #[inline]
    pub fn control(&self) -> &Control<T> {
        &self.inner
    }
    #[inline]
    pub fn control_mut(&mut self) -> &mut Control<T> {
        &mut self.inner
    }
}

// ===============================================================================================================

pub trait ContainerInner: MemberInner {
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control>;
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control>;

    fn gravity(&self) -> (layout::Gravity, layout::Gravity);
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity);
}
impl<T: ContainerInner> controls::Container for Member<T> {
    #[inline]
    default fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
        self.inner.find_control_by_id_mut(id)
    }
    #[inline]
    default fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
        self.inner.find_control_by_id(id)
    }

    #[inline]
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
        self.inner.gravity()
    }
    #[inline]
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity) {
        self.inner.set_gravity(&mut self.base, w, h)
    }

    #[inline]
    fn as_container(&self) -> &controls::Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut controls::Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<controls::Container> {
        self
    }
}

// ===============================================================================================================

pub trait SingleContainerInner: ContainerInner {
    fn set_child(&mut self, base: &mut MemberBase, Option<Box<controls::Control>>) -> Option<Box<controls::Control>>;
    fn child(&self) -> Option<&controls::Control>;
    fn child_mut(&mut self) -> Option<&mut controls::Control>;
}

pub struct SingleContainer<T: SingleContainerInner> {
    inner: T,
}

impl<T: SingleContainerInner> MemberInner for SingleContainer<T> {
    type Id = T::Id;

    #[inline]
    fn size(&self) -> (u16, u16) {
        self.inner.size()
    }
    #[inline]
    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.inner.on_set_visibility(base)
    }
    #[inline]
    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: SingleContainerInner> HasInner for SingleContainer<T> {
    type Inner = T;
    type Params = ();

    #[inline]
    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        SingleContainer { inner: inner }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
impl<T: SingleContainerInner + ContainerInner> ContainerInner for SingleContainer<T> {
    #[inline]
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
        self.inner.find_control_by_id_mut(id)
    }
    #[inline]
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
        self.inner.find_control_by_id(id)
    }

    #[inline]
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
        self.inner.gravity()
    }
    #[inline]
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity) {
        self.inner.set_gravity(base, w, h)
    }
}
impl<T: SingleContainerInner + ControlInner + Drawable> Drawable for SingleContainer<T> {
    #[inline]
    fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
        self.inner.draw(base, coords)
    }
    #[inline]
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(base, w, h)
    }
    #[inline]
    fn invalidate(&mut self, base: &mut MemberControlBase) {
        self.inner.invalidate(base)
    }
}
impl<T: SingleContainerInner + ControlInner> HasLayoutInner for SingleContainer<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
}
impl<T: SingleContainerInner + ControlInner> ControlInner for SingleContainer<T> {
    #[inline]
    fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
        self.inner.on_added_to_container(base, parent, x, y)
    }
    #[inline]
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container) {
        self.inner.on_removed_from_container(base, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&controls::Member> {
        self.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&controls::Member> {
        self.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut MemberControlBase, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        self.inner.fill_from_markup(base, markup, registry)
    }
}
impl<T: SingleContainerInner> controls::SingleContainer for Member<SingleContainer<T>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<controls::Control>>) -> Option<Box<controls::Control>> {
        self.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&controls::Control> {
        self.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut controls::Control> {
        self.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &controls::SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut controls::SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<controls::SingleContainer> {
        self
    }
}
impl<T: SingleContainerInner> controls::Member for Member<SingleContainer<T>> {
    #[inline]
    fn is_container(&self) -> Option<&controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut controls::Container> {
        Some(self)
    }
}
impl<T: SingleContainerInner> controls::Container for Member<SingleContainer<T>> {
    #[inline]
    fn is_single_mut(&mut self) -> Option<&mut controls::SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single(&self) -> Option<&controls::SingleContainer> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> controls::SingleContainer for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<controls::Control>>) -> Option<Box<controls::Control>> {
        self.inner.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&controls::Control> {
        self.inner.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut controls::Control> {
        self.inner.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &controls::SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut controls::SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<controls::SingleContainer> {
        self
    }
}
impl<T: SingleContainerInner + ControlInner> controls::Member for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut controls::Container> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> controls::Container for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
        if self.base.id == id {
            Some(self)
        } else {
            self.inner.inner.find_control_by_id_mut(id)
        }
    }
    #[inline]
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
        if self.base.id == id {
            Some(self)
        } else {
            self.inner.inner.find_control_by_id(id)
        }
    }

    #[inline]
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
        self.inner.inner.gravity()
    }
    #[inline]
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity) {
        self.inner.inner.set_gravity(&mut self.base, w, h)
    }

    #[inline]
    fn is_single_mut(&mut self) -> Option<&mut controls::SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single(&self) -> Option<&controls::SingleContainer> {
        Some(self)
    }

    #[inline]
    fn as_container(&self) -> &controls::Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut controls::Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<controls::Container> {
        self
    }
}

// ===============================================================================================================

pub trait MultiContainerInner: ContainerInner {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, Box<controls::Control>) -> Option<Box<controls::Control>>;
    fn remove_child_from(&mut self, base: &mut MemberBase, index: usize) -> Option<Box<controls::Control>>;
    fn child_at(&self, index: usize) -> Option<&controls::Control>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut controls::Control>;

    #[inline]
    fn is_empty(&self) -> bool {
        self.len() < 1
    }
    #[inline]
    fn clear(&mut self, base: &mut MemberBase) {
        let len = self.len();
        for index in (0..len).rev() {
            self.remove_child_from(base, index);
        }
    }
    #[inline]
    fn push_child(&mut self, base: &mut MemberBase, child: Box<controls::Control>) {
        let len = self.len();
        self.set_child_to(base, len, child);
    }
    #[inline]
    fn pop_child(&mut self, base: &mut MemberBase) -> Option<Box<controls::Control>> {
        let len = self.len();
        if len > 0 {
            self.remove_child_from(base, len - 1)
        } else {
            None
        }
    }
}

pub struct MultiContainer<T: MultiContainerInner> {
    inner: T,
}

impl<T: MultiContainerInner> MemberInner for MultiContainer<T> {
    type Id = T::Id;

    fn size(&self) -> (u16, u16) {
        self.inner.size()
    }

    fn on_set_visibility(&mut self, base: &mut MemberBase) {
        self.inner.on_set_visibility(base)
    }

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: MultiContainerInner> HasInner for MultiContainer<T> {
    type Inner = T;
    type Params = ();

    #[inline]
    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        MultiContainer { inner: inner }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
impl<T: MultiContainerInner + ContainerInner> ContainerInner for MultiContainer<T> {
    #[inline]
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
        self.inner.find_control_by_id_mut(id)
    }
    #[inline]
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
        self.inner.find_control_by_id(id)
    }

    #[inline]
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
        self.inner.gravity()
    }
    #[inline]
    fn set_gravity(&mut self, base: &mut MemberBase, w: layout::Gravity, h: layout::Gravity) {
        self.inner.set_gravity(base, w, h)
    }
}
impl<T: MultiContainerInner + ControlInner + Drawable> Drawable for MultiContainer<T> {
    #[inline]
    fn draw(&mut self, base: &mut MemberControlBase, coords: Option<(i32, i32)>) {
        self.inner.draw(base, coords)
    }
    #[inline]
    fn measure(&mut self, base: &mut MemberControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(base, w, h)
    }
    #[inline]
    fn invalidate(&mut self, base: &mut MemberControlBase) {
        self.inner.invalidate(base)
    }
}
impl<T: MultiContainerInner + ControlInner> HasLayoutInner for MultiContainer<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
}
impl<T: MultiContainerInner + ControlInner> ControlInner for MultiContainer<T> {
    #[inline]
    fn on_added_to_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container, x: i32, y: i32) {
        self.inner.on_added_to_container(base, parent, x, y)
    }
    #[inline]
    fn on_removed_from_container(&mut self, base: &mut MemberControlBase, parent: &controls::Container) {
        self.inner.on_removed_from_container(base, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&controls::Member> {
        self.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut controls::Member> {
        self.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&controls::Member> {
        self.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut controls::Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, base: &mut MemberControlBase, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        self.inner.fill_from_markup(base, markup, registry)
    }
}
impl<T: MultiContainerInner> controls::MultiContainer for Member<MultiContainer<T>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<controls::Control>) -> Option<Box<controls::Control>> {
        self.inner
            .inner
            .set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<controls::Control>> {
        self.inner
            .inner
            .remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&controls::Control> {
        self.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut controls::Control> {
        self.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &controls::MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut controls::MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<controls::MultiContainer> {
        self
    }
}
impl<T: MultiContainerInner> controls::Member for Member<MultiContainer<T>> {
    #[inline]
    fn is_container(&self) -> Option<&controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut controls::Container> {
        Some(self)
    }
}
impl<T: MultiContainerInner> controls::Container for Member<MultiContainer<T>> {
    #[inline]
    fn is_multi_mut(&mut self) -> Option<&mut controls::MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi(&self) -> Option<&controls::MultiContainer> {
        Some(self)
    }
}
impl<T: MultiContainerInner + ControlInner> controls::Container for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn find_control_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Control> {
        if self.base.id == id {
            Some(self)
        } else {
            self.inner.inner.find_control_by_id_mut(id)
        }
    }
    #[inline]
    fn find_control_by_id(&self, id: ids::Id) -> Option<&controls::Control> {
        if self.base.id == id {
            Some(self)
        } else {
            self.inner.inner.find_control_by_id(id)
        }
    }

    #[inline]
    fn gravity(&self) -> (layout::Gravity, layout::Gravity) {
        self.inner.inner.gravity()
    }
    #[inline]
    fn set_gravity(&mut self, w: layout::Gravity, h: layout::Gravity) {
        self.inner.inner.set_gravity(&mut self.base, w, h)
    }

    #[inline]
    fn is_multi_mut(&mut self) -> Option<&mut controls::MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi(&self) -> Option<&controls::MultiContainer> {
        Some(self)
    }

    #[inline]
    fn as_container(&self) -> &controls::Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut controls::Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<controls::Container> {
        self
    }
}
impl<T: MultiContainerInner + ControlInner> controls::Member for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut controls::Container> {
        Some(self)
    }
}
impl<T: MultiContainerInner + ControlInner> controls::MultiContainer for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<controls::Control>) -> Option<Box<controls::Control>> {
        self.inner
            .inner
            .inner
            .set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<controls::Control>> {
        self.inner
            .inner
            .inner
            .remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&controls::Control> {
        self.inner.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut controls::Control> {
        self.inner.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &controls::MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut controls::MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<controls::MultiContainer> {
        self
    }
}

// ===============================================================================================================

pub trait HasLabelInner {
    fn label(&self) -> ::std::borrow::Cow<str>;
    fn set_label(&mut self, base: &mut MemberBase, label: &str);
}
impl<T: HasLabelInner + MemberInner> controls::HasLabel for Member<T> {
    #[inline]
    fn label(&self) -> ::std::borrow::Cow<str> {
        self.inner.label()
    }
    #[inline]
    fn set_label(&mut self, label: &str) {
        self.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + ControlInner> controls::HasLabel for Member<Control<T>> {
    #[inline]
    fn label(&self) -> ::std::borrow::Cow<str> {
        self.inner.inner.label()
    }
    #[inline]
    fn set_label(&mut self, label: &str) {
        self.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + SingleContainerInner> controls::HasLabel for Member<SingleContainer<T>> {
    #[inline]
    fn label(&self) -> ::std::borrow::Cow<str> {
        self.inner.inner.label()
    }
    #[inline]
    fn set_label(&mut self, label: &str) {
        self.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + MultiContainerInner> controls::HasLabel for Member<MultiContainer<T>> {
    #[inline]
    fn label(&self) -> ::std::borrow::Cow<str> {
        self.inner.inner.label()
    }
    #[inline]
    fn set_label(&mut self, label: &str) {
        self.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + ControlInner + SingleContainerInner> controls::HasLabel for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn label(&self) -> ::std::borrow::Cow<str> {
        self.inner.inner.inner.label()
    }
    #[inline]
    fn set_label(&mut self, label: &str) {
        self.inner.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + ControlInner + MultiContainerInner> controls::HasLabel for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn label(&self) -> ::std::borrow::Cow<str> {
        self.inner.inner.inner.label()
    }
    #[inline]
    fn set_label(&mut self, label: &str) {
        self.inner.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<controls::HasLabel> {
        self
    }
}

// ===============================================================================================================

pub trait ClickableInner {
    fn on_click(&mut self, cb: Option<callbacks::Click>);
}
impl<T: ClickableInner + MemberInner> controls::Clickable for Member<T> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::Click>) {
        self.inner.on_click(cb)
    }

    #[inline]
    fn as_clickable(&self) -> &controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<controls::Clickable> {
        self
    }
}
impl<T: ClickableInner + ControlInner> controls::Clickable for Member<Control<T>> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::Click>) {
        self.inner.inner.on_click(cb)
    }

    #[inline]
    fn as_clickable(&self) -> &controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<controls::Clickable> {
        self
    }
}
impl<T: ClickableInner + ControlInner + SingleContainerInner> controls::Clickable for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::Click>) {
        self.inner.inner.inner.on_click(cb)
    }

    #[inline]
    fn as_clickable(&self) -> &controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<controls::Clickable> {
        self
    }
}
impl<T: ClickableInner + ControlInner + MultiContainerInner> controls::Clickable for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::Click>) {
        self.inner.inner.inner.on_click(cb)
    }

    #[inline]
    fn as_clickable(&self) -> &controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<controls::Clickable> {
        self
    }
}

// ===============================================================================================================

pub trait HasOrientationInner {
    fn layout_orientation(&self) -> layout::Orientation;
    fn set_layout_orientation(&mut self, base: &mut MemberBase, orientation: layout::Orientation);
}
impl<T: HasOrientationInner + MemberInner> controls::HasOrientation for Member<T> {
    #[inline]
    fn layout_orientation(&self) -> layout::Orientation {
        self.inner.layout_orientation()
    }
    #[inline]
    fn set_layout_orientation(&mut self, value: layout::Orientation) {
        self.inner.set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<controls::HasOrientation> {
        self
    }
}
impl<T: HasOrientationInner + ControlInner> controls::HasOrientation for Member<Control<T>> {
    #[inline]
    fn layout_orientation(&self) -> layout::Orientation {
        self.inner.inner.layout_orientation()
    }
    #[inline]
    fn set_layout_orientation(&mut self, value: layout::Orientation) {
        self.inner
            .inner
            .set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<controls::HasOrientation> {
        self
    }
}
impl<T: HasOrientationInner + SingleContainerInner> controls::HasOrientation for Member<SingleContainer<T>> {
    #[inline]
    fn layout_orientation(&self) -> layout::Orientation {
        self.inner.inner.layout_orientation()
    }
    #[inline]
    fn set_layout_orientation(&mut self, value: layout::Orientation) {
        self.inner
            .inner
            .set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<controls::HasOrientation> {
        self
    }
}
impl<T: HasOrientationInner + MultiContainerInner> controls::HasOrientation for Member<MultiContainer<T>> {
    #[inline]
    fn layout_orientation(&self) -> layout::Orientation {
        self.inner.inner.layout_orientation()
    }
    #[inline]
    fn set_layout_orientation(&mut self, value: layout::Orientation) {
        self.inner
            .inner
            .set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<controls::HasOrientation> {
        self
    }
}
impl<T: HasOrientationInner + SingleContainerInner + ControlInner> controls::HasOrientation for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn layout_orientation(&self) -> layout::Orientation {
        self.inner.inner.inner.layout_orientation()
    }
    #[inline]
    fn set_layout_orientation(&mut self, value: layout::Orientation) {
        self.inner
            .inner
            .inner
            .set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<controls::HasOrientation> {
        self
    }
}
impl<T: HasOrientationInner + MultiContainerInner + ControlInner> controls::HasOrientation for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn layout_orientation(&self) -> layout::Orientation {
        self.inner.inner.inner.layout_orientation()
    }
    #[inline]
    fn set_layout_orientation(&mut self, value: layout::Orientation) {
        self.inner
            .inner
            .inner
            .set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<controls::HasOrientation> {
        self
    }
}

// ===============================================================================================================

pub trait ApplicationInner: Sized + 'static {
    fn with_name(name: &str) -> Box<controls::Application>;
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window>;
    fn name(&self) -> ::std::borrow::Cow<str>;
    fn start(&mut self);
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Member>;
    fn find_member_by_id(&self, id: ids::Id) -> Option<&controls::Member>;
}
pub struct Application<T: ApplicationInner> {
    inner: T,
}
impl<T: ApplicationInner> controls::Application for Application<T> {
    #[inline]
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window> {
        self.inner.new_window(title, size, menu)
    }
    #[inline]
    fn name(&self) -> ::std::borrow::Cow<str> {
        self.inner.name()
    }
    #[inline]
    fn start(&mut self) {
        self.inner.start()
    }
    #[inline]
    fn find_member_by_id_mut(&mut self, id: ids::Id) -> Option<&mut controls::Member> {
        self.inner.find_member_by_id_mut(id)
    }
    #[inline]
    fn find_member_by_id(&self, id: ids::Id) -> Option<&controls::Member> {
        self.inner.find_member_by_id(id)
    }
}
impl<T: ApplicationInner> controls::AsAny for Application<T> {
    #[inline]
    fn as_any(&self) -> &Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut Any {
        self
    }
    #[inline]
    fn into_any(self: Box<Self>) -> Box<Any> {
        self
    }
}

impl<T: ApplicationInner> HasInner for Application<T> {
    type Inner = T;
    type Params = ();

    #[inline]
    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        Application { inner }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}
impl<T: ApplicationInner> Application<T> {
    #[inline]
    pub fn with_name(name: &str) -> Box<controls::Application> {
        T::with_name(name)
    }
}
impl<T: ApplicationInner> seal::Sealed for Application<T> {}

// ===============================================================================================================

pub trait WindowInner: HasLabelInner + SingleContainerInner {
    fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window>;
}

impl<T: WindowInner> controls::Window for Member<SingleContainer<T>> {}

impl<T: WindowInner> Member<SingleContainer<T>> {
    #[inline]
    pub fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::WindowMenu) -> Box<controls::Window> {
        T::with_params(title, window_size, menu)
    }
}

// ===============================================================================================================

pub trait ButtonInner: ControlInner + ClickableInner + HasLabelInner {
    fn with_label(label: &str) -> Box<controls::Button>;
}

impl<T: ButtonInner> controls::Button for Member<Control<T>> {}

impl<T: ButtonInner> Member<Control<T>> {
    #[inline]
    pub fn with_label(label: &str) -> Box<controls::Button> {
        T::with_label(label)
    }
}

// ===============================================================================================================

pub trait LinearLayoutInner
    : ControlInner + MultiContainerInner + HasOrientationInner {
    fn with_orientation(orientation: layout::Orientation) -> Box<controls::LinearLayout>;
}

impl<T: LinearLayoutInner> controls::LinearLayout for Member<Control<MultiContainer<T>>> {}

impl<T: LinearLayoutInner> Member<Control<MultiContainer<T>>> {
    #[inline]
    pub fn with_orientation(orientation: layout::Orientation) -> Box<controls::LinearLayout> {
        T::with_orientation(orientation)
    }
}

// ===============================================================================================================

pub trait FrameInner: ControlInner + SingleContainerInner + HasLabelInner {
    fn with_label(label: &str) -> Box<controls::Frame>;
}

impl<T: FrameInner> controls::Frame for Member<Control<SingleContainer<T>>> {}

impl<T: FrameInner> Member<Control<SingleContainer<T>>> {
    #[inline]
    pub fn with_label(label: &str) -> Box<controls::Frame> {
        T::with_label(label)
    }
}

// ===============================================================================================================

pub trait SplittedInner
    : MultiContainerInner + ControlInner + HasOrientationInner {
    fn with_content(first: Box<controls::Control>, second: Box<controls::Control>, orientation: layout::Orientation) -> Box<controls::Splitted>;
    fn set_splitter(&mut self, base: &mut MemberControlBase, pos: f32);
    fn splitter(&self) -> f32;

    fn first(&self) -> &controls::Control;
    fn second(&self) -> &controls::Control;
    fn first_mut(&mut self) -> &mut controls::Control;
    fn second_mut(&mut self) -> &mut controls::Control;
}

impl<T: SplittedInner> controls::Splitted for Member<Control<MultiContainer<T>>> {
    fn set_splitter(&mut self, pos: f32) {
        self.inner
            .inner
            .inner
            .set_splitter(unsafe { utils::member_control_base_mut_unchecked(&mut self.base) },
                          pos)
    }
    fn splitter(&self) -> f32 {
        self.inner.inner.inner.splitter()
    }
    fn first(&self) -> &controls::Control {
        self.inner.inner.inner.first()
    }
    fn second(&self) -> &controls::Control {
        self.inner.inner.inner.second()
    }
    fn first_mut(&mut self) -> &mut controls::Control {
        self.inner.inner.inner.first_mut()
    }
    fn second_mut(&mut self) -> &mut controls::Control {
        self.inner.inner.inner.second_mut()
    }
}

impl<T: SplittedInner> Member<Control<MultiContainer<T>>> {
    #[inline]
    pub fn with_content(first: Box<controls::Control>, second: Box<controls::Control>, orientation: layout::Orientation) -> Box<controls::Splitted> {
        T::with_content(first, second, orientation)
    }
}

// ===============================================================================================================

pub trait Final {}

pub trait OuterDrawable {
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
    fn invalidate(&mut self);
    fn set_skip_draw(&mut self, skip: bool);
    fn is_skip_draw(&self) -> bool;

    fn as_drawable(&self) -> &OuterDrawable;
    fn as_drawable_mut(&mut self) -> &mut OuterDrawable;
    fn into_drawable(self: Box<Self>) -> Box<OuterDrawable>;
}

pub(crate) mod seal {
    pub trait Sealed {}
}
