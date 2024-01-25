use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct BasicParameterType {
	#[serde(rename = "_type")]
	_type: code_actions::StringType,
}

#[derive(Serialize, Deserialize)]
pub struct BooleanType {
}

#[derive(Serialize, Deserialize)]
pub struct ConcreteEntityType {
	#[serde(rename = "entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateTimeType {
}

#[derive(Serialize, Deserialize)]
pub struct DecimalType {
}

#[derive(Serialize, Deserialize)]
pub struct EntityTypeParameterType {
	#[serde(rename = "type_parameter_pointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationType {
	#[serde(rename = "enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
pub struct IntegerType {
}

#[derive(Serialize, Deserialize)]
pub struct ListType {
	#[serde(rename = "parameter")]
	parameter: code_actions::ConcreteEntityType,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowActionInfo {
	#[serde(rename = "caption")]
	caption: String,
	#[serde(rename = "category")]
	category: String,
	#[serde(rename = "icon_data")]
	icon_data: Binary,
	#[serde(rename = "icon_data_dark")]
	icon_data_dark: Binary,
	#[serde(rename = "image_data")]
	image_data: Binary,
	#[serde(rename = "image_data_dark")]
	image_data_dark: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct ParameterizedEntityType {
	#[serde(rename = "type_parameter_pointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct StringTemplateParameterType {
	#[serde(rename = "grammar")]
	grammar: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringType {
}

#[derive(Serialize, Deserialize)]
pub struct TypeParameter {
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct VoidType {
}

