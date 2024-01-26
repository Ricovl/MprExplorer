use bson::Binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CustomIcon {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "CharacterCode")]
	character_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tags", deserialize_with = "deserialize_settings")]
	tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CustomIconCollection {
	#[serde(rename = "$ID")]
	_id: Uuid,

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
	#[serde(rename = "Icons", deserialize_with = "deserialize_settings")]
	icons: Vec<custom_icons::CustomIcon>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Prefix")]
	prefix: String,
}

