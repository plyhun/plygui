use super::{callbacks, controls, ids};

use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;

use serde::de::{self, Deserialize, Deserializer, MapAccess, Visitor};
use typemap::{Key, TypeMap};

pub type MemberType = String;
pub type MemberSpawner = fn() -> Box<dyn controls::Control>;

struct CallbackKeyWrapper<T>(PhantomData<T>);
struct CallbackWrapper<T: callbacks::Callback>(T);

impl<T: 'static> Key for CallbackKeyWrapper<T>
where
    T: callbacks::Callback,
{
    type Value = CallbackWrapper<T>;
}

pub const ID: &str = "id";
pub const TYPE: &str = "type";
pub const CHILD: &str = "child";
pub const CHILDREN: &str = "children";

const FIELDS: &[&str] = &[ID, TYPE];

pub const MEMBER_TYPE_SPLITTED: &str = "Splitted";
pub const MEMBER_TYPE_FRAME: &str = "Frame";
pub const MEMBER_TYPE_LINEAR_LAYOUT: &str = "LinearLayout";
pub const MEMBER_TYPE_BUTTON: &str = "Button";
pub const MEMBER_TYPE_TEXT: &str = "Text";
pub const MEMBER_TYPE_IMAGE: &str = "Image";

pub struct MarkupRegistry {
    spawners: HashMap<MemberType, MemberSpawner>,
    ids: HashMap<String, ids::Id>,
    bindings: HashMap<String, TypeMap>,
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

    pub fn push_callback<CallbackFn>(&mut self, name: &str, callback: CallbackFn) -> Result<(), MarkupError>
    where
        CallbackFn: callbacks::Callback + 'static,
    {
        if self.bindings.get(name).is_none() {
            let mut tm = TypeMap::new();
            tm.insert::<CallbackKeyWrapper<CallbackFn>>(CallbackWrapper(callback));
            self.bindings.insert(name.into(), tm);
            Ok(())
        } else {
            Err(MarkupError::CallbackAlreadyBinded)
        }
    }
    pub fn peek_callback<CallbackFn>(&self, name: &str) -> Result<&CallbackFn, MarkupError>
    where
        CallbackFn: callbacks::Callback + 'static,
    {
        let tm = self.bindings.get(name).ok_or(MarkupError::CallbackNotFound)?;
        tm.get::<CallbackKeyWrapper<CallbackFn>>().ok_or(MarkupError::CallbackNotFound).map(|wrapper| &wrapper.0)
    }
    pub fn pop_callback<CallbackFn>(&mut self, name: &str) -> Result<CallbackFn, MarkupError>
    where
        CallbackFn: callbacks::Callback + 'static,
    {
        let tm = self.bindings.get_mut(name).ok_or(MarkupError::CallbackNotFound)?;
        tm.remove::<CallbackKeyWrapper<CallbackFn>>().ok_or(MarkupError::CallbackNotFound).map(|wrapper| wrapper.0)
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
    pub attributes: HashMap<String, MarkupNode>,
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

impl<'de> Deserialize<'de> for Markup {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_struct("Markup", FIELDS, MarkupVisitor)
    }
}

struct MarkupVisitor;

impl<'de> Visitor<'de> for MarkupVisitor {
    type Value = Markup;

    fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str("struct Markup")
    }

    fn visit_map<V>(self, mut map: V) -> Result<Markup, V::Error>
    where
        V: MapAccess<'de>,
    {
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
                }
                ID => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field(ID));
                    }
                    id = Some(map.next_value()?);
                }
                CHILD => {
                    if child_found {
                        return Err(de::Error::duplicate_field("child / children"));
                    } else {
                        attributes.insert(key.into(), MarkupNode::Child(map.next_value::<Markup>()?));
                        child_found = true;
                    }
                }
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
                }
            }
        }
        Ok(Markup {
            id: id,
            member_type: member_type.ok_or_else(|| de::Error::missing_field(TYPE))?,
            attributes: attributes,
        })
    }
}

pub fn parse_markup(json: &str, registry: &mut MarkupRegistry) -> Box<dyn super::controls::Control> {
    let markup: Markup = super::serde_json::from_str(json).unwrap();

    let mut control = registry.member(&markup.member_type).unwrap()();
    control.fill_from_markup(&markup, registry);
    control
}

#[macro_export]
macro_rules! bind_markup_callback {
    ($reg: ident, $cb: ident) => {
        registry.bind_callback(stringify!($cb), $cb as CallbackPtr).unwrap();
    };
}
#[macro_export]
macro_rules! fill_from_markup_base {
	($this: expr, $mem: expr, $mrk: ident, $reg: ident, $typ:ty, [$($arg:ident),+]) => {
		if !&[$($arg),+].contains(&$mrk.member_type.as_str()) {
			match $mrk.id {
				Some(ref id) => panic!("Markup does not belong to {}: {} ({})", stringify!($typ), $mrk.member_type, id),
				None => panic!("Markup does not belong to {}: {}", stringify!($typ), $mrk.member_type),
			}
		}
    	if let Some(ref id) = $mrk.id {
    		$reg.store_id(&id, $mem.id).unwrap();
    	}
	}
}
#[macro_export]
macro_rules! fill_from_markup_label {
    ($this: expr, $mem: expr, $mrk: ident) => {
        use plygui_api::development::HasLabelInner;
        $this.set_label($mem, &$mrk.attributes.get("label").unwrap().as_attribute());
    };
}
#[macro_export]
macro_rules! fill_from_markup_callbacks {
	($this: expr, $mrk: ident, $reg: ident, [$($cbname:ident => $cbtyp:ty),+]) => {
		$(if let Some(callback) = $mrk.attributes.get(stringify!($cbname)) {
    		let callback: $cbtyp = $reg.pop_callback(callback.as_attribute()).unwrap();
    		$this.$cbname(Some(callback));
    	})+
	}
}
#[macro_export]
macro_rules! fill_from_markup_children {
    ($this: expr, $mem: expr, $mrk: ident, $reg: ident) => {
        for child_markup in $mrk.attributes.get(::plygui_api::markup::CHILDREN).unwrap_or(&::plygui_api::markup::MarkupNode::Children(vec![])).as_children() {
            use plygui_api::development::MultiContainerInner;

            let mut child = $reg.member(&child_markup.member_type).unwrap()();
            child.fill_from_markup(child_markup, $reg);
            $this.push_child($mem, child);
        }
    };
}
#[macro_export]
macro_rules! fill_from_markup_child {
    ($this: expr, $mem: expr, $mrk: ident, $reg: ident) => {
        if let Some(child_markup) = $mrk.attributes.get(::plygui_api::markup::CHILD).map(|m| m.as_child()) {
            use plygui_api::development::SingleContainerInner;

            let mut child = $reg.member(&child_markup.member_type).unwrap()();
            child.fill_from_markup(child_markup, $reg);
            $this.set_child($mem, Some(child));
        }
    };
}
