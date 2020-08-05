use crate::controls::{Adapted, Control, HasLabel};
use crate::types::{RecursiveTupleVecIterator, RecursiveTupleVecIteratorWrapper};
use crate::sdk;
use crate::types::{adapter, Adapter, AdapterIterator, AsAny, RecursiveTupleVec, Spawnable};
use std::any::Any;
use std::marker::PhantomData;
use std::usize;

pub struct StringTupleVecAdapter<C: HasLabel + Spawnable> {
    item: RecursiveTupleVec<String>,
    on_item_change: Option<sdk::AdapterInnerCallback>,
    _marker: PhantomData<C>,
}
impl<'a, C: HasLabel + Spawnable> IntoIterator for &'a StringTupleVecAdapter<C> {
    type Item = (&'a [usize], adapter::Node, &'a String);
    type IntoIter = RecursiveTupleVecIterator<'a, String>;

    fn into_iter(self) -> Self::IntoIter {
        RecursiveTupleVecIterator::with_item(&self.item)
    }
}

impl<C: HasLabel + Spawnable> StringTupleVecAdapter<C> {
    pub fn new() -> Self {
        Self::from(RecursiveTupleVec::default())
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
    unsafe fn into_items_indexes_iter<'a>(&'a self) -> Box<dyn AdapterIterator<'a> + 'a> {
        Box::new(
            RecursiveTupleVecIteratorWrapper {
                inner: IntoIterator::into_iter(self)
            }
        )
    }
}
impl<C: HasLabel + Spawnable> sdk::AdapterInner for StringTupleVecAdapter<C> {
    fn on_item_change(&mut self, cb: Option<sdk::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
