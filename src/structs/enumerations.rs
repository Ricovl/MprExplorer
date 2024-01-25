use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Condition {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AttributeValue")]
	attribute_value: String,
	#[serde(rename = "EditableVisible")]
	editable_visible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Enumeration {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Values")]
	values: Vec<enumerations::EnumerationValue, >,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationValue {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Caption")]
	caption: texts::Text,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Name")]
	name: String,
}

