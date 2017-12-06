pub use layout::*;

#[derive(Debug, Clone)]
pub struct LayoutBase {
	pub width: Size,
	pub height: Size,
	pub gravity: Gravity,
	pub orientation: Orientation,
	pub alignment: Alignment,
}