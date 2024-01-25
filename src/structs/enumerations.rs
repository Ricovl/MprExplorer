use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Condition {
	#[serde(rename = "attribute_value")]
	attribute_value: String,
	#[serde(rename = "editable_visible")]
	editable_visible: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Enumeration {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "values")]
	values: Vec<enumerations::EnumerationValue, >,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationValue {
	#[serde(rename = "caption")]
	caption: texts::Text,
	#[serde(rename = "image")]
	image: String,
	#[serde(rename = "name")]
	name: String,
}

