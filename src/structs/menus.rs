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
	action: forms::NoAction,
	#[serde(rename = "alternative_text")]
	alternative_text: Null,
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "icon")]
	icon: forms::IconCollectionIcon,
	#[serde(rename = "items")]
	items: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItemCollection {
	#[serde(rename = "items")]
	items: Vec<>,
}

