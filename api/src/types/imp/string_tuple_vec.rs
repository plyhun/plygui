use crate::controls::{Adapted, Control, HasLabel};
use crate::types::{RecursiveTupleVecIterator};
use crate::sdk;
use crate::types::{adapter, Adapter, AsAny, RecursiveTupleVec, Spawnable, VecItemChangeOption};
use std::any::Any;
use std::marker::PhantomData;
use std::usize;

pub struct StringTupleVecAdapter<C: HasLabel + Spawnable> {
    item: RecursiveTupleVec<String>,
    on_item_change: Option<sdk::AdapterInnerCallback>,
    _marker: PhantomData<C>,
}
impl<C: HasLabel + Spawnable> StringTupleVecAdapter<C> {
	pub fn new() -> Self {
        Self::from(RecursiveTupleVec::default())
    }
    pub fn put<'a, 'b: 'a>(&'a mut self, indexes: &'b [usize], value: VecItemChangeOption<RecursiveTupleVec<String>>) -> Result<Option<RecursiveTupleVec<String>>, &'b [usize]> {
    	let value_some = value.is_some();
        let value_is_branch = if value_some { value.as_ref().value.is_some() } else { false };
        let ret = self.item.put(indexes, value);
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
        	if ret.is_ok() {
        		let change = match ret.as_ref().unwrap() {
        			Some(_) if value_some => adapter::Change::Edited(indexes, if value_is_branch {adapter::Node::Branch(false)} else {adapter::Node::Leaf}),
        			Some(_) => adapter::Change::Removed(indexes),
        			None if value_some => adapter::Change::Added(indexes, if value_is_branch {adapter::Node::Branch(false)} else {adapter::Node::Leaf}),
        			_ => adapter::Change::Edited(indexes, adapter::Node::Leaf),
        		};
        		cb.on_item_change(change);
        	}            
        }
        ret
    }
    pub fn get_mut<'a, 'b: 'a>(&'a mut self, indexes: &'b [usize]) -> Option<&'a mut RecursiveTupleVec<String>> {
        self.item.get_mut(indexes)
    }
    pub fn get<'a, 'b: 'a>(&'a self, indexes: &'b [usize]) -> Option<&'a RecursiveTupleVec<String>> {
	    self.item.get(indexes)    
    }
}
impl<C: HasLabel + Spawnable> From<RecursiveTupleVec<String>> for StringTupleVecAdapter<C> {
    fn from(a: RecursiveTupleVec<String>) -> Self {
        StringTupleVecAdapter {
            item: a,
            on_item_change: None,
            _marker: PhantomData,
        }
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
    fn len_at(&self, indexes: &[usize]) -> Option<usize> {
        if indexes.len() == 0 {
            if let Some(ref value) = self.item.value {
                Some(value.len())
            } else {
                None
            }
        } else {
            self.item.get(indexes).and_then(|n| n.value.as_ref()).map(|n| n.len())
        }
    }
    fn node_at(&self, indexes: &[usize]) -> Option<adapter::Node> {
        self.item.get(indexes).map(|n| if n.value.is_some() { adapter::Node::Branch(true) } else { adapter::Node::Leaf })
    }
    fn spawn_item_view(&mut self, indexes: &[usize], _: &dyn Adapted) -> Option<Box<dyn Control>> {
        self.item.get(indexes).map(|n| {
            let mut control = C::spawn();
            control.as_any_mut().downcast_mut::<C>().unwrap().set_label(n.id.as_str().into());

            control
        })
    }
    fn for_each<'a, 'b:'a, 'c: 'b>(&'c self, f: &'a mut dyn adapter::FnNodeItem) {
        let mut iterator = RecursiveTupleVecIterator::with_item(&self.item);
        while let Some((indexes, ref node, _item)) = iterator.next() {
            f(indexes, node);
        }
    }
}
impl<C: HasLabel + Spawnable> sdk::AdapterInner for StringTupleVecAdapter<C> {
    fn on_item_change(&mut self, cb: Option<sdk::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
