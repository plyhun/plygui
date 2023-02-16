use crate::layout;

use crate::inner::control::Control;

pub struct Matrix<T: Sized> {
    pub cols: Vec<Column<T>>,
    pub rows: Vec<Row<T>>,
    pub default_row_height: layout::Size
}
impl <T: Sized> Default for Matrix<T> {
	fn default() -> Self {
		Self { cols: Vec::new(), default_row_height: layout::Size::WrapContent, rows: Vec::new() }
	}
}
pub struct Column<T: Sized> {
    pub control: Option<Box<dyn Control>>,
    pub native: T,
    pub width: layout::Size,
}
pub struct Row<T: Sized> {
    pub cells: Vec<Option<Cell<T>>>,
    pub control: Option<Box<dyn Control>>,
    pub native: T,
    pub height: layout::Size,
}
pub struct Cell<T: Sized> {
    pub control: Option<Box<dyn Control>>,
    pub native: T,
}
impl<T: Sized> Row<T> {
	pub fn cell_at(&self, index: usize) -> Option<&Cell<T>> {
		self.cells.get(index).and_then(|cell| cell.as_ref())
	}
	pub fn cell_at_mut(&mut self, index: usize) -> Option<&mut Cell<T>> {
		self.cells.get_mut(index).and_then(|cell| cell.as_mut())
	}
}
impl<T: Sized> Matrix<T> {
	pub fn cell_at<I: AsRef<[usize]>>(&self, index: I) -> Option<&Cell<T>> {
		let index = index.as_ref();
		if index.len() != 2 {
		    None
		} else {
    		self.rows.get(index[0]).and_then(|row| row.cells.get(index[1])).and_then(|cell| cell.as_ref())
		}
	}
	pub fn cell_at_mut<I: AsRef<[usize]>>(&mut self, index: I) -> Option<&mut Cell<T>> {
		let index = index.as_ref();
		if index.len() != 2 {
		    None
		} else {
    		self.rows.get_mut(index[0]).and_then(|row| row.cells.get_mut(index[1])).and_then(|cell| cell.as_mut())
		}
	}
	pub fn column_at(&self, index: usize) -> Option<&Column<T>> {
		self.cols.get(index)
	}
	pub fn column_at_mut(&mut self, index: usize) -> Option<&mut Column<T>> {
		self.cols.get_mut(index)
	}
    pub fn row_at(&self, index: usize) -> Option<&Row<T>> {
		self.rows.get(index)
	}
	pub fn row_at_mut(&mut self, index: usize) -> Option<&mut Row<T>> {
		self.rows.get_mut(index)
	}
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::Index<I> for Matrix<T> {
	type Output = Cell<T>;
	fn index(&self, index: I) -> &Self::Output {
		self.cell_at(index).unwrap()
	}
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::IndexMut<I> for Matrix<T> {
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		self.cell_at_mut(index).unwrap()
	}
}