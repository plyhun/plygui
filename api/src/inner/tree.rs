use crate::types;

use super::auto::{HasInner, Abstract, Spawnable};
use super::container::AContainer;
use super::item_clickable::{ItemClickable, ItemClickableInner};
use super::adapted::{AAdapted, Adapted, AdaptedInner};
use super::control::{AControl, Control, ControlInner};
use super::member::{AMember, Member};

define! {
    Tree: Control + Adapted + ItemClickable {
	    /*base: {
            pub on_item_click: Option<OnItemClick>,
        }*/
	    inner: {}
	    outer: {}
	    constructor: {
    	    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Tree>;
	    }
	    extends: { ItemClickable }
    }
}

/*impl<T: TreeInner + 'static> ATree<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        Self { base: TreeBase { on_item_click: None }, inner }
    }
}*/
impl<II: TreeInner, T: HasInner<I = II> + Abstract + 'static> TreeInner for T {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Tree> {
        <<Self as HasInner>::I as TreeInner>::with_adapter(adapter)
    }
}
impl<T: TreeInner> NewTree for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Tree> {
        T::with_adapter(adapter)
    }
}
// hello E0119
/*impl<T: TreeInner> ItemClickable for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
    #[inline]
    fn on_item_click(&mut self, cb: Option<OnItemClick>) {
        self.inner.inner.inner.inner.base.on_item_click = cb;
    }
    #[inline]
    fn item_click(&mut self, arg: usize, item_view: &mut dyn Control, skip_callbacks: bool) {
        if !skip_callbacks{
            let self2 = self as *mut Self;
            if let Some(ref mut callback) = self.inner.inner.inner.inner.base.on_item_click {
                (callback.as_mut())(unsafe { &mut *self2 }, arg, item_view)
            }
        }
    }
    #[inline]
    fn as_item_clickable(&self) -> &dyn ItemClickable {
        self
    }
    #[inline]
    fn as_item_clickable_mut(&mut self) -> &mut dyn ItemClickable {
        self
    }
    #[inline]
    fn into_item_clickable(self: Box<Self>) -> Box<dyn ItemClickable> {
        self
    }
}*/

impl<T: TreeInner> Tree for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
    #[inline]
    fn as_tree(&self) -> &dyn Tree {
        self
    }
    #[inline]
    fn as_tree_mut(&mut self) -> &mut dyn Tree {
        self
    }
    #[inline]
    fn into_tree(self: Box<Self>) -> Box<dyn Tree> {
        self
    }
}

impl<T: TreeInner> Spawnable for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
    fn spawn() -> Box<dyn Control> {
        <T as Spawnable>::spawn()
    }
}
