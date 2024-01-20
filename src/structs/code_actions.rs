use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct StringTemplateParameterType {
	#[serde(rename = "Grammar")]
	grammar: String,
}

#[derive(Serialize, Deserialize)]
pub struct DecimalType {
}

#[derive(Serialize, Deserialize)]
pub struct ConcreteEntityType {
	#[serde(rename = "Entity")]
	entity: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnumerationType {
	#[serde(rename = "Enumeration")]
	enumeration: String,
}

#[derive(Serialize, Deserialize)]
pub struct BasicParameterType {
	#[serde(rename = "Type")]
	type: CodeActions$ParameterizedEntityType,
}

#[derive(Serialize, Deserialize)]
pub struct DateTimeType {
}

#[derive(Serialize, Deserialize)]
pub struct StringType {
}

#[derive(Serialize, Deserialize)]
pub struct IntegerType {
}

#[derive(Serialize, Deserialize)]
pub struct EntityTypeParameterType {
	#[serde(rename = "TypeParameterPointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct ParameterizedEntityType {
	#[serde(rename = "TypeParameterPointer")]
	type_parameter_pointer: Binary,
}

#[derive(Serialize, Deserialize)]
pub struct BooleanType {
}

#[derive(Serialize, Deserialize)]
pub struct TypeParameter {
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowActionInfo {
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
pub struct VoidType {
}

#[derive(Serialize, Deserialize)]
pub struct ListType {
	#[serde(rename = "Parameter")]
	parameter: CodeActions$ConcreteEntityType,
}

