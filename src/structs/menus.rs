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
	item_collection: Option<menus::MenuItemCollection>,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItem {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Action")]
	action: Option<forms::OnAction>,
	#[serde(rename = "AlternativeText")]
	alternative_text: Option<Empty>,
	#[serde(rename = "Caption")]
	caption: Option<texts::Text>,
	#[serde(rename = "Icon")]
	icon: Option<Empty>,
	#[serde(rename = "Items", deserialize_with = "deserialize_settings")]
	items: Vec<menus::MenuItem>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItemCollection {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Items", deserialize_with = "deserialize_settings")]
	items: Vec<menus::MenuItem>,
}

