use crate::controls::Control;

pub trait Adapter {
	fn len(&self) -> usize;
	fn spawn_group_view(&mut self) -> Box<dyn Control>;
	fn spawn_item_view(&mut self) -> Box<dyn Control>;
}