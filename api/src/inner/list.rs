use super::auto::HasInner;
use super::container::AContainer;
use super::adapted::{AAdapted, Adapted, AdaptedInner};
use super::control::{AControl};
use super::member::{AMember, MemberInner};

define! {
    List: Adapted {}
}

impl<II: ListInner, T: HasInner<I = II> + 'static> ListInner for T {
}

impl<T: ListInner> List for AMember<AControl<AContainer<AAdapted<AList<T>>>>> {
    #[inline]
    fn as_list(&self) -> &dyn List {
        self
    }
    #[inline]
    fn as_list_mut(&mut self) -> &mut dyn List {
        self
    }
    #[inline]
    fn into_list(self: Box<Self>) -> Box<dyn List> {
        self
    }
}

