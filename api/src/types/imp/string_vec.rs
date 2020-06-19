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

impl<C: HasLabel + Spawnable> StringVecAdapter<C> {
    pub fn new() -> Self {
        StringVecAdapter { items: Vec::new(), on_item_change: None, _marker: PhantomData }
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
    fn len(&self) -> usize {
        self.items.len()
    }
    fn node_at(&self, _: usize) -> adapter::Node {
        adapter::Node::Leaf
    }
	fn spawn_item_view(&mut self, indexes: &[usize], _node: adapter::Node, _parent: &dyn Adapted) -> Box<dyn Control> {
	    let mut control = C::spawn();
	    control.as_any_mut().downcast_mut::<C>().unwrap().set_label(self.items[indexes[0]].as_str().into());
    	control
	}
}
impl<C: HasLabel + Spawnable> sdk::AdapterInner for StringVecAdapter<C> {
    fn on_item_change(&mut self, cb: Option<sdk::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
