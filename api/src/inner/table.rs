use crate::{layout, types};

use super::auto::{HasInner, Abstract, Spawnable};
use super::container::AContainer;
use super::item_clickable::{ItemClickable, ItemClickableInner};
use super::adapted::{AAdapted, Adapted, AdaptedInner, AdaptedBase};
use super::control::{AControl, Control, ControlInner, ControlBase};
use super::member::{AMember, Member, MemberBase};

use std::collections::HashMap;

define! {
    Table: Control + Adapted + ItemClickable {
	    /*base: {
            pub on_item_click: Option<OnItemClick>,
        }*/
	    inner: {
	        fn set_column_width(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, index: usize, size: layout::Size);
            fn set_row_height(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, index: usize, size: layout::Size);
            fn resize(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, width: usize, height: usize) -> (usize, usize);
            fn size(&self, _: &MemberBase, _: &ControlBase, adapted: &AdaptedBase) -> (usize, usize) {
                (adapted.adapter.len_at(&[]).unwrap_or(0), adapted.adapter.len_at(&[0]).unwrap_or(0))
            }
        }
	    outer: {
            fn resize(&mut self, width: usize, height: usize) -> (usize, usize);
            fn size(&self) -> (usize, usize);
            fn set_column_width(&mut self, index: usize, size: layout::Size);
            fn set_row_height(&mut self, index: usize, size: layout::Size);
        }
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
    #[inline]
    fn resize(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, width: usize, height: usize) -> (usize, usize) {
        self.inner_mut().resize(member, control, adapted, width, height)
    }
    #[inline]
    fn size(&self, member: &MemberBase, control: &ControlBase, adapted: &AdaptedBase) -> (usize, usize) {
        self.inner().size(member, control, adapted)
    }
    #[inline]
    fn set_column_width(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, index: usize, size: layout::Size) {
        self.inner_mut().set_column_width(member, control, adapted, index, size)
    }
    #[inline]
    fn set_row_height(&mut self, member: &mut MemberBase, control: &mut ControlBase, adapted: &mut AdaptedBase, index: usize, size: layout::Size) {
        self.inner_mut().set_row_height(member, control, adapted, index, size)
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
    fn resize(&mut self, width: usize, height: usize) -> (usize, usize) {
        let (m,c,a,t) = self.as_adapted_parts_mut();
        t.resize(m, c, a, width, height)
    }
    #[inline]
    fn size(&self) -> (usize, usize) {
        let (m,c,a,t) = self.as_adapted_parts();
        t.size(m, c, a)
    }
    #[inline]
    fn set_column_width(&mut self, index: usize, size: layout::Size) {
        let (m,c,a,t) = self.as_adapted_parts_mut();
        t.set_column_width(m, c, a, index, size)
    }
    #[inline]
    fn set_row_height(&mut self, index: usize, size: layout::Size) {
        let (m,c,a,t) = self.as_adapted_parts_mut();
        t.set_row_height(m, c, a, index, size)
    }
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
    pub rows: Vec<TableRow<T>>,
    pub default_row_height: layout::Size
}
impl <T: Sized> Default for TableData<T> {
	fn default() -> Self {
		Self { cols: Vec::new(), default_row_height: layout::Size::WrapContent, rows: Vec::new() }
	}
}
pub struct TableColumn<T: Sized> {
    pub control: Option<Box<dyn Control>>,
    pub native: T,
    pub width: layout::Size,
}
pub struct TableRow<T: Sized> {
    pub cells: Vec<Option<TableCell<T>>>,
    pub control: Option<Box<dyn Control>>,
    pub native: T,
    pub height: layout::Size,
}
pub struct TableCell<T: Sized> {
    pub control: Option<Box<dyn Control>>,
    pub native: T,
}
impl<T: Sized> TableRow<T> {
	pub fn cell_at(&self, index: usize) -> Option<&TableCell<T>> {
		self.cells.get(index).and_then(|cell| cell.as_ref())
	}
	pub fn cell_at_mut(&mut self, index: usize) -> Option<&mut TableCell<T>> {
		self.cells.get_mut(index).and_then(|cell| cell.as_mut())
	}
}
impl<T: Sized> TableData<T> {
	pub fn cell_at<I: AsRef<[usize]>>(&self, index: I) -> Option<&TableCell<T>> {
		let index = index.as_ref();
		if index.len() != 2 {
		    None
		} else {
    		self.rows.get(index[0]).and_then(|row| row.cells.get(index[1])).and_then(|cell| cell.as_ref())
		}
	}
	pub fn cell_at_mut<I: AsRef<[usize]>>(&mut self, index: I) -> Option<&mut TableCell<T>> {
		let index = index.as_ref();
		if index.len() != 2 {
		    None
		} else {
    		self.rows.get_mut(index[0]).and_then(|row| row.cells.get_mut(index[1])).and_then(|cell| cell.as_mut())
		}
	}
	pub fn column_at(&self, index: usize) -> Option<&TableColumn<T>> {
		self.cols.get(index)
	}
	pub fn column_at_mut(&mut self, index: usize) -> Option<&mut TableColumn<T>> {
		self.cols.get_mut(index)
	}
    pub fn row_at(&self, index: usize) -> Option<&TableRow<T>> {
		self.rows.get(index)
	}
	pub fn row_at_mut(&mut self, index: usize) -> Option<&mut TableRow<T>> {
		self.rows.get_mut(index)
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

