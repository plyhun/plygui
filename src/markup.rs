use std::fmt;
use std::collections::HashMap;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

pub const ID: &str = "id";
pub const TYPE: &str = "type";
pub const CHILD: &str = "child";
pub const CHILDREN: &str = "children";
const FIELDS: &'static [&'static str] = &[ID, TYPE];

#[derive(Debug, Clone)]
pub struct Markup {
	pub id: Option<String>,
	pub member_type: String,
	pub attributes: HashMap<String, MarkupNode>
}

#[derive(Debug, Clone)]
pub enum MarkupNode {
	Attribute(String),
	Child(Markup),
	Children(Vec<Markup>),
}

pub type MarkupIds = HashMap<String, super::ids::Id>;

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
                        return Err(de::Error::duplicate_field("type"));
                    }
                    member_type = Some(map.next_value()?);
                },
                ID => {
                    if id.is_some() {
                        return Err(de::Error::duplicate_field("id"));
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
	        member_type: member_type.ok_or_else(|| de::Error::missing_field("type"))?,
	        attributes: attributes,
        })
    }
}