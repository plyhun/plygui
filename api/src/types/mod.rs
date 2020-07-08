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
pub struct RecursiveTupleVec<K: Sized>{ pub id: K, pub value: Option<Vec<RecursiveTupleVec<K>>> }

impl<K: Sized + Default> Default for RecursiveTupleVec<K> {
    fn default() -> Self {
        Self {
            id: Default::default(), value: None,
        }
    }
}
impl<K: Sized + Default> RecursiveTupleVec<K> {
    fn put_with_defaults_inner<'a, 'b: 'a>(value: Option<&'a mut Vec<RecursiveTupleVec<K>>>, indexes: &'b [usize], passed: usize, mut new: Option<RecursiveTupleVec<K>>) -> Result<Option<RecursiveTupleVec<K>>, &'b [usize]> {
        if indexes.len() == (passed+1) {
            if let Some(value) = value {
                if value.len() > indexes[passed] {
                    if new.is_some() {
                        if let Some(ref mut new) = new {
                            ::std::mem::swap(new, &mut value[indexes[passed]]);
                        }
                        Ok(new)
                    } else {
                        Ok(Some(value.remove(indexes[passed])))
                    }
                } else {
                    if value.len() == indexes[passed] {
                        if new.is_some() {
                            value.push(new.unwrap());
                        }
                        Ok(None)
                    } else {
                        value.push(Default::default());
                        Self::put_with_defaults_inner(value[indexes[passed]].value.as_mut(), indexes, passed+1, new)
                    }
                }
            } else {
                Err(indexes)
            }
        } else if indexes.len() > (passed+1) {
            if let Some(value) = value {
                Self::put_with_defaults_inner(value[indexes[passed]].value.as_mut(), indexes, passed+1, new)
            } else {
                Err(&indexes[..passed])
            }
        } else {
            panic!("Should not happen: indexes.len({}) < passed=1({})", indexes.len(), (passed+1))
        }
    }
    pub fn put_with_defaults<'a, 'b: 'a>(&'a mut self, indexes: &'b [usize], value: Option<RecursiveTupleVec<K>>) -> Result<Option<RecursiveTupleVec<K>>, &'b [usize]> {
        Self::put_with_defaults_inner(self.value.as_mut(), indexes, 0, value)
    }
}

impl<K: Sized> RecursiveTupleVec<K> {
    pub fn get_mut_at_vec<'a, 'b:'a, A: AsMut<[Self]>>(this: &'a mut A, indexes: &'b [usize]) -> Option<&'a mut RecursiveTupleVec<K>> {
        let this = this.as_mut();
        if indexes.len() == 0 || this.len() == 0 {
            return None;
        }
        let index = indexes[0];
        let valid_index = this.len()-1;
        if valid_index == index { 
            Some(&mut this[index])
        } else if valid_index > index {
            this[index].get_mut(&indexes[1..])
        } else {
            None
        }
    }
    pub fn get_at_vec<'a, 'b:'a, A: AsRef<[Self]>>(this: &'a A, indexes: &'b [usize]) -> Option<&'a RecursiveTupleVec<K>> {
        let this = this.as_ref();
        if indexes.len() == 0 || this.len() == 0 {
            return None;
        }
        let index = indexes[0];
        let valid_index = this.len()-1;
        if valid_index == index { 
            Some(&this[index])
        } else if valid_index > index {
            this[index].get(&indexes[1..])
        } else {
            None
        }
    }
    pub fn with_value(id: K, value: Option<Vec<RecursiveTupleVec<K>>>) -> Self {
        Self { id, value }
    }
    fn put_inner<'a, 'b: 'a>(value: Option<&'a mut Vec<RecursiveTupleVec<K>>>, indexes: &'b [usize], passed: usize, mut new: Option<RecursiveTupleVec<K>>) -> Result<Option<RecursiveTupleVec<K>>, &'b [usize]> {
        if indexes.len() == (passed+1) {
            if let Some(value) = value {
                if value.len() > indexes[passed] {
                    if new.is_some() {
                        if let Some(ref mut new) = new {
                            ::std::mem::swap(new, &mut value[indexes[passed]]);
                        }
                        Ok(new)
                    } else {
                        Ok(Some(value.remove(indexes[passed])))
                    }
                } else {
                    if value.len() == indexes[passed] {
                        if new.is_some() {
                            value.push(new.unwrap());
                        }
                        Ok(None)
                    } else {// here
                        Err(indexes)
                    }
                }
            } else {
                Err(indexes)
            }
        } else if indexes.len() > (passed+1) {
            if let Some(value) = value {
                Self::put_inner(value[indexes[passed]].value.as_mut(), indexes, passed+1, new)
            } else {
                Err(&indexes[..passed])
            }
        } else {
            Err(indexes)
        }
    }
    pub fn put<'a, 'b: 'a>(&'a mut self, indexes: &'b [usize], value: Option<RecursiveTupleVec<K>>) -> Result<Option<RecursiveTupleVec<K>>, &'b [usize]> {
        Self::put_inner(self.value.as_mut(), indexes, 0, value)
    }
    
    fn get_mut_inner<'a, 'b: 'a>(value: Option<&'a mut Vec<RecursiveTupleVec<K>>>, indexes: &'b [usize]) -> Option<&'a mut RecursiveTupleVec<K>> {
        if indexes.len() < 1 {
            return None;
        }
        if let Some(ivalue) = value {
            let len = ivalue.len();
            if indexes[0] < len {
                if len <= 1 {
                    return Some(&mut ivalue[indexes[0]]);
                } else {
                    return Self::get_mut_inner(ivalue[indexes[0]].value.as_mut(), &indexes[1..]);
                }
            }
        }
        None
    }
    pub fn get_mut<'a, 'b: 'a>(&'a mut self, indexes: &'b [usize]) -> Option<&'a mut RecursiveTupleVec<K>> {
        Self::get_mut_inner(self.value.as_mut(), indexes)
    }
    
    fn get_inner<'a, 'b: 'a>(value: Option<&'a Vec<RecursiveTupleVec<K>>>, indexes: &'b [usize]) -> Option<&'a RecursiveTupleVec<K>> {
        if indexes.len() < 1 {
            return None;
        }
        if let Some(ivalue) = value {
            let len = ivalue.len();
            if indexes[0] < len {
                if len <= 1 {
                    return Some(&ivalue[indexes[0]]);
                } else {
                    return Self::get_inner(ivalue[indexes[0]].value.as_ref(), &indexes[1..]);
                }
            }
        }
        None
    }
    pub fn get<'a, 'b: 'a>(&'a self, indexes: &'b [usize]) -> Option<&'a RecursiveTupleVec<K>> {
        Self::get_inner(self.value.as_ref(), indexes)
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
