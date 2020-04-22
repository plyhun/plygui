use super::auto::{AsAny};
use super::adapted::{Adapted, AdapterInnerCallback};
use super::control::{Control};

pub trait AdapterInner: 'static {
    fn on_item_change(&mut self, cb: Option<AdapterInnerCallback>);
}

pub trait Adapter: AdapterInner + AsAny {
	fn len(&self) -> usize;
	fn spawn_item_view(&mut self, indexes: &[usize], node: AdapterNode, parent: &dyn Adapted) -> Box<dyn Control>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AdapterNode {
	Leaf,
	Branch(bool),
}
