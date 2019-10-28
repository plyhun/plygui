use crate::auto::{AsAny, MaybeContainer, MaybeControl, MaybeHasSize, MaybeHasVisibility, MaybeMember, HasSize};
use crate::{ids, runtime};

use super::{HasBase, HasInner};
use super::seal::Sealed;
use super::native_id::{HasNativeId, HasNativeIdInner};
use super::control::Control;
use super::container::Container;

#[cfg(feature = "type_check")]
use std::any::TypeId;

use std::borrow::Cow;
use std::marker::PhantomData;
use std::rc::Rc;
use std::any::Any;

pub trait Member: HasNativeId + MaybeControl + MaybeContainer + MaybeHasSize + MaybeHasVisibility + AsAny + Sealed {
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

#[repr(C)]
pub struct MemberBase {
    id: ids::Id,
    functions: MemberFunctions,
    app: usize,
    tag: Option<String>,

    _no_threads: PhantomData<Rc<()>>,
}
#[repr(C)]
pub struct AMember<T: MemberInner> {
    base: MemberBase,
    inner: T,
}
#[repr(C)]
pub struct MemberFunctions {
    _as_any: unsafe fn(&MemberBase) -> &dyn Any,
    _as_any_mut: unsafe fn(&mut MemberBase) -> &mut dyn Any,
    _as_member: unsafe fn(&MemberBase) -> &dyn Member,
    _as_member_mut: unsafe fn(&mut MemberBase) -> &mut dyn Member,
}
impl MemberFunctions {
    #[inline]
    pub fn new(
        _as_any: unsafe fn(&MemberBase) -> &dyn Any,
        _as_any_mut: unsafe fn(&mut MemberBase) -> &mut dyn Any,
        _as_member: unsafe fn(&MemberBase) -> &dyn Member,
        _as_member_mut: unsafe fn(&mut MemberBase) -> &mut dyn Member,
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
    pub fn as_member(&self) -> &dyn Member {
        unsafe { (self.functions._as_member)(self) }
    }
    #[inline]
    pub fn as_member_mut(&mut self) -> &mut dyn Member {
        unsafe { (self.functions._as_member_mut)(self) }
    }
}
impl<T: MemberInner> HasBase for AMember<T> {
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

impl<T: MemberInner> HasNativeId for AMember<T> {
    #[inline]
    unsafe fn native_id(&self) -> usize {
        self.inner.native_id().into()
    }
}
impl<T: MemberInner> MaybeMember for AMember<T> {
    #[inline]
    default fn is_member(&self) -> Option<&dyn Member> {
        None
    }
    #[inline]
    default fn is_member_mut(&mut self) -> Option<&mut dyn Member> {
        None
    }
}
impl<T: MemberInner> MaybeControl for AMember<T> {
    #[inline]
    default fn is_control(&self) -> Option<&dyn Control> {
        None
    }
    #[inline]
    default fn is_control_mut(&mut self) -> Option<&mut dyn Control> {
        None
    }
}
impl<T: MemberInner> MaybeContainer for AMember<T> {
    #[inline]
    default fn is_container(&self) -> Option<&dyn Container> {
        None
    }
    #[inline]
    default fn is_container_mut(&mut self) -> Option<&mut dyn Container> {
        None
    }
}
impl<T: MemberInner> MaybeHasSize for AMember<T> {
    default fn is_has_size(&self) -> Option<&dyn HasSize> {
        None
    }
    default fn is_has_size_mut(&mut self) -> Option<&mut dyn HasSize> {
        None
    }
}
impl<T: MemberInner> MaybeHasVisibility for AMember<T> {}

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
impl<T: MemberInner> HasInner for AMember<T> {
    type Inner = T;
    type Params = MemberFunctions;

    #[inline]
    fn with_inner(inner: Self::Inner, params: Self::Params) -> Self {
        AMember {
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
impl<T: MemberInner> Sealed for AMember<T> {}

