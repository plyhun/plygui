use crate::types::{adapter, AsAny, Adapter, Spawnable, RecursiveTupleVec};
use crate::sdk;
use crate::controls::{Adapted, Control, HasLabel};
use std::any::Any;
use std::marker::PhantomData;

pub struct StringTupleVecAdapter<C: HasLabel + Spawnable> {
    items: Vec<RecursiveTupleVec<String>>,
    on_item_change: Option<sdk::AdapterInnerCallback>,
    _marker: PhantomData<C>,
}
impl <C: HasLabel + Spawnable> StringTupleVecAdapter<C> {
    pub fn new() -> Self {
        Self::from(Vec::new())
    }
}
impl<C: HasLabel + Spawnable> From<Vec<RecursiveTupleVec<String>>> for StringTupleVecAdapter<C> {
    fn from(a: Vec<RecursiveTupleVec<String>>) -> Self {
        StringTupleVecAdapter { items: a, on_item_change: None, _marker: PhantomData }
    }
}
impl<C: HasLabel + Spawnable> AsAny for StringTupleVecAdapter<C> {
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
impl<C: HasLabel + Spawnable> Adapter for StringTupleVecAdapter<C> {
    fn len_at(&self, indexes: &[usize]) -> usize {
        if indexes.len() == 0 {
            self.items.len()
        } else {
            if let Some(item) = RecursiveTupleVec::get_at_vec(&self.items, indexes) {
                item.value.as_ref().unwrap().len() // TODO
            } else {
                0
            }
        }
    }
    fn node_at(&self, indexes: &[usize]) -> adapter::Node {
        if RecursiveTupleVec::get_at_vec(&self.items, indexes).is_some() {
            adapter::Node::Branch(true)
        } else {
            adapter::Node::Leaf
        }
    }
	fn spawn_item_view(&mut self, indexes: &[usize], _node: adapter::Node, _parent: &dyn Adapted) -> Box<dyn Control> {
	    let mut control = C::spawn();
	    if let Some(item) = RecursiveTupleVec::get_mut_at_vec(&mut self.items, indexes) {
    	    control.as_any_mut().downcast_mut::<C>().unwrap().set_label(item.id.as_str().into());
	    }
    	control
	}
}
impl<C: HasLabel + Spawnable> sdk::AdapterInner for StringTupleVecAdapter<C> {
    fn on_item_change(&mut self, cb: Option<sdk::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
