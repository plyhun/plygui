use super::ids::Id;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Size {
    MatchParent,
    WrapContent,
    Exact(u16),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Alignment {
	None,
    Above(Id),
    Below(Id),
    ToLeftOf(Id),
    ToRightOf(Id),
    AlignTop(Id),
    AlignBottom(Id),
    AlignLeft(Id),
    AlignRight(Id),
    AlignParentLeft,
    AlignParentRight,
    AlignParentTop,
    AlignParentBottom,
}


#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub type Gravity = u8;
pub mod gravity {
	pub const CENTER: super::Gravity = 0;
	pub const TOP: super::Gravity = 1 << 0;
	pub const BOTTOM: super::Gravity = 1 << 1;
	pub const LEFT: super::Gravity = 1 << 2;
	pub const RIGHT: super::Gravity = 1 << 3;
	pub const CENTER_HORIZONTAL: super::Gravity = 1 << 4;
	pub const CENTER_VERTICAL: super::Gravity = 1 << 5;
	pub const START: super::Gravity = 1 << 6;
	pub const END: super::Gravity = 1 << 7;
}

#[derive(Debug, Clone, Builder)]
pub struct Attributes {
	pub width: Size,
	pub height: Size,
	pub gravity: Gravity,
	pub orientation: Orientation,
	pub alignment: Alignment,
}
