use crate::{AdapterView, Adapter, Control};
use crate::imp;

pub struct SimpleTextAdapter {
    items: Vec<String>
}

impl SimpleTextAdapter {
    pub fn new() -> Self {
        SimpleTextAdapter { items: Vec::new() }
    }
    pub fn with_iterator<'a, T, I>(i: I) -> Self where T: AsRef<str>, I: Iterator<Item=T> {
        let mut t = Self::new();
        for item in i {
            t.items.push(String::from(item.as_ref()));
        }
        t
    }
    pub fn with_into_iterator<'a, T, I>(i: I) -> Self where T: AsRef<str>, I: IntoIterator<Item=T> {
        Self::with_iterator(i.into_iter())
    }
}
impl SimpleTextAdapter {
    pub fn text_at(&self, i: usize) -> Option<&String> {
        self.items.get(i)    
    }
    pub fn text_at_mut(&mut self, i: usize) -> Option<&mut String> {
        self.items.get_mut(i)    
    }
    pub fn push<T: AsRef<str>>(&mut self, arg: T) {
        self.items.push(String::from(arg.as_ref()));
    }
}
impl Adapter for SimpleTextAdapter {
    fn len(&self) -> usize {
        self.items.len()
    }
	fn spawn_item_view(&mut self, i: usize, _parent: &dyn AdapterView) -> Box<dyn Control> {
    	imp::Text::with_text(self.items[i].as_str()).into_control()
    	//imp::Button::with_label(self.items[i].as_str()).into_control()
	}
}

