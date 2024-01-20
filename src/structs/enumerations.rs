use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Enumeration {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Values")]
	values: Vec<Enumerations$EnumerationValue>,
}

#[derive(Serialize, Deserialize)]
pub struct Condition {
	#[serde(rename = "AttributeValue")]
	attribute_value: String,
	#[serde(rename = "EditableVisible")]
	editable_visible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationValue {
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Name")]
	name: String,
}

