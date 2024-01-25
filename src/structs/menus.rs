use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct MenuDocument {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "item_collection")]
	item_collection: menus::MenuItemCollection,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItem {
	#[serde(rename = "action")]
	action: forms::MicroflowAction,
	#[serde(rename = "alternative_text")]
	alternative_text: Empty,
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "icon")]
	icon: Empty,
	#[serde(rename = "items")]
	items: Vec<menus::MenuItem, >,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItemCollection {
	#[serde(rename = "items")]
	items: Vec<menus::MenuItem, >,
}

