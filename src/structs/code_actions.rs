use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct BasicParameterType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Type")]
	var_type: code_actions::StringType,
}

#[derive(Serialize, Deserialize)]
pub struct BooleanType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct ConcreteEntityType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct DateTimeType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct DecimalType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct EntityTypeParameterType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "TypeParameterPointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
pub struct IntegerType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct ListType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Parameter")]
	parameter: code_actions::ConcreteEntityType,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowActionInfo {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Caption")]
	caption: String,
	#[serde(rename = "Category")]
	category: String,
	#[serde(rename = "IconData")]
	icon_data: Binary,
	#[serde(rename = "IconDataDark")]
	icon_data_dark: Binary,
	#[serde(rename = "ImageData")]
	image_data: Binary,
	#[serde(rename = "ImageDataDark")]
	image_data_dark: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct ParameterizedEntityType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "TypeParameterPointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct StringTemplateParameterType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Grammar")]
	grammar: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

#[derive(Serialize, Deserialize)]
pub struct TypeParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct VoidType {
	#[serde(rename = "$ID")]
	_id: Uuid,

}

