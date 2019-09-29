use crate::{development, controls};

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Progress {
    Value(u32, u32),
    Undefined,
    None
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
#[derive(Debug, Clone, PartialEq)]
pub enum FindBy {
    Id(crate::ids::Id),
    Tag(String)
}

pub enum ApplicationResult {
    New(Box<dyn controls::Application>),
    Existing(Box<dyn controls::Application>),
    ErrorNonUiThread,
    ErrorUnspecified
}
impl ApplicationResult {
    pub fn unwrap(self) -> Box<dyn controls::Application> {
        match self {
            ApplicationResult::New(app) | ApplicationResult::Existing(app) => app,
            ApplicationResult::ErrorNonUiThread => panic!("Application requested from non-UI thread"),
            ApplicationResult::ErrorUnspecified => panic!("Error getting Application"),
        }
    }
}

pub trait Adapter: development::AdapterInner {
	fn len(&self) -> usize;
	
	fn spawn_item_view(&mut self, i: usize, parent: &dyn controls::AdapterView) -> Box<dyn controls::Control>;
}

