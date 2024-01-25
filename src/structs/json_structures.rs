use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct JsonElement {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Children")]
	children: Vec<json_structures::JsonElement, >,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "ExposedItemName")]
	exposed_item_name: String,
	#[serde(rename = "ExposedName")]
	exposed_name: String,
	#[serde(rename = "FractionDigits")]
	fraction_digits: i64,
	#[serde(rename = "IsDefaultType")]
	is_default_type: bool,
	#[serde(rename = "MaxLength")]
	max_length: i64,
	#[serde(rename = "MaxOccurs")]
	max_occurs: i64,
	#[serde(rename = "MinOccurs")]
	min_occurs: i64,
	#[serde(rename = "Nillable")]
	nillable: bool,
	#[serde(rename = "OriginalValue")]
	original_value: String,
	#[serde(rename = "Path")]
	path: String,
	#[serde(rename = "PrimitiveType")]
	primitive_type: String,
	#[serde(rename = "TotalDigits")]
	total_digits: i64,
	#[serde(rename = "WarningMessage")]
	warning_message: String,
}

#[derive(Serialize, Deserialize)]
pub struct JsonStructure {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Elements")]
	elements: Vec<json_structures::JsonElement, >,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "JsonSnippet")]
	json_snippet: String,
	#[serde(rename = "Name")]
	name: String,
}

