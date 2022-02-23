use crate::controls::{Adapted, Control, HasLabel};
use crate::sdk;
use crate::types::{adapter, AsAny, Adapter, Spawnable};
use std::any::Any;
use std::marker::PhantomData;

pub struct StringTableAdapterColumn {
    pub label: Option<String>,
    pub cells: Vec<Option<String>>,
}

pub struct StringTableAdapter<C: HasLabel + Spawnable> {
    columns: Vec<StringTableAdapterColumn>,
    on_item_change: Option<sdk::AdapterInnerCallback>,
    _marker: PhantomData<C>,
}

impl<C: HasLabel + Spawnable> From<Vec<StringTableAdapterColumn>> for StringTableAdapter<C> {
    fn from(a: Vec<StringTableAdapterColumn>) -> Self {
        StringTableAdapter { columns: a, on_item_change: None, _marker: PhantomData }
    }
}
impl<C: HasLabel + Spawnable> StringTableAdapter<C> {
    pub fn empty() -> Self {
        Self::from(Vec::new())
    }
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        let mut t = Self::from(Vec::with_capacity(width));
        for _ in 0..width {
            let mut col = StringTableAdapterColumn {
                cells:  Vec::with_capacity(height),
                label: None,
            };
            for _ in 0..height {
                col.cells.push(None);
            }
            t.columns.push(col);
        }
        t
    }
    pub fn with_column_iterator<'a, T, I>(i: I, height: usize) -> Self where T: AsRef<str>, I: Iterator<Item=T> {
        let mut t = Self::empty();
        for label in i {
            let mut col = StringTableAdapterColumn {
                cells:  Vec::with_capacity(height),
                label: Some(label.as_ref().into()),
            };
            for _ in 0..height {
                col.cells.push(None);
            }
            t.columns.push(col);
        }
        t
    }
    pub fn with_matrix<'a, M, R, S>(a: M) -> Self where M: AsRef<[R]>, R: AsRef<[S]>, S: AsRef<str> {
        let mut v = Vec::with_capacity(a.as_ref().len());
        for row in a.as_ref() {
            let mut col = StringTableAdapterColumn {
                cells:  Vec::with_capacity(row.as_ref().len()),
                label: None,
            };
            for cell in row.as_ref() {
                col.cells.push(Some(cell.as_ref().into()));
            }
            v.push(col);
        }
        Self::from(v)
    }
    pub fn with_into_iterator<'a, T, I, II>(i: II) -> Self where T: AsRef<str>, I: IntoIterator<Item=T>, II: IntoIterator<Item=I> {
        let mut t = Self::empty();
        for row in i.into_iter() {
            let mut col = StringTableAdapterColumn {
                cells:  Vec::new(),
                label: None,
            };
            for cell in row.into_iter() {
                col.cells.push(Some(cell.as_ref().into()));
            }
            t.columns.push(col);
        }
        t
    }
    pub fn column_at(&self, x: usize) -> Option<&StringTableAdapterColumn> {
        self.columns.get(x)
    }
    pub fn column_at_mut(&mut self, x: usize) -> Option<&mut StringTableAdapterColumn> {
        self.columns.get_mut(x)
    }
    pub fn set_column_label_at<T: AsRef<str>>(&mut self, arg: Option<T>, x: usize) {
        let added = arg.is_some();
        let existed = self.columns.get_mut(x).is_some();
        self.columns[x].label = arg.map(|arg| arg.as_ref().into());
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            let indices = &[x];
            if !added && !existed {
                return
            }
            if added && existed {
                cb.on_item_change(adapter::Change::Edited(indices, adapter::Node::Branch(true)))
            } else if added {
                cb.on_item_change(adapter::Change::Added(indices, adapter::Node::Branch(true)))
            } else {
                cb.on_item_change(adapter::Change::Removed(indices))
            }
        }
    }
    pub fn text_at(&self, x: usize, y: usize) -> Option<&String> {
        self.columns.get(x).and_then(|col| col.cells.get(y)).and_then(|cell| cell.as_ref())
    }
    pub fn text_at_mut(&mut self, x: usize, y: usize) -> Option<&mut String> {
        self.columns.get_mut(x).and_then(|col| col.cells.get_mut(y)).and_then(|cell| cell.as_mut())
    }
    pub fn set_text_at<T: AsRef<str>>(&mut self, arg: Option<T>, x: usize, y: usize) {
        let added = arg.is_some();
        let existed = self.columns.get_mut(x).and_then(|col| col.cells.get_mut(y)).is_some();
        self.columns[x].cells[y] = arg.map(|arg| arg.as_ref().into());
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            let indices = &[x, y];
            if !added && !existed {
                return
            }
            if added && existed {
                cb.on_item_change(adapter::Change::Edited(indices, adapter::Node::Leaf))
            } else if added {
                cb.on_item_change(adapter::Change::Added(indices, adapter::Node::Leaf))
            } else {
                cb.on_item_change(adapter::Change::Removed(indices))
            }
        }
    }
}
impl<C: HasLabel + Spawnable> AsAny for StringTableAdapter<C> {
    #[inline]
    fn as_any(&self) -> &dyn Any {
        self
    }
    #[inline]
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
    #[inline]
    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }
}
impl<C: HasLabel + Spawnable> Adapter for StringTableAdapter<C> {
    fn len_at(&self, indexes: &[usize]) -> Option<usize> {
        if indexes.len() == 0 {
            Some(self.columns.len())
        } else if indexes.len() == 1 {
            self.columns.get(indexes[0]).map(|col| col.cells.len())
        } else {
            None
        }
    }
    fn node_at(&self, indexes: &[usize]) -> Option<adapter::Node> {
        if indexes.len() == 1 {
            Some(adapter::Node::Branch(true))
        } else if indexes.len() == 2 {
            Some(adapter::Node::Leaf)
        } else {
            None
        }
    }
	fn spawn_item_view(&mut self, indexes: &[usize], _parent: &dyn Adapted) -> Option<Box<dyn Control>> {
	    if indexes.len() == 2 {
	        self.columns[indexes[0]].cells[indexes[1]].as_ref().map(|str| {
    	        let mut control = C::spawn();
        	    control.as_any_mut().downcast_mut::<C>().unwrap().set_label(str.as_str().into());
            	control
	        })
        } else {
            None
        }
	}
	fn for_each<'a, 'b:'a, 'c: 'b>(&'c self, f: &'a mut dyn adapter::FnNodeItem) {
	    let mut cols = self.columns.iter().enumerate();
	    while let Some((x, column)) = cols.next() {
	        f(&[x], &adapter::Node::Branch(true));
	        let mut cells = column.cells.iter().enumerate();
	        while let Some((y, _cell)) = cells.next() {
	            f(&[x, y], &adapter::Node::Leaf);
	        }
	    }
	}
}
impl<C: HasLabel + Spawnable> sdk::AdapterInner for StringTableAdapter<C> {
    fn on_item_change(&mut self, cb: Option<sdk::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
