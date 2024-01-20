use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CustomIcon {
	#[serde(rename = "character_code")]
	character_code: i64,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "tags")]
	tags: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomIconCollection {
	#[serde(rename = "collection_class")]
	collection_class: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "font_data")]
	font_data: Binary,
	#[serde(rename = "icons")]
	icons: Vec<>,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "prefix")]
	prefix: String,
}

