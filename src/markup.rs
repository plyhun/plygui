use std::fmt;
use std::collections::HashMap;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

pub type MemberType = String;
pub type CallbackPtr = usize;
pub type MemberSpawner = fn() -> Box<super::traits::UiControl>;

pub const ID: &str = "id";
pub const TYPE: &str = "type";
pub const CHILD: &str = "child";
pub const CHILDREN: &str = "children";

const FIELDS: &[&str] = &[ID, TYPE];

pub const MEMBER_TYPE_LINEAR_LAYOUT: &str = "LinearLayout";
pub const MEMBER_TYPE_BUTTON: &str = "Button";

pub struct MarkupRegistry {
	spawners: HashMap<MemberType, MemberSpawner>,
	ids: HashMap<String, super::ids::Id>,
	bindings: HashMap<String, CallbackPtr>,
}

impl MarkupRegistry {
	pub fn new() -> MarkupRegistry {
		MarkupRegistry {
			spawners: HashMap::new(),
			ids: HashMap::new(),
			bindings: HashMap::new(),
		}
	}
	
	pub fn register_member(&mut self, member_type: MemberType, member_spawner: MemberSpawner) -> Result<(), MarkupError> {
		if self.spawners.get(&member_type).is_none() {
			self.spawners.insert(member_type, member_spawner); 
			Ok(())
		} else {
			Err(MarkupError::MemberAlreadyRegistered)
		}
	}
	pub fn unregister_member(&mut self, member_type: &MemberType) -> Result<(), MarkupError> {
		if self.spawners.remove(member_type).is_none() {
			Err(MarkupError::MemberNotFound)
		} else {
			Ok(())
		}
	}
	pub fn member(&self, member_type: &MemberType) -> Result<&MemberSpawner, MarkupError> {
		self.spawners.get(member_type).ok_or(MarkupError::MemberNotFound)
	}
	
	pub fn bind_callback(&mut self, name: &str, callback: CallbackPtr) -> Result<(), MarkupError> {
		if self.bindings.get(name).is_none() {
			self.bindings.insert(name.into(), callback); 
			Ok(())
		} else {
			Err(MarkupError::CallbackAlreadyBinded)
		}
	}
	pub fn unbind_callback(&mut self, name: &str) -> Result<(), MarkupError> {
		if self.bindings.remove(name).is_none() {
			Err(MarkupError::CallbackNotFound)
		} else {
			Ok(())
		}
	}
	pub fn callback(&self, name: &str) -> Result<&CallbackPtr, MarkupError> {
		self.bindings.get(name).ok_or(MarkupError::CallbackNotFound)
	}
	
	pub fn store_id(&mut self, control_id: &str, generated_id: super::ids::Id) -> Result<(), MarkupError> {
		if self.ids.get(control_id).is_none() {
			self.ids.insert(control_id.into(), generated_id); 
			Ok(())
		} else {
			Err(MarkupError::IdAlreadyExists)
		}
	}
	pub fn remove_id(&mut self, control_id: &str) -> Result<(), MarkupError> {
		if self.ids.remove(control_id).is_none() {
			Err(MarkupError::IdNotFound)
		} else {
			Ok(())
		}
	}
	pub fn id(&self, control_id: &str) -> Result<&super::ids::Id, MarkupError> {
		self.ids.get(control_id).ok_or(MarkupError::IdNotFound)
	}
}

#[derive(Debug, Clone)]
pub enum MarkupError {
	MemberNotFound,
	MemberAlreadyRegistered,
	CallbackNotFound,
	CallbackAlreadyBinded,
	IdNotFound,
	IdAlreadyExists,
}

#[derive(Debug, Clone)]
pub struct Markup {
	pub id: Option<String>,
	pub member_type: MemberType,
	pub attributes: HashMap<String, MarkupNode>
}

#[derive(Debug, Clone)]
pub enum MarkupNode {
	Attribute(String),
	Child(Markup),
	Children(Vec<Markup>),
}

impl MarkupNode {
	pub fn as_attribute(&self) -> &str {
		match *self {
			MarkupNode::Attribute(ref attr) => attr.as_str(),
			_ => panic!("MarkupNode is not an Attribute: {:?}", self),
		}
	}
	pub fn as_child(&self) -> &Markup {
		match *self {
			MarkupNode::Child(ref markup) => markup,
			_ => panic!("MarkupNode is not a Child Markup: {:?}", self),
		}
	}
	pub fn as_children(&self) -> &[Markup] {
		match *self {
			MarkupNode::Children(ref children) => children.as_slice(),
			_ => panic!("MarkupNode is not the Children Markups: {:?}", self),
		}
	}
}

impl <'de> Deserialize<'de> for Markup {
	fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
		deserializer.deserialize_struct("Markup", FIELDS, MarkupVisitor)
	}
}

struct MarkupVisitor;

impl<'de> Visitor<'de> for MarkupVisitor {
    type Value = Markup;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("struct Markup")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Markup, V::Error> where V: MapAccess<'de> {
        let mut id = None;
        let mut member_type = None;
        let mut child_found = false;
        
        let mut attributes = HashMap::new();
        
        while let Some(key) = map.next_key()? {
        	println!("{} found", key);
            
            match key {
                TYPE => {
                    if member_type.is_some() {
                        return Err(de::Error::duplicate_field(TYPE));
                    }
                    member_type = Some(map.next_value()?);
                },
                ID => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field(ID));
                    }
                    id = Some(map.next_value()?);
                },
                CHILD => {
                	if child_found {
	                	return Err(de::Error::duplicate_field("child / children"));
                	} else {
	                	attributes.insert(key.into(), MarkupNode::Child(map.next_value::<Markup>()?));
	                	child_found = true;
                	}
                },
                CHILDREN => {
                	if child_found {
	                	return Err(de::Error::duplicate_field("child / children"));
                	} else {
	                	attributes.insert(key.into(), MarkupNode::Children(map.next_value::<Vec<Markup>>()?));
	                	child_found = true;
                	}
                }
                _ => {
                	attributes.insert(key.into(), MarkupNode::Attribute(map.next_value::<String>()?));
                },
            }
        }
        Ok(Markup {
	        id: id,
	        member_type: member_type.ok_or_else(|| de::Error::missing_field(TYPE))?,
	        attributes: attributes,
        })
    }
}

pub fn parse_markup(json: &str, registry: &mut MarkupRegistry) -> Box<super::traits::UiControl> {
	let markup: Markup = super::serde_json::from_str(json).unwrap();
	
	let mut control = registry.member(&markup.member_type).unwrap()();
	control.fill_from_markup(&markup, registry);
	control
}

#[macro_export]
macro_rules! bind_markup_callback {
	($reg: ident, $cb: ident) => {
		registry.bind_callback(stringify!($cb), $cb as CallbackPtr).unwrap();
	}
}
#[macro_export]
macro_rules! fill_from_markup_base {
	($this: expr, $mrk: ident, $reg: ident, $typ:ty, [$($arg:ident),+]) => {
		if !&[$($arg),+].contains(&$mrk.member_type.as_str()) {
			match $mrk.id {
				Some(ref id) => panic!("Markup does not belong to {}: {} ({})", stringify!($typ), $mrk.member_type, id),
				None => panic!("Markup does not belong to {}: {}", stringify!($typ), $mrk.member_type),
			}
		}		
    	if let Some(ref id) = $mrk.id {
    		$reg.store_id(&id, $this.id()).unwrap();
    	}
	}
}
#[macro_export]
macro_rules! fill_from_markup_label {
	($this: expr, $mrk: ident) => {
		$this.set_label(&$mrk.attributes.get("label").unwrap().as_attribute());
	}
}
#[macro_export]
macro_rules! fill_from_markup_callbacks {
	($this: expr, $mrk: ident, $reg: ident, [$($cbname:expr => $cbtyp:ty),+]) => {
		$(if let Some(callback) = $mrk.attributes.get($cbname) {
    		let callback = $reg.callback(callback.as_attribute()).unwrap();
    		$this.on_left_click(Some(Box::new(unsafe { 
    			let callback: $cbtyp = mem::transmute(*callback);
    			callback 
    		})));
    	})+
	}
}