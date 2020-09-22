use crate::controls;

pub mod imp;

pub mod adapter {
	pub use crate::inner::adapter::{Node, Change, FnNodeItem};
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
                if indexes.len() == 1 {
                    return Some(&mut ivalue[indexes[0]]);
                } else {
                    return Self::get_mut_inner(ivalue[indexes[0]].value.as_mut(), &indexes[1..]);
                }
            }
        }
        None
    }
    pub fn get_mut<'a, 'b: 'a>(&'a mut self, indexes: &'b [usize]) -> Option<&'a mut RecursiveTupleVec<K>> {
        if indexes.len() == 0 {
            Some(self)
        } else {
            Self::get_mut_inner(self.value.as_mut(), indexes)
        }
    }
    
    fn get_inner<'a, 'b: 'a>(value: Option<&'a Vec<RecursiveTupleVec<K>>>, indexes: &'b [usize]) -> Option<&'a RecursiveTupleVec<K>> {
        if indexes.len() < 1 {
            return None;
        }
        if let Some(ivalue) = value {
            let len = ivalue.len();
            if indexes[0] < len {
                if indexes.len() == 1 {
                    return Some(&ivalue[indexes[0]]);
                } else {
                    return Self::get_inner(ivalue[indexes[0]].value.as_ref(), &indexes[1..]);
                }
            }
        }
        None
    }
    pub fn get<'a, 'b: 'a>(&'a self, indexes: &'b [usize]) -> Option<&'a RecursiveTupleVec<K>> {
        if indexes.len() == 0 {
            Some(self)
        } else {
            Self::get_inner(self.value.as_ref(), indexes)
        }
    }
}

#[derive(Debug, Clone, )]
enum RecursiveTupleVecIteratorStatus {
    Created,
    Root,
    Node(usize),
    Branch(usize),
}
struct RecursiveTupleVecIterator<'a, K: Sized> {
    status: RecursiveTupleVecIteratorStatus,
    indexes: Vec<usize>,
    lengths: Vec<usize>,
    item: &'a RecursiveTupleVec<K>
}
impl<'a, K: Sized> RecursiveTupleVecIterator<'a, K> {
    pub fn with_item(item: &'a RecursiveTupleVec<K>) -> Self {
        RecursiveTupleVecIterator {
            status: RecursiveTupleVecIteratorStatus::Created,
            indexes: vec![],
            lengths: vec![],
            item: item,
        }
    }
    fn node(&self) -> Option<adapter::Node> {
        self.item.get(&self.indexes).map(|n| if n.value.is_some() { adapter::Node::Branch(true) } else { adapter::Node::Leaf })
    }
    fn get(&mut self) -> Option<(&[usize], adapter::Node, &'a K)> {
        match self.status {
            RecursiveTupleVecIteratorStatus::Branch(_) | RecursiveTupleVecIteratorStatus::Node(_) if self.indexes.len() == 0 => {
                None
            }
            _ => {
                let indexes = unsafe { ::std::mem::transmute(self.indexes.as_slice()) };
                let maybe_r = self.item.get(indexes);
                if let Some(r) = maybe_r {
                    //self.len = if let Some(ref r) = r.value { r.len() } else { 0 };    
                    let node = self.node().unwrap();
                    self.status = if let Some(ref value) = r.value {
                        RecursiveTupleVecIteratorStatus::Branch(value.len())
                    } else {
                        RecursiveTupleVecIteratorStatus::Node(self.lengths[self.lengths.len()-1])
                    };
                    Some((self.indexes.as_slice(), node, &r.id))
                } else { 
                    None
                }
            }
        }
    }
    pub fn next(&mut self) -> Option<(&[usize], adapter::Node, &'a K)> {
        let ilen = self.indexes.len();
        match self.status {
            RecursiveTupleVecIteratorStatus::Created => {
                self.status = RecursiveTupleVecIteratorStatus::Root;
                Some((&[], self.node().unwrap(), &self.item.id))
            },
            RecursiveTupleVecIteratorStatus::Root => {
                self.status = if let Some(ref value) = self.item.value {
                    self.indexes.push(0);
                    self.lengths.push(value.len());
                    RecursiveTupleVecIteratorStatus::Branch(value.len())
                } else {
                    RecursiveTupleVecIteratorStatus::Node(0)
                };
                self.get()
            },
            RecursiveTupleVecIteratorStatus::Node(len) => {
                let ilen1 = ilen-1;
                if self.indexes[ilen1]+1 >= len {
                    self.indexes.pop();
                    self.lengths.pop();
                    if self.indexes.len() > 0 {
                        self.indexes[ilen1-1] += 1; 
                    }
                    self.status = if self.lengths.len() > 0 && self.lengths[self.lengths.len()-1] > 0 {
                        RecursiveTupleVecIteratorStatus::Branch(self.lengths[self.lengths.len()-1])
                    } else {
                        RecursiveTupleVecIteratorStatus::Node(0)
                    };
                } else {
                    self.status = RecursiveTupleVecIteratorStatus::Node(len-1);
                    self.indexes[ilen1] += 1; 
                }
                self.get()
            },
            RecursiveTupleVecIteratorStatus::Branch(len) => {
                let indexes = unsafe { ::std::mem::transmute(self.indexes.as_slice()) };
                if let Some(r) = self.item.get(indexes) {
                    if let Some(ref value) = r.value {
                        self.indexes.push(0);
                        self.lengths.push(value.len());
                    }
                }
                self.status = RecursiveTupleVecIteratorStatus::Node(len);
                self.get()
            }
        }
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
