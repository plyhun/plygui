use crate::{layout, types};

use super::auto::{HasInner, Spawnable};
use super::container::Container;
use super::drawable::{Drawable, OuterDrawable};
use super::has_layout::{HasLayout, HasLayoutInner};
use super::has_size::{HasSize, HasSizeInner, OnSize};
use super::has_visibility::{HasVisibility, HasVisibilityInner, OnVisibility};
use super::member::{AMember, Member, MemberBase, MemberInner};
use super::button::MaybeButton;
use super::layout_linear::MaybeLinearLayout;
use super::splitted::MaybeSplitted;
use super::frame::MaybeFrame;
use super::image::MaybeImage;
use super::list::MaybeList;
use super::progress_bar::MaybeProgressBar;
use super::text::MaybeText;

pub trait Control: HasSize + HasVisibility + HasLayout + OuterDrawable
        + MaybeButton + MaybeLinearLayout + MaybeSplitted + MaybeFrame + MaybeImage + MaybeList + MaybeProgressBar + MaybeText {
    fn on_added_to_container(&mut self, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16);
    fn on_removed_from_container(&mut self, parent: &dyn Container);

    fn parent(&self) -> Option<&dyn Member>;
    fn parent_mut(&mut self) -> Option<&mut dyn Member>;
    fn root(&self) -> Option<&dyn Member>;
    fn root_mut(&mut self) -> Option<&mut dyn Member>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, markup: &crate::markup::Markup, registry: &mut crate::markup::MarkupRegistry);

    fn as_control(&self) -> &dyn Control;
    fn as_control_mut(&mut self) -> &mut dyn Control;
    fn into_control(self: Box<Self>) -> Box<dyn Control>;
}

pub trait ControlInner: HasSizeInner + HasVisibilityInner + HasLayoutInner + Drawable + Spawnable {
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16);
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container);

    fn parent(&self) -> Option<&dyn Member>;
    fn parent_mut(&mut self) -> Option<&mut dyn Member>;
    fn root(&self) -> Option<&dyn Member>;
    fn root_mut(&mut self) -> Option<&mut dyn Member>;

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, mberarkup: &crate::markup::Markup, registry: &mut crate::markup::MarkupRegistry);
}

#[repr(C)]
pub struct ControlBase {
    pub layout: layout::Attributes,
    pub visibility: types::Visibility,
    pub measured: (u16, u16),
    pub coords: Option<(i32, i32)>,
    pub skip_draw: bool,

    pub on_size: Option<OnSize>,
    pub on_visibility: Option<OnVisibility>,
}
#[repr(C)]
pub struct AControl<T: ControlInner> {
    pub base: ControlBase,
    pub inner: T,
}

maybe!(Control);

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
impl<T: ControlInner> MemberInner for AControl<T> {}

impl<T: ControlInner> AControl<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        AControl { inner: inner, base: Default::default() }
    }
}
impl<T: ControlInner> OuterDrawable for AMember<AControl<T>> {
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
impl<T: ControlInner> HasLayout for AMember<AControl<T>> {
    fn layout(&self) -> (layout::Size, layout::Size) {
        (self.inner.base.layout.width, self.inner.base.layout.height)
    }
    fn set_layout(&mut self, width: layout::Size, height: layout::Size) {
        self.inner.base.layout.width = width;
        self.inner.base.layout.width = height;
    }

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
    fn as_has_layout(&self) -> &dyn HasLayout {
        self
    }
    #[inline]
    fn as_has_layout_mut(&mut self) -> &mut dyn HasLayout {
        self
    }
    #[inline]
    fn into_has_layout(self: Box<Self>) -> Box<dyn HasLayout> {
        self
    }
}
impl<T: ControlInner> HasVisibility for AMember<AControl<T>> {
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
    fn on_visibility(&mut self, callback: Option<OnVisibility>) {
        self.inner.base.on_visibility = callback;
    }
    #[inline]
    fn as_has_visibility(&self) -> &dyn HasVisibility {
        self
    }
    #[inline]
    fn as_has_visibility_mut(&mut self) -> &mut dyn HasVisibility {
        self
    }
    #[inline]
    fn into_has_visibility(self: Box<Self>) -> Box<dyn HasVisibility> {
        self
    }
}
impl<T: ControlInner> HasSize for AMember<AControl<T>> {
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
    fn on_size(&mut self, callback: Option<OnSize>) {
        self.inner.base.on_size = callback;
    }

    #[inline]
    fn as_has_size(&self) -> &dyn HasSize {
        self
    }
    #[inline]
    fn as_has_size_mut(&mut self) -> &mut dyn HasSize {
        self
    }
    #[inline]
    fn into_has_size(self: Box<Self>) -> Box<dyn HasSize> {
        self
    }
}
impl<T: ControlInner> Control for AMember<AControl<T>> {
    #[inline]
    fn on_added_to_container(&mut self, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16) {
        #[cfg(feature = "type_check")]
        unsafe {
            use std::any::Any;
            if self.inner.inner.native_id().type_id() != parent.type_id() {
                panic!("Attempt to use the control from an incompatible backend!")
            }
        }
        self.inner.base.coords = Some((x, y));
        self.inner.inner.on_added_to_container(&mut self.base, &mut self.inner.base, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, parent: &dyn Container) {
        self.inner.inner.on_removed_from_container(&mut self.base, &mut self.inner.base, parent);
        self.inner.base.coords = None;
    }

    #[inline]
    fn parent(&self) -> Option<&dyn Member> {
        self.inner.inner.parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner.inner.parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn Member> {
        self.inner.inner.root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner.inner.root_mut()
    }

    #[cfg(feature = "markup")]
    default fn fill_from_markup(&mut self, markup: &crate::markup::Markup, registry: &mut crate::markup::MarkupRegistry) {
        self.inner.inner.fill_from_markup(&mut self.base, &mut self.inner.base, markup, registry)
    }

    #[inline]
    fn as_control(&self) -> &dyn Control {
        self
    }
    #[inline]
    fn as_control_mut(&mut self) -> &mut dyn Control {
        self
    }
    #[inline]
    fn into_control(self: Box<Self>) -> Box<dyn Control> {
        self
    }
}
impl<T: ControlInner> AMember<AControl<T>> {
    #[inline]
    pub fn control(&self) -> &AControl<T> {
        &self.inner
    }
    #[inline]
    pub fn control_mut(&mut self) -> &mut AControl<T> {
        &mut self.inner
    }
    #[inline]
    pub fn call_on_size(&mut self, w: u16, h: u16) {
        let self2 = self as *mut Self;
        if let Some(ref mut cb) = self.inner.base.on_size {
            (cb.as_mut())(unsafe { &mut *self2 }, w, h);
        }
    }
    #[inline]
    pub fn call_on_visibility(&mut self, v: types::Visibility) {
        let self2 = self as *mut Self;
        if let Some(ref mut cb) = self.inner.base.on_visibility {
            (cb.as_mut())(unsafe { &mut *self2 }, v);
        }
    }
    #[inline]
    pub fn as_control_parts_mut(&mut self) -> (&mut MemberBase, &mut ControlBase, &mut T) {
        let self2 = self as *mut Self;
        let self3 = self as *mut Self;
        (&mut unsafe { &mut *self2 }.base, &mut unsafe { &mut *self3 }.inner.base, &mut self.inner.inner)
    }
    #[inline]
    pub fn spawn() -> Box<dyn Control> {
        T::spawn()
    }
}
impl<T: ControlInner> HasInner for AControl<T> {
    type I = T;

    #[inline]
    fn inner(&self) -> &Self::I {
        &self.inner
    }
    #[inline]
    fn inner_mut(&mut self) -> &mut Self::I {
        &mut self.inner
    }
    #[inline]
    fn into_inner(self) -> Self::I {
        self.inner
    }
}
impl<II: ControlInner, T: HasInner<I = II> + 'static> ControlInner for T {
    #[inline]
    fn on_added_to_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container, x: i32, y: i32, w: u16, h: u16) {
        self.inner_mut().on_added_to_container(member, control, parent, x, y, w, h)
    }
    #[inline]
    fn on_removed_from_container(&mut self, member: &mut MemberBase, control: &mut ControlBase, parent: &dyn Container) {
        self.inner_mut().on_removed_from_container(member, control, parent)
    }

    #[inline]
    fn parent(&self) -> Option<&dyn Member> {
        self.inner().parent()
    }
    #[inline]
    fn parent_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner_mut().parent_mut()
    }
    #[inline]
    fn root(&self) -> Option<&dyn Member> {
        self.inner().root()
    }
    #[inline]
    fn root_mut(&mut self) -> Option<&mut dyn Member> {
        self.inner_mut().root_mut()
    }

    #[cfg(feature = "markup")]
    fn fill_from_markup(&mut self, member: &mut MemberBase, control: &mut ControlBase, markup: &crate::markup::Markup, registry: &mut crate::markup::MarkupRegistry) {
        self.inner_mut().fill_from_markup(member, control, markup, registry)
    }
}
