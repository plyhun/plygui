use crate::controls;

pub mod imp;

pub mod adapter {
	pub use crate::inner::adapter::{Node, Change};
}

pub use crate::inner::{
    auto::{AsAny, Spawnable},
    adapter::{Adapter},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}
impl Default for Visibility {
    fn default() -> Self {
        Visibility::Visible
    }
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
    None,
}
impl Default for Progress {
    fn default() -> Self {
        Progress::Undefined
    }
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
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FindBy<'a> {
    Id(crate::ids::Id),
    Tag(&'a str),
}

#[derive(Debug, Clone)]
pub struct RecursiveHashMap<K: Sized>{ pub inner: ::std::collections::HashMap<K, Option<RecursiveHashMap<K>>> }

#[derive(Debug, Clone)]
pub struct RecursiveTupleVec<K: Sized>{ pub id: K, value: Option<Vec<RecursiveTupleVec<K>>> }

impl<K: Sized> RecursiveTupleVec<K> {
    pub fn get_mut(&mut self, indexes: &[usize]) -> Option<&mut K> {
        let mut value = self.value.as_mut();
        if let Some(ivalue) = value {
            let len = ivalue.len();
            for i in 0..indexes.len() {
                if indexes[i] < len {
                    if i == len-1 {
                        return Some(&mut ivalue[indexes[i]].id);
                    } else {
                        value = ivalue[indexes[i]].value.as_mut();
                    }
                }
            }
        }
        None
    }
}

pub enum ApplicationResult {
    New(Box<dyn controls::Application>),
    Existing(Box<dyn controls::Application>),
    ErrorNonUiThread,
    ErrorUnspecified,
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
