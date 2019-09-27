pub use crate::auto::{ClickableInner, CloseableInner, HasImageInner, HasLabelInner, HasProgressInner, HasSizeInner, HasVisibilityInner};

use crate::{callbacks, controls, ids, layout, runtime, types, defaults};

use std::any::Any;
use std::borrow::Cow;
use std::cell::UnsafeCell;
use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::mpsc;
use std::cmp;

#[cfg(feature = "type_check")]
use std::any::TypeId;

pub trait NativeId: Any + Debug + Clone + PartialEq + Eq + PartialOrd + Ord + Hash + Into<usize> + Sized {}

// ===============================================================================================================

pub trait HasNativeIdInner: 'static {
    type Id: NativeId;

    unsafe fn native_id(&self) -> Self::Id;
}

// ==========================================================================================================

pub trait HasBase: Sized + 'static {
    type Base: Sized;

    fn base(&self) -> &Self::Base;
    fn base_mut(&mut self) -> &mut Self::Base;
}

pub trait HasInner: Sized + 'static {
    type Inner: Sized;
    type Params: Sized;

    fn with_inner(inner: Self::Inner, params: Self::Params) -> Self;
    fn as_inner(&self) -> &Self::Inner;
    fn as_inner_mut(&mut self) -> &mut Self::Inner;
    fn into_inner(self) -> Self::Inner;
}

#[repr(C)]
pub struct MemberBase {
    id: ids::Id,
    functions: MemberFunctions,
    app: usize,
    tag: Option<String>,

    _no_threads: PhantomData<Rc<()>>,
}
#[repr(C)]
pub struct Member<T: MemberInner> {
    base: MemberBase,
    inner: T,
}
#[repr(C)]
pub struct MemberFunctions {
    _as_any: unsafe fn(&MemberBase) -> &dyn Any,
    _as_any_mut: unsafe fn(&mut MemberBase) -> &mut dyn Any,
    _as_member: unsafe fn(&MemberBase) -> &dyn controls::Member,
    _as_member_mut: unsafe fn(&mut MemberBase) -> &mut dyn controls::Member,
}
impl MemberFunctions {
    #[inline]
    pub fn new(
        _as_any: unsafe fn(&MemberBase) -> &dyn Any,
        _as_any_mut: unsafe fn(&mut MemberBase) -> &mut dyn Any,
        _as_member: unsafe fn(&MemberBase) -> &dyn controls::Member,
        _as_member_mut: unsafe fn(&mut MemberBase) -> &mut dyn controls::Member,
    ) -> Self {
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
            functions: functions,
            app: runtime::APPLICATION.with(|a| *a.borrow()),
            tag: None,
            _no_threads: PhantomData,
        }
    }
    pub fn id(&self) -> ids::Id {
        self.id
    }
    pub fn tag(&self) -> Option<Cow<str>> {
        self.tag.as_ref().map(|t| t.as_str().into())
    }
    pub fn set_tag(&mut self, tag: Option<Cow<str>>) {
        self.tag = tag.map(|t| t.into());
    }
    #[inline]
    pub fn as_any(&self) -> &dyn Any {
        unsafe { (self.functions._as_any)(self) }
    }
    #[inline]
    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        unsafe { (self.functions._as_any_mut)(self) }
    }
    #[inline]
    pub fn as_member(&self) -> &dyn controls::Member {
        unsafe { (self.functions._as_member)(self) }
    }
    #[inline]
    pub fn as_member_mut(&mut self) -> &mut dyn controls::Member {
        unsafe { (self.functions._as_member_mut)(self) }
    }
}
impl<T: MemberInner> HasBase for Member<T> {
    type Base = MemberBase;

    #[inline]
    fn base(&self) -> &Self::Base {
        &self.base
    }
    #[inline]
    fn base_mut(&mut self) -> &mut Self::Base {
        &mut self.base
    }
}

pub trait MemberInner: HasNativeIdInner + Sized + 'static {}

impl<T: MemberInner> controls::HasNativeId for Member<T> {
    #[inline]
    unsafe fn native_id(&self) -> usize {
        self.inner.native_id().into()
    }
}
impl<T: MemberInner> controls::MaybeControl for Member<T> {
    #[inline]
    default fn is_control(&self) -> Option<&dyn controls::Control> {
        None
    }
    #[inline]
    default fn is_control_mut(&mut self) -> Option<&mut dyn controls::Control> {
        None
    }
}
impl<T: MemberInner> controls::MaybeContainer for Member<T> {
    #[inline]
    default fn is_container(&self) -> Option<&dyn controls::Container> {
        None
    }
    #[inline]
    default fn is_container_mut(&mut self) -> Option<&mut dyn controls::Container> {
        None
    }
}
impl<T: MemberInner> controls::MaybeHasSize for Member<T> {}
impl<T: MemberInner> controls::MaybeHasVisibility for Member<T> {}

impl<T: MemberInner> controls::Member for Member<T> {
    #[inline]
    fn id(&self) -> ids::Id {
        self.base.id
    }
    fn tag(&self) -> Option<Cow<str>> {
        self.base.tag()
    }
    fn set_tag(&mut self, tag: Option<Cow<str>>) {
        self.base.set_tag(tag)
    }
    #[cfg(feature = "type_check")]
    unsafe fn type_id(&self) -> TypeId {
        self.inner.native_id().type_id()
    }

    #[inline]
    fn as_member(&self) -> &dyn controls::Member {
        self
    }
    #[inline]
    fn as_member_mut(&mut self) -> &mut dyn controls::Member {
        self
    }
    #[inline]
    fn into_member(self: Box<Self>) -> Box<dyn controls::Member> {
        self
    }
}
impl<T: MemberInner> controls::AsAny for Member<T> {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
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
    #[inline]
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<T: MemberInner> seal::Sealed for Member<T> {}

// ===============================================================================================================

pub trait HasLayoutInner: MemberInner {
    fn on_layout_changed(&mut self, base: &mut MemberBase);

    fn layout_margin(&self, _member: &MemberBase) -> layout::BoundarySize {
        layout::BoundarySize::AllTheSame(0)
    }
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
    fn layout_margin(&self) -> layout::BoundarySize {
        self.inner.inner.layout_margin(&self.base)
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
    fn as_has_layout(&self) -> &dyn controls::HasLayout {
        self
    }
    #[inline]
    fn as_has_layout_mut(&mut self) -> &mut dyn controls::HasLayout {
        self
    }
    #[inline]
    fn into_has_layout(self: Box<Self>) -> Box<dyn controls::HasLayout> {
        self
    }
}

// ===============================================================================================================

pub trait Drawable: Sized + 'static {
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase);
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase);
    fn measure(&mut self, _member: &mut MemberBase, control: &mut ControlBase, parent_width: u16, parent_height: u16) -> (u16, u16, bool) {
        let old_size = control.measured;
        control.measured = match control.visibility {
            types::Visibility::Gone => (0, 0),
            _ => {
                let w = match control.layout.width {
                    layout::Size::MatchParent => parent_width,
                    layout::Size::Exact(w) => w,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                let h = match control.layout.height {
                    layout::Size::MatchParent => parent_height,
                    layout::Size::Exact(h) => h,
                    layout::Size::WrapContent => defaults::THE_ULTIMATE_ANSWER_TO_EVERYTHING,
                };
                (cmp::max(0, w as i32) as u16, cmp::max(0, h as i32) as u16)
            }
        };
        (control.measured.0, control.measured.1, control.measured != old_size)
    }
}

// ===============================================================================================================

pub trait ControlInner: HasSizeInner + HasVisibilityInner + HasLayoutInner + Drawable {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, w: u16, h: u16);
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container);

    fn parent(&self) -> Option<&dyn controls::Member>;
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member>;
    fn root(&self) -> Option<&dyn controls::Member>;
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry);
}

#[repr(C)]
pub struct ControlBase {
    pub layout: layout::Attributes,
    pub visibility: types::Visibility,
    pub measured: (u16, u16),
    pub coords: Option<(i32, i32)>,
    pub skip_draw: bool,

    pub on_size: Option<callbacks::OnSize>,
    pub on_visibility: Option<callbacks::OnVisibility>,
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
            visibility: types::Visibility::Visible,
            measured: (0, 0),
            coords: None,
            skip_draw: false,

            on_size: None,
            on_visibility: None,
        }
    }
}

impl<T: ControlInner> HasNativeIdInner for Control<T> {
    type Id = T::Id;

    #[inline]
    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}

impl<T: ControlInner> MemberInner for Control<T> {}

impl<T: ControlInner> HasBase for Control<T> {
    type Base = ControlBase;

    #[inline]
    fn base(&self) -> &Self::Base {
        &self.base
    }
    #[inline]
    fn base_mut(&mut self) -> &mut Self::Base {
        &mut self.base
    }
}
impl<T: ControlInner> HasInner for Control<T> {
    type Inner = T;
    type Params = ();

    #[inline]
    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        Control { inner: inner, base: Default::default() }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
    #[inline]
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<T: ControlInner> HasLayoutInner for Control<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
    #[inline]
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner.layout_margin(member)
    }
}
impl<T: ControlInner> controls::MaybeHasSize for Member<Control<T>> {
    fn is_has_size(&self) -> Option<&dyn controls::HasSize> {
        Some(self)
    }
    fn is_has_size_mut(&mut self) -> Option<&mut dyn controls::HasSize> {
        Some(self)
    }
}
impl<T: ControlInner> OuterDrawable for Member<Control<T>> {
    #[inline]
    fn draw(&mut self, coords: Option<(i32, i32)>) {
        if coords.is_some() {
            self.inner.base.coords = coords;
        }
        if !self.is_skip_draw() {
            self.inner.inner.draw(&mut self.base, &mut self.inner.base)
        }
    }
    #[inline]
    fn coords(&self) -> Option<(i32, i32)> {
        self.inner.base.coords
    }
    #[inline]
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.inner.measure(&mut self.base, &mut self.inner.base, w, h)
    }
    #[inline]
    fn invalidate(&mut self) {
        self.inner.inner.invalidate(&mut self.base, &mut self.inner.base)
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
    fn as_drawable(&self) -> &dyn OuterDrawable {
        self
    }
    #[inline]
    fn as_drawable_mut(&mut self) -> &mut dyn OuterDrawable {
        self
    }
    #[inline]
    fn into_drawable(self: Box<Self>) -> Box<dyn OuterDrawable> {
        self
    }
}
impl<T: ControlInner> controls::HasVisibility for Member<Control<T>> {
    #[inline]
    fn visibility(&self) -> types::Visibility {
        self.inner.base.visibility
    }
    #[inline]
    fn set_visibility(&mut self, visibility: types::Visibility) {
        if self.inner.inner.on_visibility_set(&mut self.base, visibility) {
            self.inner.base.visibility = visibility;
            self.call_on_visibility(visibility);
        }
    }
    #[inline]
    fn on_visibility(&mut self, callback: Option<callbacks::OnVisibility>) {
        self.inner.base.on_visibility = callback;
    }
    #[inline]
    fn as_has_visibility(&self) -> &dyn controls::HasVisibility {
        self
    }
    #[inline]
    fn as_has_visibility_mut(&mut self) -> &mut dyn controls::HasVisibility {
        self
    }
    #[inline]
    fn into_has_visibility(self: Box<Self>) -> Box<dyn controls::HasVisibility> {
        self
    }
}
impl<T: ControlInner> controls::HasSize for Member<Control<T>> {
    #[inline]
    fn size(&self) -> (u16, u16) {
        self.inner.base.measured
    }
    #[inline]
    fn set_size(&mut self, width: u16, height: u16) {
        if self.inner.inner.on_size_set(&mut self.base, (width, height)) {
            self.inner.base.measured = (width, height);
            self.call_on_size(width, height);
        }
    }
    #[inline]
    fn on_size(&mut self, callback: Option<callbacks::OnSize>) {
        self.inner.base.on_size = callback;
    }

    #[inline]
    fn as_has_size(&self) -> &dyn controls::HasSize {
        self
    }
    #[inline]
    fn as_has_size_mut(&mut self) -> &mut dyn controls::HasSize {
        self
    }
    #[inline]
    fn into_has_size(self: Box<Self>) -> Box<dyn controls::HasSize> {
        self
    }
}
impl<T: ControlInner> controls::Control for Member<Control<T>> {
    #[inline]
    fn on_added_to_container(&mut self, parent: &dyn controls::Container, x: i32, y: i32, w: u16, h: u16) {
        #[cfg(feature = "type_check")]
        unsafe {
            if self.inner.inner.native_id().type_id() != parent.type_id() {
                panic!("Attempt to use the control from an incompatible backend!")
            }
        }
        self.inner.base.coords = Some((x, y));
        self.inner.inner.on_added_to_container(&mut self.base, &mut self.inner.base, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, parent: &dyn controls::Container) {
        self.inner.inner.on_removed_from_container(&mut self.base, &mut self.inner.base, parent);
        self.inner.base.coords = None;
    }

    #[inline]
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.inner.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn controls::Member> {
        self.inner.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    default fn fill_from_markup(&mut self, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        self.inner.inner.fill_from_markup(&mut self.base, &mut self.inner.base, markup, registry)
    }

    #[inline]
    fn as_control(&self) -> &dyn controls::Control {
        self
    }
    #[inline]
    fn as_control_mut(&mut self) -> &mut dyn controls::Control {
        self
    }
    #[inline]
    fn into_control(self: Box<Self>) -> Box<dyn controls::Control> {
        self
    }
}
impl<T: ControlInner> controls::MaybeControl for Member<Control<T>> {
    #[inline]
    fn is_control(&self) -> Option<&dyn controls::Control> {
        Some(self)
    }
    #[inline]
    fn is_control_mut(&mut self) -> Option<&mut dyn controls::Control> {
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

    #[inline]
    pub fn call_on_size(&mut self, w: u16, h: u16) {
        let self2 = self as *mut Self;
        if let Some(ref mut cb) = self.inner.base_mut().on_size {
            (cb.as_mut())(unsafe { &mut *self2 }, w, h);
        }
    }
    #[inline]
    pub fn call_on_visibility(&mut self, v: types::Visibility) {
        let self2 = self as *mut Self;
        if let Some(ref mut cb) = self.inner.base_mut().on_visibility {
            (cb.as_mut())(unsafe { &mut *self2 }, v);
        }
    }
    #[inline]
    pub fn as_base_parts_mut(&mut self) -> (&mut MemberBase, &mut ControlBase, &mut T) {
        let self2 = self as *mut Self;
        let self3 = self as *mut Self;
        (unsafe { &mut *self2 }.base_mut(), unsafe { &mut *self3 }.as_inner_mut().base_mut(), self.as_inner_mut().as_inner_mut())
    }
    
    #[inline]
    pub fn control_base_parts_mut(base: &mut MemberBase) -> (&mut MemberBase, &mut ControlBase) {
        let this = base as *mut _ as *mut Self;
        (base, unsafe { &mut *this }.as_inner_mut().base_mut())
    }
}

// ===============================================================================================================

pub trait ContainerInner: MemberInner {
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control>;
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control>;
}
impl<T: ContainerInner> controls::Container for Member<T> {
    #[inline]
    default fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    default fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        self.inner.find_control(arg)
    }

    #[inline]
    fn as_container(&self) -> &dyn controls::Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut dyn controls::Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<dyn controls::Container> {
        self
    }
}

// ===============================================================================================================

pub trait SingleContainerInner: ContainerInner {
    fn set_child(&mut self, base: &mut MemberBase, child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>>;
    fn child(&self) -> Option<&dyn controls::Control>;
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control>;
}

pub struct SingleContainer<T: SingleContainerInner> {
    inner: T,
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
    #[inline]
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<T: SingleContainerInner> HasNativeIdInner for SingleContainer<T> {
    type Id = T::Id;

    #[inline]
    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: SingleContainerInner> MemberInner for SingleContainer<T> {}

impl<T: SingleContainerInner + ContainerInner> ContainerInner for SingleContainer<T> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        self.inner.find_control(arg)
    }
}
impl<T: SingleContainerInner + ControlInner + Drawable> Drawable for SingleContainer<T> {
    #[inline]
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.draw(member, control)
    }
    #[inline]
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(member, control, w, h)
    }
    #[inline]
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
impl<T: SingleContainerInner + ControlInner> HasLayoutInner for SingleContainer<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
    #[inline]
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner.layout_margin(member)
    }
}
impl<T: SingleContainerInner + ControlInner> HasSizeInner for SingleContainer<T> {
    fn on_size_set(&mut self, base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.inner.on_size_set(base, value)
    }
}
impl<T: SingleContainerInner + ControlInner> HasVisibilityInner for SingleContainer<T> {
    fn on_visibility_set(&mut self, base: &mut MemberBase, value: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, value)
    }
}
impl<T: SingleContainerInner + ControlInner> ControlInner for SingleContainer<T> {
    #[inline]
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, w: u16, h: u16) {
        self.inner.on_added_to_container(member, control, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container) {
        self.inner.on_removed_from_container(member, control, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn controls::Member> {
        self.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        self.inner.fill_from_markup(member, control, markup, registry)
    }
}
impl<T: SingleContainerInner> controls::SingleContainer for Member<SingleContainer<T>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn controls::Control> {
        self.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control> {
        self.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &dyn controls::SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut dyn controls::SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<dyn controls::SingleContainer> {
        self
    }
}
impl<T: SingleContainerInner> controls::MaybeContainer for Member<SingleContainer<T>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn controls::Container> {
        Some(self)
    }
}
impl<T: SingleContainerInner> controls::Container for Member<SingleContainer<T>> {
    #[inline]
    fn is_single_mut(&mut self) -> Option<&mut dyn controls::SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single(&self) -> Option<&dyn controls::SingleContainer> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> controls::SingleContainer for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn set_child(&mut self, child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.inner.set_child(&mut self.base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn controls::Control> {
        self.inner.inner.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control> {
        self.inner.inner.inner.child_mut()
    }

    #[inline]
    fn as_single_container(&self) -> &dyn controls::SingleContainer {
        self
    }
    #[inline]
    fn as_single_container_mut(&mut self) -> &mut dyn controls::SingleContainer {
        self
    }
    #[inline]
    fn into_single_container(self: Box<Self>) -> Box<dyn controls::SingleContainer> {
        self
    }
}
impl<T: SingleContainerInner + ControlInner> controls::MaybeContainer for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn controls::Container> {
        Some(self)
    }
}
impl<T: SingleContainerInner + ControlInner> controls::Container for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.find_control(arg)
    }

    #[inline]
    fn is_single_mut(&mut self) -> Option<&mut dyn controls::SingleContainer> {
        Some(self)
    }
    #[inline]
    fn is_single(&self) -> Option<&dyn controls::SingleContainer> {
        Some(self)
    }

    #[inline]
    fn as_container(&self) -> &dyn controls::Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut dyn controls::Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<dyn controls::Container> {
        self
    }
}

// ===============================================================================================================

pub trait MultiContainerInner: ContainerInner {
    fn len(&self) -> usize;
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>>;
    fn remove_child_from(&mut self, base: &mut MemberBase, index: usize) -> Option<Box<dyn controls::Control>>;
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control>;
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control>;

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
    fn push_child(&mut self, base: &mut MemberBase, child: Box<dyn controls::Control>) {
        let len = self.len();
        self.set_child_to(base, len, child);
    }
    #[inline]
    fn pop_child(&mut self, base: &mut MemberBase) -> Option<Box<dyn controls::Control>> {
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
impl<T: MultiContainerInner> HasNativeIdInner for MultiContainer<T> {
    type Id = T::Id;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: MultiContainerInner> MemberInner for MultiContainer<T> {}

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
    #[inline]
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<T: MultiContainerInner + ContainerInner> ContainerInner for MultiContainer<T> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        self.inner.find_control(arg)
    }
}
impl<T: MultiContainerInner + ControlInner + Drawable> Drawable for MultiContainer<T> {
    #[inline]
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.draw(member, control)
    }
    #[inline]
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(member, control, w, h)
    }
    #[inline]
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
impl<T: MultiContainerInner + ControlInner> HasLayoutInner for MultiContainer<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner.layout_margin(member)
    }
}
impl<T: MultiContainerInner + ControlInner> HasSizeInner for MultiContainer<T> {
    fn on_size_set(&mut self, base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.inner.on_size_set(base, value)
    }
}
impl<T: MultiContainerInner + ControlInner> HasVisibilityInner for MultiContainer<T> {
    fn on_visibility_set(&mut self, base: &mut MemberBase, value: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, value)
    }
}
impl<T: MultiContainerInner + ControlInner> ControlInner for MultiContainer<T> {
    #[inline]
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, w: u16, h: u16) {
        self.inner.on_added_to_container(member, control, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container) {
        self.inner.on_removed_from_container(member, control, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn controls::Member> {
        self.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        self.inner.fill_from_markup(member, control, markup, registry)
    }
}
impl<T: MultiContainerInner> controls::MultiContainer for Member<MultiContainer<T>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        self.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        self.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &dyn controls::MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut dyn controls::MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<dyn controls::MultiContainer> {
        self
    }
}
impl<T: MultiContainerInner> controls::MaybeContainer for Member<MultiContainer<T>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn controls::Container> {
        Some(self)
    }
}
impl<T: MultiContainerInner> controls::Container for Member<MultiContainer<T>> {
    #[inline]
    fn is_multi_mut(&mut self) -> Option<&mut dyn controls::MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi(&self) -> Option<&dyn controls::MultiContainer> {
        Some(self)
    }
}
impl<T: MultiContainerInner + ControlInner> controls::Container for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.find_control(arg)
    }

    #[inline]
    fn is_multi_mut(&mut self) -> Option<&mut dyn controls::MultiContainer> {
        Some(self)
    }
    #[inline]
    fn is_multi(&self) -> Option<&dyn controls::MultiContainer> {
        Some(self)
    }

    #[inline]
    fn as_container(&self) -> &dyn controls::Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut dyn controls::Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<dyn controls::Container> {
        self
    }
}
impl<T: MultiContainerInner + ControlInner> controls::MaybeContainer for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn is_container(&self) -> Option<&dyn controls::Container> {
        Some(self)
    }
    #[inline]
    fn is_container_mut(&mut self) -> Option<&mut dyn controls::Container> {
        Some(self)
    }
}
impl<T: MultiContainerInner + ControlInner> controls::MultiContainer for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn len(&self) -> usize {
        self.inner.inner.inner.len()
    }
    #[inline]
    fn set_child_to(&mut self, index: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.inner.set_child_to(&mut self.base, index, child)
    }
    #[inline]
    fn remove_child_from(&mut self, index: usize) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.inner.remove_child_from(&mut self.base, index)
    }
    #[inline]
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        self.inner.inner.inner.child_at(index)
    }
    #[inline]
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        self.inner.inner.inner.child_at_mut(index)
    }

    #[inline]
    fn as_multi_container(&self) -> &dyn controls::MultiContainer {
        self
    }
    #[inline]
    fn as_multi_container_mut(&mut self) -> &mut dyn controls::MultiContainer {
        self
    }
    #[inline]
    fn into_multi_container(self: Box<Self>) -> Box<dyn controls::MultiContainer> {
        self
    }
}

// ===============================================================================================================

impl<T: MemberInner + HasLabelInner> controls::HasLabel for Member<T> {
    #[inline]
    fn label(&self) -> Cow<str> {
        self.inner.label(&self.base)
    }
    #[inline]
    fn set_label(&mut self, label: Cow<str>) {
        self.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &dyn controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut dyn controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<dyn controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + ControlInner + MemberInner> controls::HasLabel for Member<Control<T>> {
    #[inline]
    fn label(&self) -> Cow<'_, str> {
        self.inner.inner.label(&self.base)
    }
    #[inline]
    fn set_label(&mut self, label: Cow<str>) {
        self.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &dyn controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut dyn controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<dyn controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + SingleContainerInner> controls::HasLabel for Member<SingleContainer<T>> {
    #[inline]
    fn label(&self) -> Cow<'_, str> {
        self.inner.inner.label(&self.base)
    }
    #[inline]
    fn set_label(&mut self, label: Cow<str>) {
        self.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &dyn controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut dyn controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<dyn controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + MultiContainerInner> controls::HasLabel for Member<MultiContainer<T>> {
    #[inline]
    fn label(&self) -> Cow<'_, str> {
        self.inner.inner.label(&self.base)
    }
    #[inline]
    fn set_label(&mut self, label: Cow<str>) {
        self.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &dyn controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut dyn controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<dyn controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + ControlInner + SingleContainerInner> controls::HasLabel for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn label(&self) -> Cow<'_, str> {
        self.inner.inner.inner.label(&self.base)
    }
    #[inline]
    fn set_label(&mut self, label: Cow<str>) {
        self.inner.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &dyn controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut dyn controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<dyn controls::HasLabel> {
        self
    }
}
impl<T: HasLabelInner + ControlInner + MultiContainerInner> controls::HasLabel for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn label(&self) -> Cow<'_, str> {
        self.inner.inner.inner.label(&self.base)
    }
    #[inline]
    fn set_label(&mut self, label: Cow<str>) {
        self.inner.inner.inner.set_label(&mut self.base, label)
    }

    #[inline]
    fn as_has_label(&self) -> &dyn controls::HasLabel {
        self
    }
    #[inline]
    fn as_has_label_mut(&mut self) -> &mut dyn controls::HasLabel {
        self
    }
    #[inline]
    fn into_has_label(self: Box<Self>) -> Box<dyn controls::HasLabel> {
        self
    }
}

// ==============================================================================================================

impl<T: CloseableInner + MemberInner> controls::Closeable for Member<T> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.inner.on_close(callback)
    }

    #[inline]
    fn as_closeable(&self) -> &dyn controls::Closeable {
        self
    }
    #[inline]
    fn as_closeable_mut(&mut self) -> &mut dyn controls::Closeable {
        self
    }
    #[inline]
    fn into_closeable(self: Box<Self>) -> Box<dyn controls::Closeable> {
        self
    }
}

impl<T: CloseableInner + ControlInner> controls::Closeable for Member<Control<T>> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.inner.inner.on_close(callback)
    }

    #[inline]
    fn as_closeable(&self) -> &dyn controls::Closeable {
        self
    }
    #[inline]
    fn as_closeable_mut(&mut self) -> &mut dyn controls::Closeable {
        self
    }
    #[inline]
    fn into_closeable(self: Box<Self>) -> Box<dyn controls::Closeable> {
        self
    }
}

impl<T: CloseableInner + SingleContainerInner> controls::Closeable for Member<SingleContainer<T>> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.inner.inner.on_close(callback)
    }

    #[inline]
    fn as_closeable(&self) -> &dyn controls::Closeable {
        self
    }
    #[inline]
    fn as_closeable_mut(&mut self) -> &mut dyn controls::Closeable {
        self
    }
    #[inline]
    fn into_closeable(self: Box<Self>) -> Box<dyn controls::Closeable> {
        self
    }
}

impl<T: CloseableInner + MultiContainerInner> controls::Closeable for Member<MultiContainer<T>> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.inner.inner.on_close(callback)
    }

    #[inline]
    fn as_closeable(&self) -> &dyn controls::Closeable {
        self
    }
    #[inline]
    fn as_closeable_mut(&mut self) -> &mut dyn controls::Closeable {
        self
    }
    #[inline]
    fn into_closeable(self: Box<Self>) -> Box<dyn controls::Closeable> {
        self
    }
}

impl<T: CloseableInner + ControlInner + SingleContainerInner> controls::Closeable for Member<Control<SingleContainer<T>>> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.inner.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.inner.inner.inner.on_close(callback)
    }

    #[inline]
    fn as_closeable(&self) -> &dyn controls::Closeable {
        self
    }
    #[inline]
    fn as_closeable_mut(&mut self) -> &mut dyn controls::Closeable {
        self
    }
    #[inline]
    fn into_closeable(self: Box<Self>) -> Box<dyn controls::Closeable> {
        self
    }
}

impl<T: CloseableInner + ControlInner + MultiContainerInner> controls::Closeable for Member<Control<MultiContainer<T>>> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.inner.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.inner.inner.inner.on_close(callback)
    }

    #[inline]
    fn as_closeable(&self) -> &dyn controls::Closeable {
        self
    }
    #[inline]
    fn as_closeable_mut(&mut self) -> &mut dyn controls::Closeable {
        self
    }
    #[inline]
    fn into_closeable(self: Box<Self>) -> Box<dyn controls::Closeable> {
        self
    }
}

// ===============================================================================================================

impl<T: ClickableInner + MemberInner> controls::Clickable for Member<T> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::OnClick>) {
        self.inner.on_click(cb)
    }
    #[inline]
    fn click(&mut self, skip_callbacks: bool) {
        self.inner.click(skip_callbacks)
    }

    #[inline]
    fn as_clickable(&self) -> &dyn controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut dyn controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<dyn controls::Clickable> {
        self
    }
}
impl<T: ClickableInner + ControlInner> controls::Clickable for Member<Control<T>> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::OnClick>) {
        self.inner.inner.on_click(cb)
    }
    #[inline]
    fn click(&mut self, skip_callbacks: bool) {
        self.inner.inner.click(skip_callbacks)
    }

    #[inline]
    fn as_clickable(&self) -> &dyn controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut dyn controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<dyn controls::Clickable> {
        self
    }
}
impl<T: ClickableInner + ControlInner + SingleContainerInner> controls::Clickable for Member<Control<SingleContainer<T>>> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::OnClick>) {
        self.inner.inner.inner.on_click(cb)
    }
    #[inline]
    fn click(&mut self, skip_callbacks: bool) {
        self.inner.inner.inner.click(skip_callbacks)
    }

    #[inline]
    fn as_clickable(&self) -> &dyn controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut dyn controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<dyn controls::Clickable> {
        self
    }
}
impl<T: ClickableInner + ControlInner + MultiContainerInner> controls::Clickable for Member<Control<MultiContainer<T>>> {
    #[inline]
    fn on_click(&mut self, cb: Option<callbacks::OnClick>) {
        self.inner.inner.inner.on_click(cb)
    }
    #[inline]
    fn click(&mut self, skip_callbacks: bool) {
        self.inner.inner.inner.click(skip_callbacks)
    }

    #[inline]
    fn as_clickable(&self) -> &dyn controls::Clickable {
        self
    }
    #[inline]
    fn as_clickable_mut(&mut self) -> &mut dyn controls::Clickable {
        self
    }
    #[inline]
    fn into_clickable(self: Box<Self>) -> Box<dyn controls::Clickable> {
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
    fn as_has_orientation(&self) -> &dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<dyn controls::HasOrientation> {
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
        self.inner.inner.set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<dyn controls::HasOrientation> {
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
        self.inner.inner.set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<dyn controls::HasOrientation> {
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
        self.inner.inner.set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<dyn controls::HasOrientation> {
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
        self.inner.inner.inner.set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<dyn controls::HasOrientation> {
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
        self.inner.inner.inner.set_layout_orientation(&mut self.base, value)
    }

    #[inline]
    fn as_has_orientation(&self) -> &dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn as_has_orientation_mut(&mut self) -> &mut dyn controls::HasOrientation {
        self
    }
    #[inline]
    fn into_has_orientation(self: Box<Self>) -> Box<dyn controls::HasOrientation> {
        self
    }
}

// ===============================================================================================================

pub trait ApplicationInner: HasNativeIdInner + 'static {
    fn get() -> Box<Application<Self>>
    where
        Self: Sized;
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window>;
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray>;

    fn remove_window(&mut self, id: Self::Id);
    fn remove_tray(&mut self, id: Self::Id);

    fn name(&self) -> Cow<'_, str>;
    fn start(&mut self);

    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member>;
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member>;

    fn exit(&mut self, skip_on_close: bool) -> bool;

    fn on_frame_async_feeder(&mut self, feeder: callbacks::AsyncFeeder<callbacks::OnFrame>) -> callbacks::AsyncFeeder<callbacks::OnFrame> {
        feeder
    }
    fn on_frame(&mut self, feeder: &mut callbacks::AsyncFeeder<callbacks::OnFrame>, cb: callbacks::OnFrame) {
        let _ = feeder.feed(cb);
    }
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &(dyn controls::Member)> + 'a>;
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &mut (dyn controls::Member)> + 'a>;
}
pub struct Application<T: ApplicationInner> {
    inner: Rc<UnsafeCell<ApplicationInnerWrapper<T>>>,
}
pub struct ApplicationBase {
    queue: mpsc::Receiver<callbacks::OnFrame>,
    sender: mpsc::Sender<callbacks::OnFrame>,
}
pub struct ApplicationInnerWrapper<T: ApplicationInner> {
    base: ApplicationBase,
    inner: T,
}
impl ApplicationBase {
    pub fn sender(&mut self) -> &mut mpsc::Sender<callbacks::OnFrame> {
        &mut self.sender
    }
    pub fn queue(&mut self) -> &mut mpsc::Receiver<callbacks::OnFrame> {
        &mut self.queue
    }
}
impl<T: ApplicationInner> HasBase for Application<T> {
    type Base = ApplicationBase;

    fn base(&self) -> &Self::Base {
        unsafe { &(&*self.inner.get()).base }
    }
    fn base_mut(&mut self) -> &mut Self::Base {
        unsafe { &mut (&mut *self.inner.get()).base }
    }
}
impl<T: ApplicationInner> controls::HasNativeId for Application<T> {
    #[inline]
    unsafe fn native_id(&self) -> usize {
        self.as_inner().native_id().into()
    }
}
impl<T: ApplicationInner> controls::Application for Application<T> {
    #[inline]
    fn new_window(&mut self, title: &str, size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        self.as_inner_mut().new_window(title, size, menu)
    }
    #[inline]
    fn new_tray(&mut self, title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        self.as_inner_mut().new_tray(title, menu)
    }
    #[inline]
    fn name(&self) -> Cow<'_, str> {
        self.as_inner().name()
    }
    #[inline]
    fn start(&mut self) {
        self.as_inner_mut().start()
    }
    #[inline]
    fn find_member_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Member> {
        self.as_inner_mut().find_member_mut(arg)
    }
    #[inline]
    fn find_member(&self, arg: types::FindBy) -> Option<&dyn controls::Member> {
        self.as_inner().find_member(arg)
    }
    #[inline]
    fn exit(mut self: Box<Self>, skip_on_close: bool) -> bool {
        let exited = self.as_inner_mut().exit(skip_on_close);
        if exited {
            runtime::deinit(&self.inner);
        }
        exited
    }
    #[inline]
    fn on_frame_async_feeder(&mut self) -> callbacks::AsyncFeeder<callbacks::OnFrame> {
        let feeder = self.base_mut().sender().clone();
        self.as_inner_mut().on_frame_async_feeder(feeder.into())
    }
    #[inline]
    fn on_frame(&mut self, cb: callbacks::OnFrame) {
        let mut feeder = self.base_mut().sender().clone().into();
        self.as_inner_mut().on_frame(&mut feeder, cb)
    }
    #[inline]
    fn members<'a>(&'a self) -> Box<dyn Iterator<Item = &'a (dyn controls::Member)> + 'a> {
        self.as_inner().members()
    }
    #[inline]
    fn members_mut<'a>(&'a mut self) -> Box<dyn Iterator<Item = &'a mut (dyn controls::Member)> + 'a> {
        self.as_inner_mut().members_mut()
    }
}
impl<T: ApplicationInner> controls::AsAny for Application<T> {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}

impl<T: ApplicationInner> HasInner for Application<T> {
    type Inner = T;
    type Params = ();

    #[inline]
    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        let (tx, rx) = mpsc::channel();
        Application {
            inner: Rc::new(UnsafeCell::new(ApplicationInnerWrapper {
                base: ApplicationBase { sender: tx, queue: rx },
                inner: inner,
            })),
        }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        unsafe { &(&*self.inner.get()).inner }
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        unsafe { &mut (&mut *self.inner.get()).inner }
    }
    #[inline]
    fn into_inner(self) -> Self::Inner {
        panic!("Never unwrap an Application");
    }
}
impl<T: ApplicationInner> Application<T> {
    #[inline]
    pub fn get() -> types::ApplicationResult {
        let (inner, ready) = runtime::get::<T>();
        if let Some(inner) = inner {
            types::ApplicationResult::Existing(Box::new(Application { inner }))
        } else if ready {
            types::ApplicationResult::ErrorNonUiThread
        } else {
            let app = T::get();
            runtime::init(app.inner.clone());
            types::ApplicationResult::New(app)
        }
    }
}
impl<T: ApplicationInner> seal::Sealed for Application<T> {}

// ===============================================================================================================

pub struct WindowBase {
    pub visibility: types::Visibility,
    pub on_size: Option<callbacks::OnSize>,
    pub on_visibility: Option<callbacks::OnVisibility>,
}
impl WindowBase {
    pub(crate) fn new() -> Self {
        WindowBase {
            visibility: types::Visibility::Visible,

            on_size: None,
            on_visibility: None,
        }
    }
}

pub struct Window<T: WindowInner> {
    base: WindowBase,
    inner: T,
}

impl<T: WindowInner> HasBase for Window<T> {
    type Base = WindowBase;

    fn base(&self) -> &Self::Base {
        &self.base
    }
    fn base_mut(&mut self) -> &mut Self::Base {
        &mut self.base
    }
}
impl<T: WindowInner> HasInner for Window<T> {
    type Inner = T;
    type Params = ();

    fn with_inner(inner: Self::Inner, _: Self::Params) -> Self {
        Window { base: WindowBase::new(), inner: inner }
    }
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<T: WindowInner> HasNativeIdInner for Window<T> {
    type Id = T::Id;

    #[inline]
    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: WindowInner> MemberInner for Window<T> {}

impl<T: WindowInner> ContainerInner for Window<T> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        self.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        self.inner.find_control(arg)
    }
}
impl<T: WindowInner> SingleContainerInner for Window<T> {
    #[inline]
    fn set_child(&mut self, base: &mut MemberBase, child: Option<Box<dyn controls::Control>>) -> Option<Box<dyn controls::Control>> {
        self.inner.set_child(base, child)
    }
    #[inline]
    fn child(&self) -> Option<&dyn controls::Control> {
        self.inner.child()
    }
    #[inline]
    fn child_mut(&mut self) -> Option<&mut dyn controls::Control> {
        self.inner.child_mut()
    }
}
impl<T: WindowInner> HasLabelInner for Window<T> {
    #[inline]
    fn label(&self, base: &MemberBase) -> Cow<'_, str> {
        self.inner.label(base)
    }
    #[inline]
    fn set_label(&mut self, base: &mut MemberBase, label: Cow<str>) {
        self.inner.set_label(base, label)
    }
}

impl<T: WindowInner> CloseableInner for Window<T> {
    fn close(&mut self, skip_callbacks: bool) -> bool {
        self.inner.close(skip_callbacks)
    }
    fn on_close(&mut self, callback: Option<callbacks::OnClose>) {
        self.inner.on_close(callback)
    }
}
impl<T: WindowInner> Member<SingleContainer<Window<T>>> {
    pub fn call_on_size(&mut self, w: u16, h: u16) {
        let self2 = self as *mut Self;
        if let Some(ref mut cb) = self.inner.inner.base_mut().on_size {
            (cb.as_mut())(unsafe { &mut *self2 }, w, h);
        }
    }
    pub fn call_on_visibility(&mut self, v: types::Visibility) {
        let self2 = self as *mut Self;
        if let Some(ref mut cb) = self.inner.inner.base_mut().on_visibility {
            (cb.as_mut())(unsafe { &mut *self2 }, v);
        }
    }
}

pub trait WindowInner: HasSizeInner + HasVisibilityInner + HasLabelInner + CloseableInner + SingleContainerInner {
    fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::Menu) -> Box<Member<SingleContainer<Window<Self>>>>;
    fn size(&self) -> (u16, u16);
    fn position(&self) -> (i32, i32);
}
impl<T: WindowInner> controls::HasVisibility for Member<SingleContainer<Window<T>>> {
    fn visibility(&self) -> types::Visibility {
        self.inner.inner.base.visibility
    }
    fn set_visibility(&mut self, visibility: types::Visibility) {
        if self.inner.inner.inner.on_visibility_set(&mut self.base, visibility) {
            self.inner.inner.base.visibility = visibility;
            self.call_on_visibility(visibility);
        }
    }
    fn on_visibility(&mut self, callback: Option<callbacks::OnVisibility>) {
        self.inner.inner.base.on_visibility = callback;
    }

    fn as_has_visibility(&self) -> &dyn controls::HasVisibility {
        self
    }
    #[inline]
    fn as_has_visibility_mut(&mut self) -> &mut dyn controls::HasVisibility {
        self
    }
    #[inline]
    fn into_has_visibility(self: Box<Self>) -> Box<dyn controls::HasVisibility> {
        self
    }
}
impl<T: WindowInner> controls::MaybeHasSize for Member<SingleContainer<Window<T>>> {
    fn is_has_size(&self) -> Option<&dyn controls::HasSize> {
        Some(self)
    }
    fn is_has_size_mut(&mut self) -> Option<&mut dyn controls::HasSize> {
        Some(self)
    }
}
impl<T: WindowInner> controls::HasSize for Member<SingleContainer<Window<T>>> {
    #[inline]
    fn size(&self) -> (u16, u16) {
        self.inner.inner.inner.size()
    }
    #[inline]
    fn set_size(&mut self, width: u16, height: u16) {
        if self.inner.inner.inner.on_size_set(&mut self.base, (width, height)) {
            self.call_on_size(width, height);
        }
    }
    #[inline]
    fn on_size(&mut self, callback: Option<callbacks::OnSize>) {
        self.inner.inner.base.on_size = callback;
    }

    #[inline]
    fn as_has_size(&self) -> &dyn controls::HasSize {
        self
    }
    #[inline]
    fn as_has_size_mut(&mut self) -> &mut dyn controls::HasSize {
        self
    }
    #[inline]
    fn into_has_size(self: Box<Self>) -> Box<dyn controls::HasSize> {
        self
    }
}
impl<T: WindowInner> controls::Window for Member<SingleContainer<Window<T>>> {}
/* // Ban free creation of Window, use Application for that
impl<T: WindowInner> Member<SingleContainer<Window<T>>> {
    #[inline]
    pub fn with_params(title: &str, window_size: types::WindowStartSize, menu: types::Menu) -> Box<dyn controls::Window> {
        T::with_params(title, window_size, menu)
    }
}*/

// ===============================================================================================================

pub trait ButtonInner: ControlInner + ClickableInner + HasLabelInner {
    fn with_label(label: &str) -> Box<Member<Control<Self>>>;
}

impl<T: ButtonInner> controls::Button for Member<Control<T>> {}

impl<T: ButtonInner> Member<Control<T>> {
    #[inline]
    pub fn with_label(label: &str) -> Box<dyn controls::Button> {
        T::with_label(label)
    }
}

// ===============================================================================================================

pub trait LinearLayoutInner: ControlInner + MultiContainerInner + HasOrientationInner {
    fn with_orientation(orientation: layout::Orientation) -> Box<Member<Control<MultiContainer<Self>>>>;
}

impl<T: LinearLayoutInner> controls::LinearLayout for Member<Control<MultiContainer<T>>> {}

impl<T: LinearLayoutInner> Member<Control<MultiContainer<T>>> {
    #[inline]
    pub fn with_orientation(orientation: layout::Orientation) -> Box<dyn controls::LinearLayout> {
        T::with_orientation(orientation)
    }
}

// ===============================================================================================================

pub trait FrameInner: ControlInner + SingleContainerInner + HasLabelInner {
    fn with_label(label: &str) -> Box<Member<Control<SingleContainer<Self>>>>;
}

impl<T: FrameInner> controls::Frame for Member<Control<SingleContainer<T>>> {}

impl<T: FrameInner> Member<Control<SingleContainer<T>>> {
    #[inline]
    pub fn with_label(label: &str) -> Box<dyn controls::Frame> {
        T::with_label(label)
    }
}

// ===============================================================================================================

pub trait SplittedInner: MultiContainerInner + ControlInner + HasOrientationInner {
    fn with_content(first: Box<dyn controls::Control>, second: Box<dyn controls::Control>, orientation: layout::Orientation) -> Box<Member<Control<MultiContainer<Self>>>>;
    fn set_splitter(&mut self, base: &mut MemberBase, pos: f32);
    fn splitter(&self) -> f32;

    fn first(&self) -> &dyn controls::Control;
    fn second(&self) -> &dyn controls::Control;
    fn first_mut(&mut self) -> &mut dyn controls::Control;
    fn second_mut(&mut self) -> &mut dyn controls::Control;
}

impl<T: SplittedInner> controls::Splitted for Member<Control<MultiContainer<T>>> {
    fn set_splitter(&mut self, pos: f32) {
        self.inner.inner.inner.set_splitter(&mut self.base, pos)
    }
    fn splitter(&self) -> f32 {
        self.inner.inner.inner.splitter()
    }
    fn first(&self) -> &dyn controls::Control {
        self.inner.inner.inner.first()
    }
    fn second(&self) -> &dyn controls::Control {
        self.inner.inner.inner.second()
    }
    fn first_mut(&mut self) -> &mut dyn controls::Control {
        self.inner.inner.inner.first_mut()
    }
    fn second_mut(&mut self) -> &mut dyn controls::Control {
        self.inner.inner.inner.second_mut()
    }
}

impl<T: SplittedInner> Member<Control<MultiContainer<T>>> {
    #[inline]
    pub fn with_content(first: Box<dyn controls::Control>, second: Box<dyn controls::Control>, orientation: layout::Orientation) -> Box<dyn controls::Splitted> {
        T::with_content(first, second, orientation)
    }
}

// ===============================================================================================================

pub trait TextInner: ControlInner + HasLabelInner {
    fn with_text(text: &str) -> Box<Member<Control<Self>>>;
    fn empty() -> Box<Member<Control<Self>>> {
        Self::with_text("")
    }
}

impl<T: TextInner> controls::Text for Member<Control<T>> {}

impl<T: TextInner> Member<Control<T>> {
    #[inline]
    pub fn with_text(text: &str) -> Box<dyn controls::Text> {
        T::with_text(text)
    }
    #[inline]
    pub fn empty() -> Box<dyn controls::Text> {
        T::empty()
    }
}

// ===============================================================================================================

pub trait MessageInner: MemberInner + HasLabelInner {
    fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Box<Member<Self>>;
    fn severity(&self) -> types::MessageSeverity;
    fn start(self) -> Result<String, ()>;
}

impl<T: MessageInner> controls::Message for Member<T> {
    #[inline]
    fn severity(&self) -> types::MessageSeverity {
        self.as_inner().severity()
    }
    #[inline]
    fn start(self: Box<Self>) -> Result<String, ()> {
        self.into_inner().start()
    }
}

impl<T: MessageInner> Member<T> {
    #[inline]
    pub fn with_content(content: types::TextContent, severity: types::MessageSeverity, parent: Option<&dyn controls::Member>) -> Box<dyn controls::Message> {
        T::with_actions(content, severity, vec![], parent)
    }
    #[inline]
    pub fn with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Box<dyn controls::Message> {
        T::with_actions(content, severity, actions, parent)
    }
    #[inline]
    pub fn start_with_actions(content: types::TextContent, severity: types::MessageSeverity, actions: Vec<(String, callbacks::Action)>, parent: Option<&dyn controls::Member>) -> Result<String, ()> {
        use crate::controls::Message;
        T::with_actions(content, severity, actions, parent).start()
    }
}

// ===============================================================================================================

pub trait ImageInner: ControlInner {
    fn with_content(content: image::DynamicImage) -> Box<dyn controls::Image>;
    fn set_scale(&mut self, base: &mut MemberBase, policy: types::ImageScalePolicy);
    fn scale(&self) -> types::ImageScalePolicy;
}

impl<T: ImageInner + Sized + 'static> controls::Image for Member<Control<T>> {
    fn set_scale(&mut self, policy: types::ImageScalePolicy) {
        let base1 = self as *mut _ as *mut Member<Control<T>>;
        self.as_inner_mut().as_inner_mut().set_scale(unsafe { (&mut *base1).base_mut() }, policy)
    }
    fn scale(&self) -> types::ImageScalePolicy {
        self.as_inner().as_inner().scale()
    }
}
impl<T: ImageInner + Sized> Member<Control<T>> {
    pub fn with_content(content: image::DynamicImage) -> Box<dyn controls::Image> {
        T::with_content(content)
    }
}

// ===============================================================================================================

pub trait TrayInner: MemberInner + HasImageInner + HasLabelInner + CloseableInner {
    fn with_params(title: &str, menu: types::Menu) -> Box<Member<Self>>;
}

impl<T: TrayInner> controls::Tray for Member<T> {}
/* // Ban free creation of Tray, use Application for that
impl<T: TrayInner> Member<T> {
    #[inline]
    pub fn with_params(title: &str, menu: types::Menu) -> Box<dyn controls::Tray> {
        T::with_params(title, menu)
    }
}*/

// ===============================================================================================================

impl<T: HasImageInner> controls::HasImage for Member<T> {
    #[inline]
    fn image(&self) -> Cow<image::DynamicImage> {
        self.inner.image(&self.base)
    }
    #[inline]
    fn set_image(&mut self, i: Cow<image::DynamicImage>) {
        self.inner.set_image(&mut self.base, i)
    }

    #[inline]
    fn as_has_image(&self) -> &dyn controls::HasImage {
        self
    }
    #[inline]
    fn as_has_image_mut(&mut self) -> &mut dyn controls::HasImage {
        self
    }
    #[inline]
    fn into_has_image(self: Box<Self>) -> Box<dyn controls::HasImage> {
        self
    }
}
impl<T: HasImageInner + ControlInner> controls::HasImage for Member<Control<T>> {
    #[inline]
    fn image(&self) -> Cow<image::DynamicImage> {
        self.inner.inner.image(&self.base)
    }
    #[inline]
    fn set_image(&mut self, i: Cow<image::DynamicImage>) {
        self.inner.inner.set_image(&mut self.base, i)
    }

    #[inline]
    fn as_has_image(&self) -> &dyn controls::HasImage {
        self
    }
    #[inline]
    fn as_has_image_mut(&mut self) -> &mut dyn controls::HasImage {
        self
    }
    #[inline]
    fn into_has_image(self: Box<Self>) -> Box<dyn controls::HasImage> {
        self
    }
}

// ===============================================================================================================

impl<T: HasProgressInner> controls::HasProgress for Member<T> {
    fn progress(&self) -> types::Progress {
        self.inner.progress(&self.base)
    }
    fn set_progress(&mut self, arg0: types::Progress) {
        self.inner.set_progress(&mut self.base, arg0)
    }
    fn as_has_progress(&self) -> &dyn controls::HasProgress {
        self
    }
    fn as_has_progress_mut(&mut self) -> &mut dyn controls::HasProgress {
        self
    }
    fn into_has_progress(self: Box<Self>) -> Box<dyn controls::HasProgress> {
        self
    }
}
impl<T: HasProgressInner + ControlInner + Sized + 'static> controls::HasProgress for Member<Control<T>> {
    fn progress(&self) -> types::Progress {
        self.inner.inner.progress(&self.base)
    }
    fn set_progress(&mut self, arg0: types::Progress) {
        self.inner.inner.set_progress(&mut self.base, arg0)
    }
    fn as_has_progress(&self) -> &dyn controls::HasProgress {
        self
    }
    fn as_has_progress_mut(&mut self) -> &mut dyn controls::HasProgress {
        self
    }
    fn into_has_progress(self: Box<Self>) -> Box<dyn controls::HasProgress> {
        self
    }
}

// ===============================================================================================================

pub trait ProgressBarInner: ControlInner + HasProgressInner {
    fn with_progress(progress: types::Progress) -> Box<Member<Control<Self>>>;
}
impl<T: ProgressBarInner + Sized + 'static> controls::ProgressBar for Member<Control<T>> {}
impl<T: ProgressBarInner + Sized + 'static> Member<Control<T>> {
    pub fn with_progress(progress: types::Progress) -> Box<dyn controls::ProgressBar> {
        T::with_progress(progress)
    }
}

// ===============================================================================================================

pub trait TableInner: ControlInner + MultiContainerInner {
    fn with_dimensions(rows: usize, cols: usize) -> Box<Member<Control<MultiContainer<Self>>>>;
    
    fn row_len(&self) -> usize;
    fn column_len(&self) -> usize;
    fn table_child_at(&self, row: usize, col: usize) -> Option<&dyn controls::Control>;
    fn table_child_at_mut(&mut self, row: usize, col: usize) -> Option<&mut dyn controls::Control>;
    
    fn set_table_child_to(&mut self, base: &mut MemberBase, row: usize, col: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>>;
    fn remove_table_child_from(&mut self, base: &mut MemberBase, row: usize, col: usize) -> Option<Box<dyn controls::Control>>;
    
    fn add_row(&mut self) -> usize;
    fn add_column(&mut self) -> usize;
    fn insert_row(&mut self, row: usize) -> usize;
    fn insert_column(&mut self, col: usize) -> usize;
    fn delete_row(&mut self, row: usize) -> usize;
    fn delete_column(&mut self, col: usize) -> usize;
}

impl <T: TableInner + Sized + 'static> MultiContainerInner for T {
    fn len(&self) -> usize {
        self.row_len() * self.column_len()
    }
    fn set_child_to(&mut self, base: &mut MemberBase, index: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        if self.column_len() > 0 {
            self.set_table_child_to(base, index / self.column_len(), index % self.column_len(), child)
        } else {
            None
        }
    }
    fn remove_child_from(&mut self, base: &mut MemberBase, index: usize) -> Option<Box<dyn controls::Control>> {
        if self.column_len() > 0 {
            self.remove_table_child_from(base, index / self.column_len(), index % self.column_len())
        } else {
            None
        }
    }
    fn child_at(&self, index: usize) -> Option<&dyn controls::Control> {
        if self.column_len() > 0 {
            self.table_child_at(index / self.column_len(), index % self.column_len())
        } else {
            None
        }
    }
    fn child_at_mut(&mut self, index: usize) -> Option<&mut dyn controls::Control> {
        if self.column_len() > 0 {
            self.table_child_at_mut(index / self.column_len(), index % self.column_len())
        } else {
            None
        }
    }
}

impl <T: TableInner + Sized + 'static> controls::Table for Member<Control<MultiContainer<T>>> {
    fn row_len(&self) -> usize {
        self.inner.inner.inner.row_len()
    }
    fn column_len(&self) -> usize {
        self.inner.inner.inner.column_len()
    }
    fn table_child_at(&self, row: usize, col: usize) -> Option<&dyn controls::Control> {
        self.inner.inner.inner.table_child_at(row, col)
    }
    fn table_child_at_mut(&mut self, row: usize, col: usize) -> Option<&mut dyn controls::Control> {
        self.inner.inner.inner.table_child_at_mut(row, col)
    }
    
    fn set_table_child_to(&mut self, row: usize, col: usize, child: Box<dyn controls::Control>) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.inner.set_table_child_to(&mut self.base, row, col, child)
    }
    fn remove_table_child_from(&mut self, row: usize, col: usize) -> Option<Box<dyn controls::Control>> {
        self.inner.inner.inner.remove_table_child_from(&mut self.base, row, col)
    }
    
    fn add_row(&mut self) -> usize {
        self.inner.inner.inner.add_row()
    }
    fn add_column(&mut self) -> usize {
        self.inner.inner.inner.add_column()
    }
    fn insert_row(&mut self, row: usize) -> usize {
        self.inner.inner.inner.insert_row(row)
    }
    fn insert_column(&mut self, col: usize) -> usize {
        self.inner.inner.inner.insert_column(col)
    }
    fn delete_row(&mut self, row: usize) -> usize {
        self.inner.inner.inner.delete_row(row)
    }
    fn delete_column(&mut self, col: usize) -> usize {
        self.inner.inner.inner.delete_column(col)
    }
}
impl<T: TableInner + Sized + 'static> Member<Control<MultiContainer<T>>> {
    pub fn with_dimensions(rows: usize, cols: usize) -> Box<dyn controls::Table> {
        T::with_dimensions(rows, cols)
    }
}

// ===============================================================================================================

pub trait AdapterViewInner: ControlInner + ContainerInner {
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<Member<Control<Adapter<Self>>>>;
}

#[repr(C)]
pub struct AdapterBase {
    pub adapter: Box<dyn types::Adapter>,
}
#[repr(C)]
pub struct Adapter<T: AdapterViewInner> {
    base: AdapterBase,
    inner: T,
}

impl<T: AdapterViewInner> HasNativeIdInner for Adapter<T> {
    type Id = T::Id;

    unsafe fn native_id(&self) -> Self::Id {
        self.inner.native_id()
    }
}
impl<T: AdapterViewInner> MemberInner for Adapter<T> {}

impl<T: AdapterViewInner> HasBase for Adapter<T> {
    type Base = AdapterBase;

    #[inline]
    fn base(&self) -> &Self::Base {
        &self.base
    }
    #[inline]
    fn base_mut(&mut self) -> &mut Self::Base {
        &mut self.base
    }
}

impl<T: AdapterViewInner> HasInner for Adapter<T> {
    type Inner = T;
    type Params = Box<dyn types::Adapter>;

    #[inline]
    fn with_inner(inner: Self::Inner, adapter: Self::Params) -> Self {
        Adapter { inner, base: AdapterBase { adapter } }
    }
    #[inline]
    fn as_inner(&self) -> &Self::Inner {
        &self.inner
    }
    #[inline]
    fn as_inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
    #[inline]
    fn into_inner(self) -> Self::Inner {
        self.inner
    }
}
impl<T: AdapterViewInner + Drawable> Drawable for Adapter<T> {
    #[inline]
    fn draw(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.draw(member, control)
    }
    #[inline]
    fn measure(&mut self, member: &mut MemberBase, control: &mut ControlBase, w: u16, h: u16) -> (u16, u16, bool) {
        self.inner.measure(member, control, w, h)
    }
    #[inline]
    fn invalidate(&mut self, member: &mut MemberBase, control: &mut ControlBase) {
        self.inner.invalidate(member, control)
    }
}
impl<T: AdapterViewInner> HasLayoutInner for Adapter<T> {
    #[inline]
    fn on_layout_changed(&mut self, base: &mut MemberBase) {
        self.inner.on_layout_changed(base)
    }
    fn layout_margin(&self, member: &MemberBase) -> layout::BoundarySize {
        self.inner.layout_margin(member)
    }
}
impl<T: AdapterViewInner> HasSizeInner for Adapter<T> {
    fn on_size_set(&mut self, base: &mut MemberBase, value: (u16, u16)) -> bool {
        self.inner.on_size_set(base, value)
    }
}
impl<T: AdapterViewInner> HasVisibilityInner for Adapter<T> {
    fn on_visibility_set(&mut self, base: &mut MemberBase, value: types::Visibility) -> bool {
        self.inner.on_visibility_set(base, value)
    }
}
impl<T: AdapterViewInner> ControlInner for Adapter<T> {
    #[inline]
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container, x: i32, y: i32, w: u16, h: u16) {
        self.inner.on_added_to_container(member, control, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn controls::Container) {
        self.inner.on_removed_from_container(member, control, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&dyn controls::Member> {
        self.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn controls::Member> {
        self.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn controls::Member> {
        self.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &super::markup::Markup, registry: &mut super::markup::MarkupRegistry) {
        self.inner.fill_from_markup(member, control, markup, registry)
    }
}
impl<T: AdapterViewInner> controls::AdapterView for Member<Control<Adapter<T>>> {
    #[inline]
    fn adapter(&self) -> &dyn types::Adapter {
        self.inner.inner.base.adapter.as_ref()
    }
    #[inline]
    fn adapter_mut(&mut self) -> &mut dyn types::Adapter {
        self.inner.inner.base.adapter.as_mut()
    }

    #[inline]
    fn as_adapter_view(&self) -> &dyn controls::AdapterView {
        self
    }
    #[inline]
    fn as_adapter_view_mut(&mut self) -> &mut dyn controls::AdapterView {
        self
    }
    #[inline]
    fn into_adapter_view(self: Box<Self>) -> Box<dyn controls::AdapterView> {
        self
    }
}
impl<T: AdapterViewInner> controls::Container for Member<Control<Adapter<T>>> {
    #[inline]
    fn find_control_mut(&mut self, arg: types::FindBy) -> Option<&mut dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.inner.find_control_mut(arg)
    }
    #[inline]
    fn find_control(&self, arg: types::FindBy) -> Option<&dyn controls::Control> {
        match arg {
            types::FindBy::Id(id) => {
                if self.base.id == id {
                    return Some(self);
                }
            }
            types::FindBy::Tag(ref tag) => {
                if let Some(mytag) = self.base.tag() {
                    if tag.as_str() == mytag {
                        return Some(self);
                    }
                }
            }
        }
        self.inner.inner.inner.find_control(arg)
    }

    #[inline]
    fn is_adapter_mut(&mut self) -> Option<&mut dyn controls::AdapterView> {
        Some(self)
    }
    #[inline]
    fn is_adapter(&self) -> Option<&dyn controls::AdapterView> {
        Some(self)
    }

    #[inline]
    fn as_container(&self) -> &dyn controls::Container {
        self
    }
    #[inline]
    fn as_container_mut(&mut self) -> &mut dyn controls::Container {
        self
    }
    #[inline]
    fn into_container(self: Box<Self>) -> Box<dyn controls::Container> {
        self
    }
}
impl<T: AdapterViewInner> Member<Control<Adapter<T>>> {
    #[inline]
    pub fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<Self> {
        T::with_adapter(adapter)
    }
    
    #[inline]
    pub fn adapter_base_parts_mut(base: &mut MemberBase) -> (&mut MemberBase, &mut ControlBase, &mut AdapterBase) {
        let this = base as *mut _ as *mut Self;
        (base, unsafe { &mut *this }.as_inner_mut().base_mut(), unsafe { &mut *this }.as_inner_mut().as_inner_mut().base_mut())
    }
}

// ===============================================================================================================

pub trait ListInner: AdapterViewInner {
}

// ===============================================================================================================

pub trait OuterDrawable: seal::Sealed {
    fn draw(&mut self, coords: Option<(i32, i32)>);
    fn measure(&mut self, w: u16, h: u16) -> (u16, u16, bool);
    fn invalidate(&mut self);
    fn coords(&self) -> Option<(i32, i32)>;
    fn set_skip_draw(&mut self, skip: bool);
    fn is_skip_draw(&self) -> bool;

    fn as_drawable(&self) -> &dyn OuterDrawable;
    fn as_drawable_mut(&mut self) -> &mut dyn OuterDrawable;
    fn into_drawable(self: Box<Self>) -> Box<dyn OuterDrawable>;
}

pub(crate) mod seal {
    pub trait Sealed {}
}
