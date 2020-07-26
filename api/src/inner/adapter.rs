use super::auto::{AsAny};
use super::adapted::{Adapted, AdapterInnerCallback};
use super::control::{Control};

pub trait AdapterInner: 'static {
    fn on_item_change(&mut self, cb: Option<AdapterInnerCallback>);
}

pub trait Adapter: AdapterInner + AsAny {
	fn len_at(&self, indexes: &[usize]) -> Option<usize>;
	fn node_at(&self, indexes: &[usize]) -> Option<Node>;
	fn spawn_item_view(&mut self, indexes: &[usize], parent: &dyn Adapted) -> Option<Box<dyn Control>>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node {
	Leaf,
	Branch(bool),
}
#[derive(Debug, Clone, PartialEq)]
pub enum Change<'a> {
    Added(&'a [usize], Node),
    Removed(&'a [usize]),
    Edited(&'a [usize], Node)
}
