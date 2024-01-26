use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct MenuDocument {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ItemCollection")]
	item_collection: menus::MenuItemCollection,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItem {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	action: forms::OnAction,
	#[serde(rename = "AlternativeText")]
	alternative_text: Empty,
	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "Icon")]
	icon: Empty,
	#[serde(rename = "Items")]
	items: Vec<menus::MenuItem>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItemCollection {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Items")]
	items: Vec<menus::MenuItem>,
}

