use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct JsonElement {
	#[serde(rename = "children")]
	children: Vec<>,
	#[serde(rename = "element_type")]
	element_type: String,
	#[serde(rename = "error_message")]
	error_message: String,
	#[serde(rename = "exposed_item_name")]
	exposed_item_name: String,
	#[serde(rename = "exposed_name")]
	exposed_name: String,
	#[serde(rename = "fraction_digits")]
	fraction_digits: i64,
	#[serde(rename = "is_default_type")]
	is_default_type: bool,
	#[serde(rename = "max_length")]
	max_length: i64,
	#[serde(rename = "max_occurs")]
	max_occurs: i64,
	#[serde(rename = "min_occurs")]
	min_occurs: i64,
	#[serde(rename = "nillable")]
	nillable: bool,
	#[serde(rename = "original_value")]
	original_value: String,
	#[serde(rename = "path")]
	path: String,
	#[serde(rename = "primitive_type")]
	primitive_type: String,
	#[serde(rename = "total_digits")]
	total_digits: i64,
	#[serde(rename = "warning_message")]
	warning_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonStructure {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "elements")]
	elements: Vec<>,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "json_snippet")]
	json_snippet: String,
	#[serde(rename = "name")]
	name: String,
}

