use crate::{ids};

use super::auto::{AsAny, HasInner, Abstract};
use super::container::MaybeContainer;
use super::control::MaybeControl;
use super::closeable::MaybeCloseable;
use super::clickable::MaybeClickable;
use super::has_native_id::{HasNativeId, HasNativeIdInner};
use super::has_size::MaybeHasSize;
use super::has_visibility::MaybeHasVisibility;
use super::window::MaybeWindow;
use super::message::MaybeMessage;
use super::tray::MaybeTray;
use super::seal::Sealed;
use super::has_image::MaybeHasImage;
use super::has_layout::MaybeHasLayout;
use super::has_label::MaybeHasLabel;
use super::has_progress::MaybeHasProgress;

#[cfg(feature = "type_check")]
use std::any::TypeId;

use std::any::Any;
use std::borrow::Cow;
use std::marker::PhantomData;
use std::rc::Rc;

pub trait Member: HasNativeId + AsAny + Sealed 
        + MaybeControl + MaybeContainer + MaybeHasSize + MaybeHasVisibility + MaybeHasImage + MaybeHasLayout + MaybeHasLabel + MaybeHasProgress + MaybeCloseable + MaybeClickable
        + MaybeWindow + MaybeTray + MaybeMessage {
    fn id(&self) -> ids::Id;
    fn tag(&self) -> Option<Cow<str>>;
    fn set_tag(&mut self, tag: Option<Cow<str>>);

    #[cfg(feature = "type_check")]
    unsafe fn type_id(&self) -> TypeId;

    fn as_member(&self) -> &dyn Member;
    fn as_member_mut(&mut self) -> &mut dyn Member;
    fn into_member(self: Box<Self>) -> Box<dyn Member>;
}

pub trait MemberInner: HasNativeIdInner + Sized + 'static {}

impl<T: MemberInner> Abstract for AMember<T> {}
impl<II: MemberInner, T: HasInner<I = II> + Abstract + 'static> MemberInner for T {}

#[repr(C)]
pub struct MemberBase {
    id: ids::Id,
    tag: Option<String>,

    _as_member: unsafe fn(&MemberBase) -> &dyn Member,
    _as_member_mut: unsafe fn(&mut MemberBase) -> &mut dyn Member,
    _no_threads: PhantomData<Rc<()>>,
}
#[repr(C)]
pub struct AMember<T: MemberInner> {
    pub base: MemberBase,
    pub inner: T,
}

impl MemberBase {
    #[inline]
    pub fn with_type<T: Member>() -> Self {
        MemberBase {
            id: ids::Id::next(),
            tag: None,
            _as_member: crate::utils::base_to_member::<T>,
            _as_member_mut: crate::utils::base_to_member_mut::<T>,
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
        unsafe { (self._as_member)(self) }.as_any()
    }
    #[inline]
    pub fn as_any_mut(&mut self) -> &mut dyn Any {
        unsafe { (self._as_member_mut)(self) }.as_any_mut()
    }
    #[inline]
    pub fn as_member(&self) -> &dyn Member {
        unsafe { (self._as_member)(self) }
    }
    #[inline]
    pub fn as_member_mut(&mut self) -> &mut dyn Member {
        unsafe { (self._as_member_mut)(self) }
    }
}
impl<T: MemberInner> HasNativeId for AMember<T> {
    #[inline]
    unsafe fn native_id(&self) -> usize {
        self.inner.native_id().into()
    }
}
impl<T: MemberInner> Member for AMember<T> {
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
    fn as_member(&self) -> &dyn Member {
        self
    }
    #[inline]
    fn as_member_mut(&mut self) -> &mut dyn Member {
        self
    }
    #[inline]
    fn into_member(self: Box<Self>) -> Box<dyn Member> {
        self
    }
}
impl<T: MemberInner> AsAny for AMember<T> {
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
impl<T: MemberInner> AMember<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        AMember {
            inner: inner,
            base: MemberBase::with_type::<Self>(),
        }
    }
}

impl<T: MemberInner> HasInner for AMember<T> {
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

impl<T: MemberInner> Sealed for AMember<T> {}
