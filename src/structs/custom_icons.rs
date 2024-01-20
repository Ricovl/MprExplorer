use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CustomIcon {
	#[serde(rename = "CharacterCode")]
	character_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tags")]
	tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomIconCollection {
	#[serde(rename = "CollectionClass")]
	collection_class: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "FontData")]
	font_data: Binary,
	#[serde(rename = "Icons")]
	icons: Vec<CustomIcons$CustomIcon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Prefix")]
	prefix: String,
}

