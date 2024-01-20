use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct MenuItem {
	#[serde(rename = "Action")]
	action: Forms$NoAction,
	#[serde(rename = "AlternativeText")]
	alternative_text: Null,
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "Icon")]
	icon: Forms$IconCollectionIcon,
	#[serde(rename = "Items")]
	items: Vec<3, []>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuItemCollection {
	#[serde(rename = "Items")]
	items: Vec<Menus$MenuItem>,
}

#[derive(Serialize, Deserialize)]
pub struct MenuDocument {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ItemCollection")]
	item_collection: Menus$MenuItemCollection,
	#[serde(rename = "Name")]
	name: String,
}

