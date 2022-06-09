use crate::types;

use super::auto::{HasInner, Abstract, Spawnable};
use super::container::AContainer;
use super::item_clickable::{ItemClickable, ItemClickableInner};
use super::adapted::{AAdapted, Adapted, AdaptedInner};
use super::control::{AControl, Control, ControlInner};
use super::member::{AMember, Member};

define! {
    Table: Control + Adapted + ItemClickable {
	    /*base: {
            pub on_item_click: Option<OnItemClick>,
        }*/
	    inner: {}
	    outer: {}
	    constructor: {
    	    fn with_adapter_initial_size(adapter: Box<dyn types::Adapter>, width: usize, height: usize) -> Box<dyn Table>;
            fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Table> {
                let width = adapter.len_at(&[]).unwrap_or(0);
                let height = adapter.len_at(&[0]).unwrap_or(0);
                Self::with_adapter_initial_size(adapter, width, height)
            }
	    }
	    inner_constructor_params: {
            width: usize, height: usize
        }
	    extends: { ItemClickable }
    }
}

/*impl<T: TableInner + 'static> ATable<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        Self { base: TableBase { on_item_click: None }, inner }
    }
}*/
impl<II: TableInner, T: HasInner<I = II> + Abstract + 'static> TableInner for T {
    #[inline]
    fn with_adapter_initial_size(adapter: Box<dyn types::Adapter>, width: usize, height: usize) -> Box<dyn Table> {
        <<Self as HasInner>::I as TableInner>::with_adapter_initial_size(adapter, width, height)
    }
}
impl<T: TableInner> NewTable for AMember<AControl<AContainer<AAdapted<ATable<T>>>>> {
    #[inline]
    fn with_adapter_initial_size(adapter: Box<dyn types::Adapter>, width: usize, height: usize) -> Box<dyn Table> {
        T::with_adapter_initial_size(adapter, width, height)
    }
}
// hello E0119
/*impl<T: TableInner> ItemClickable for AMember<AControl<AContainer<AAdapted<ATable<T>>>>> {
    #[inline]
    fn on_item_click(&mut self, cb: Option<OnItemClick>) {
        self.inner.inner.inner.inner.base.on_item_click = cb;
    }
    #[inline]
    fn item_click(&mut self, arg: usize, item_view: &mut dyn Control, skip_callbacks: bool) {
        if !skip_callbacks{
            let self2 = self as *mut Self;
            if let Some(ref mut callback) = self.inner.inner.inner.inner.base.on_item_click {
                (callback.as_mut())(unsafe { &mut *self2 }, arg, item_view)
            }
        }
    }
    #[inline]
    fn as_item_clickable(&self) -> &dyn ItemClickable {
        self
    }
    #[inline]
    fn as_item_clickable_mut(&mut self) -> &mut dyn ItemClickable {
        self
    }
    #[inline]
    fn into_item_clickable(self: Box<Self>) -> Box<dyn ItemClickable> {
        self
    }
}*/

impl<T: TableInner> Table for AMember<AControl<AContainer<AAdapted<ATable<T>>>>> {
    #[inline]
    fn as_table(&self) -> &dyn Table {
        self
    }
    #[inline]
    fn as_table_mut(&mut self) -> &mut dyn Table {
        self
    }
    #[inline]
    fn into_table(self: Box<Self>) -> Box<dyn Table> {
        self
    }
}

impl<T: TableInner> Spawnable for AMember<AControl<AContainer<AAdapted<ATable<T>>>>> {
    fn spawn() -> Box<dyn Control> {
        <T as Spawnable>::spawn()
    }
}
pub struct TableData<T: Sized> {
    pub cols: Vec<TableColumn<T>>,
}
pub struct TableColumn<T: Sized> {
    pub cells: Vec<Option<TableCell<T>>>,
    pub control: Option<Box<dyn Control>>,
    pub native: T,
}
pub struct TableCell<T: Sized> {
    pub control: Option<Box<dyn Control>>,
    pub native: T,
}
impl<T: Sized> Default for TableData<T> {
    fn default() -> Self {
        Self { cols: Vec::new() }
    }
}
impl<T: Sized> TableData<T> {
	pub fn cell_at<I: AsRef<[usize]>>(&self, index: I) -> Option<&TableCell<T>> {
		let index = index.as_ref();
		if index.len() != 2 {
		    None
		} else {
    		self.cols.get(index[0]).and_then(|col| col.cells.get(index[1])).and_then(|cell| cell.as_ref())
		}
	}
	pub fn cell_at_mut<I: AsRef<[usize]>>(&mut self, index: I) -> Option<&mut TableCell<T>> {
		let index = index.as_ref();
		if index.len() != 2 {
		    None
		} else {
    		self.cols.get_mut(index[0]).and_then(|col| col.cells.get_mut(index[1])).and_then(|cell| cell.as_mut())
		}
	}
	pub fn column_at(&self, index: usize) -> Option<&TableColumn<T>> {
		self.cols.get(index)
	}
	pub fn column_at_mut(&mut self, index: usize) -> Option<&mut TableColumn<T>> {
		self.cols.get_mut(index)
	}
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::Index<I> for TableData<T> {
	type Output = TableCell<T>;
	fn index(&self, index: I) -> &Self::Output {
		self.cell_at(index).unwrap()
	}
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::IndexMut<I> for TableData<T> {
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.cell_at_mut(index).unwrap()
	}
}

