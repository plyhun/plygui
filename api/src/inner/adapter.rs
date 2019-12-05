use super::auto::{AsAny};
use super::adapted::{Adapted, AdapterInnerCallback};
use super::control::{Control};

pub trait AdapterInner: 'static {
    fn on_item_change(&mut self, cb: Option<AdapterInnerCallback>);
}

pub trait Adapter: AdapterInner + AsAny {
	fn len(&self) -> usize;
	fn spawn_item_view(&mut self, i: usize, parent: &dyn Adapted) -> Box<dyn Control>;
}
