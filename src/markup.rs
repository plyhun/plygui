use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum MarkupNode {
	Attribute(String),
	Child(Markup),
	Children(Vec<Markup>),
}

pub type Markup = HashMap<String, MarkupNode>;
pub type MarkupIds = HashMap<String, super::ids::Id>;