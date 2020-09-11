use crate::controls::{Adapted, Control, HasLabel};
use crate::sdk;
use crate::types::{adapter, AsAny, Adapter, Spawnable};
use std::any::Any;
use std::marker::PhantomData;

pub struct StringVecAdapter<C: HasLabel + Spawnable> {
    items: Vec<String>,
    on_item_change: Option<sdk::AdapterInnerCallback>,
    _marker: PhantomData<C>,
}

impl<C: HasLabel + Spawnable> From<Vec<String>> for StringVecAdapter<C> {
    fn from(a: Vec<String>) -> Self {
        StringVecAdapter { items: a, on_item_change: None, _marker: PhantomData }
    }
}
impl<C: HasLabel + Spawnable> StringVecAdapter<C> {
    pub fn new() -> Self {
        Self::from(Vec::new())
    }
    pub fn with_iterator<'a, T, I>(i: I) -> Self where T: AsRef<str>, I: Iterator<Item=T> {
        let mut t = Self::new();
        for item in i {
            t.items.push(String::from(item.as_ref()));
        }
        t
    }
    pub fn with_into_iterator<'a, T, I>(i: I) -> Self where T: AsRef<str>, I: IntoIterator<Item=T> {
        Self::with_iterator(i.into_iter())
    }

    pub fn text_at(&self, i: usize) -> Option<&String> {
        self.items.get(i)    
    }
    pub fn text_at_mut(&mut self, i: usize) -> Option<&mut String> {
        self.items.get_mut(i)    
    }
    pub fn push<T: AsRef<str>>(&mut self, arg: T) {
        let i = self.items.len();
        self.items.push(String::from(arg.as_ref()));
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            cb.on_item_change(adapter::Change::Added(&[i], adapter::Node::Leaf))
        }
    }
    pub fn pop(&mut self) -> Option<String> {
        let t = self.items.pop();
        let i = self.items.len();
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            cb.on_item_change(adapter::Change::Removed(&[i]))
        }
        t
    }
}
impl<C: HasLabel + Spawnable> AsAny for StringVecAdapter<C> {
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
impl<C: HasLabel + Spawnable> Adapter for StringVecAdapter<C> {
    fn len_at(&self, indexes: &[usize]) -> Option<usize> {
        if indexes.len() == 0 {
            Some(self.items.len())
        } else {
            None
        }
    }
    fn node_at(&self, indexes: &[usize]) -> Option<adapter::Node> {
        if indexes.len() == 1 {
            Some(adapter::Node::Leaf)
        } else {
            None
        }
    }
	fn spawn_item_view(&mut self, indexes: &[usize], _parent: &dyn Adapted) -> Option<Box<dyn Control>> {
	    if indexes.len() == 1 {
	        let mut control = C::spawn();
    	    control.as_any_mut().downcast_mut::<C>().unwrap().set_label(self.items[indexes[0]].as_str().into());
        	Some(control)
        } else {
            None
        }
	}
	fn for_each<'a, 'b:'a, 'c: 'b>(&'c self, f: &'a mut dyn adapter::FnNodeItem) {
	    let mut iter = self.items.iter().enumerate();
	    while let Some((index, _item)) = iter.next() {
	        f(&[index], &adapter::Node::Leaf);
	    }
	}
}
impl<C: HasLabel + Spawnable> sdk::AdapterInner for StringVecAdapter<C> {
    fn on_item_change(&mut self, cb: Option<sdk::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
