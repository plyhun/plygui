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
    fn add_column_inner(&mut self) {
        self.insert_column(self.columns.len());
    }
    pub fn insert_column(&mut self, col: usize) {
        let mut column = StringTableAdapterColumn {
            cells:  Vec::new(),
            label: None,
        };
        self.column_at_mut(0).map(|column_| {
            (0..column_.cells.len()).into_iter().for_each(|_| column.cells.push(None));
        });
        self.columns.insert(col, column);
        self.on_item_change.as_mut().map(|ref mut cb| {
            cb.on_item_change(adapter::Change::Added(&[col], adapter::Node::Branch(true)))
        });
    }
    pub fn remove_column(&mut self, col: usize) {
        if col >= self.columns.len() {
            return;
        }
        (0..self.columns[col].cells.len()).into_iter().for_each(|row| {
            self.on_item_change.as_mut().map(|ref mut cb| {
                cb.on_item_change(adapter::Change::Removed(&[row, col]));
            });
        });
        self.on_item_change.as_mut().map(|ref mut cb| {
            cb.on_item_change(adapter::Change::Removed(&[col]));
        });
        self.columns.remove(col);
    }
    pub fn insert_row(&mut self, index: usize) {
        self.columns.iter_mut().for_each(|col| {
            col.cells.insert(index, None);
        });
        (0..self.columns.len()).into_iter().for_each(|i| {
            self.on_item_change.as_mut().map(|cb| {
                cb.on_item_change(adapter::Change::Added(&[index, i], adapter::Node::Leaf))
            });
        });
    }
    pub fn remove_row(&mut self, index: usize) {
        let (_,h) = self.dimensions();
        if index >= h {
            return;
        }
        (0..self.columns.len()).into_iter().for_each(|i| {
            self.on_item_change.as_mut().map(|cb| {
                cb.on_item_change(adapter::Change::Removed(&[i,index]))
            });
        });
        self.columns.iter_mut().for_each(|col| {
            col.cells.remove(index);
        });
    }
    pub fn dimensions(&self) -> (usize, usize) {
        self.column_at(0).map_or((0, 0), |col| col.cells.get(0).map_or((0, 0), |_| (self.columns.len(), col.cells.len())))
    }
    pub fn set_dimensions(&mut self, width: usize, height: usize) {
        let (w,h) = self.dimensions();
        if width > w {
            (w..width).into_iter().for_each(|i| {
                self.insert_column(i);
            });
        } else if width < w {
            (width..w).into_iter().for_each(|i| {
                self.remove_column(i);
            });
        }
        if height > h {
            (h..height).into_iter().for_each(|j| {
                self.insert_row(j);
            });
        } else if height < h {
            (height..h).into_iter().for_each(|j| {
                self.remove_row(j);
            });
        }
    }
    pub fn empty() -> Self {
        Self::from(Vec::new())
    }
    pub fn with_dimensions(width: usize, height: usize) -> Self {
        let mut t = Self::from(Vec::with_capacity(width));
        for col in 0..width {
            t.add_column_inner();
            t.column_at_mut(col).map(|column| {
                for _ in 0..height {
                    column.cells.push(None);
                }
            });
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
    pub fn column_at(&self, col: usize) -> Option<&StringTableAdapterColumn> {
        self.columns.get(col)
    }
    pub fn column_at_mut(&mut self, col: usize) -> Option<&mut StringTableAdapterColumn> {
        self.columns.get_mut(col)
    }
    pub fn set_column_label_at<T: AsRef<str>>(&mut self, arg: Option<T>, col: usize) {
        let added = arg.is_some();
        let existed = self.columns.get_mut(col).is_some();
        self.columns[col].label = arg.map(|arg| arg.as_ref().into());
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            if !added && !existed {
                return
            }
            let indices = &[col];
            if added && existed {
                cb.on_item_change(adapter::Change::Edited(indices, adapter::Node::Branch(true)))
            } else if added {
                cb.on_item_change(adapter::Change::Added(indices, adapter::Node::Branch(true)))
            } else {
                cb.on_item_change(adapter::Change::Removed(indices))
            }
        }
    }
    pub fn text_at(&self, row: usize, col: usize) -> Option<&String> {
        self.columns.get(col).and_then(|col| col.cells.get(row)).and_then(|cell| cell.as_ref())
    }
    pub fn text_at_mut(&mut self, row: usize, col: usize) -> Option<&mut String> {
        self.columns.get_mut(col).and_then(|col| col.cells.get_mut(row)).and_then(|cell| cell.as_mut())
    }
    pub fn set_text_at<T: AsRef<str>>(&mut self, arg: Option<T>, row: usize, col: usize) {
        let added = arg.is_some();
        let existed = self.columns.get_mut(col).and_then(|column| column.cells.get_mut(row)).is_some();
        self.columns[col].cells[row] = arg.map(|arg| arg.as_ref().into());
        if let Some(ref mut cb) = self.on_item_change.as_mut() {
            if !added && !existed {
                return
            }
            let indices = &[row, col];
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
            Some(self.column_at(0).map(|column| column.cells.len()).unwrap_or(0))
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
	    self.alt_text_at(indexes).map(|str| {
    	        let mut control = C::spawn();
        	    control.as_any_mut().downcast_mut::<C>().unwrap().set_label(str.into());
            	control
	        })
	}
	fn for_each<'a, 'b:'a, 'c: 'b>(&'c self, f: &'a mut dyn adapter::FnNodeItem) {
	    for (x, _) in self.columns.iter().enumerate() {
	        f(&[x], &adapter::Node::Branch(true));
	    }
	    for (col, column) in self.columns.iter().enumerate() {
	        let mut cells = column.cells.iter().enumerate();
	        while let Some((row, _cell)) = cells.next() {
                dbg!(row, col);
	            f(&[row, col], &adapter::Node::Leaf);
	        }
	    }
	}
	fn alt_text_at<'a, 'b: 'a>(&'a self, indexes: &'b [usize]) -> Option<&'a str> {
    	if indexes.len() == 2 {
	        self.columns[indexes[1]].cells[indexes[0]].as_ref().map(|text| text.as_str())
        } else if indexes.len() == 1 {
            self.columns[indexes[0]].label.as_ref().map(|label| label.as_str())
        } else {
            None
        }
	}
}
impl<C: HasLabel + Spawnable> sdk::AdapterInner for StringTableAdapter<C> {
    fn on_item_change(&mut self, cb: Option<sdk::AdapterInnerCallback>) {
        self.on_item_change = cb;
    }
}
