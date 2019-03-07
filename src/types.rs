#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WindowStartSize {
    Exact(u16, u16),
    Fullscreen,
}

pub type Menu = Option<Vec<MenuItem>>;

#[derive(Debug, PartialEq)]
pub enum MenuItem {
	Action(String, crate::callbacks::Action, MenuItemRole),
	Delimiter,
	Sub(String, Vec<MenuItem>, MenuItemRole),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemRole {
	None,
	Options,
	Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageSeverity {
    Info,
    Warning,
    Alert,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextContent {
    Plain(String),
    LabelDescription(String, String),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageScalePolicy {
    CropCenter, // TODO variants
    FitCenter,  // TODO variants
                // TODO Tile
}
