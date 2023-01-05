use crate::types;

use super::auto::{HasInner, Abstract, Spawnable};
use super::container::AContainer;
use super::item_clickable::{ItemClickable, ItemClickableInner};
use super::adapted::{AAdapted, Adapted, AdaptedInner};
use super::control::{AControl, Control, ControlInner};
use super::member::{AMember, Member};
use super::adapter::Node;

define! {
    Tree: Control + Adapted + ItemClickable {
	    /*base: {
            pub on_item_click: Option<OnItemClick>,
        }*/
	    inner: {}
	    outer: {}
	    constructor: {
    	    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Tree>;
	    }
	    extends: { ItemClickable }
    }
}

/*impl<T: TreeInner + 'static> ATree<T> {
    #[inline]
    pub fn with_inner(inner: T) -> Self {
        Self { base: TreeBase { on_item_click: None }, inner }
    }
}*/
impl<II: TreeInner, T: HasInner<I = II> + Abstract + 'static> TreeInner for T {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Tree> {
        <<Self as HasInner>::I as TreeInner>::with_adapter(adapter)
    }
}
impl<T: TreeInner> NewTree for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
    #[inline]
    fn with_adapter(adapter: Box<dyn types::Adapter>) -> Box<dyn Tree> {
        T::with_adapter(adapter)
    }
}
// hello E0119
/*impl<T: TreeInner> ItemClickable for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
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

impl<T: TreeInner> Tree for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
    #[inline]
    fn as_tree(&self) -> &dyn Tree {
        self
    }
    #[inline]
    fn as_tree_mut(&mut self) -> &mut dyn Tree {
        self
    }
    #[inline]
    fn into_tree(self: Box<Self>) -> Box<dyn Tree> {
        self
    }
}

impl<T: TreeInner> Spawnable for AMember<AControl<AContainer<AAdapted<ATree<T>>>>> {
    fn spawn() -> Box<dyn Control> {
        <T as Spawnable>::spawn()
    }
}

pub struct TreeNode<T: Sized> {
    pub expanded: bool,
    pub root: Box<dyn Control>,
    pub native: T,
    pub branches: Vec<Self>,
}

impl<T: Sized> TreeNode<T> {
	pub fn node(&self) -> Node {
		if self.branches.len() == 0 {
			Node::Leaf
		} else {
			Node::Branch(self.expanded)
		}
	}
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::Index<I> for TreeNode<T> {
	type Output = Self;
	fn index(&self, index: I) -> &Self::Output {
		let mut i = 0;
		let index = index.as_ref();
		let mut ret = self;
		while i < index.len() {
			ret = &ret.branches[index[i]];
			i += 1; 
		}
		ret
	}
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::IndexMut<I> for TreeNode<T> {
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		let mut i = 0;
		let index = index.as_ref();
		let mut ret = self;
		while i < index.len() {
			ret = &mut ret.branches[index[i]];
			i += 1; 
		}
		ret
	}
}

pub struct TreeNodeList<T: Sized> (pub Vec<TreeNode<T>>);

impl<T: Sized> std::default::Default for TreeNodeList<T> {
    fn default() -> Self {
        TreeNodeList(Vec::new())
    }
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::Index<I> for TreeNodeList<T> {
	type Output = TreeNode<T>;
	fn index(&self, index: I) -> &Self::Output {
		let index = index.as_ref();
		if index.len() < 1 {
			panic!("Empty index!");
		}
		&self.0.as_slice()[index[0]][index[1..].as_ref()]
	}
}

impl<T: Sized, I: AsRef<[usize]>> std::ops::IndexMut<I> for TreeNodeList<T> {
	fn index_mut(&mut self, index: I) -> &mut Self::Output {
		let index = index.as_ref();
		if index.len() < 1 {
			panic!("Empty index!");
		}
		&mut self.0.as_mut_slice()[index[0]][index[1..].as_ref()]
	}
}

impl<T: Sized> std::ops::Deref for TreeNodeList<T> {
	type Target = [TreeNode<T>];
	fn deref(&self) -> &Self::Target {
		self.0.as_slice()
	}
}
impl<T: Sized> std::ops::DerefMut for TreeNodeList<T> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		self.0.as_mut_slice()
	}
}
