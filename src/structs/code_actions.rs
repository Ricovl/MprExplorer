use bson::Binary;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

// Todo: add all types that are in this file
#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum ParameterType {
	#[serde(rename = "CodeActions$BooleanType")]
	BooleanType(code_actions::BooleanType),
	#[serde(rename = "CodeActions$ConcreteEntityType")]
	ConcreteEntityType(code_actions::ConcreteEntityType),
	#[serde(rename = "CodeActions$DateTimeType")]
	DateTimeType(code_actions::DateTimeType),
	#[serde(rename = "CodeActions$DecimalType")]
	DecimalType(code_actions::DecimalType),
	#[serde(rename = "CodeActions$EntityTypeParameterType")]
	EntityTypeParameterType(code_actions::EntityTypeParameterType),
	#[serde(rename = "CodeActions$EnumerationType")]
	EnumerationType(code_actions::EnumerationType),
	#[serde(rename = "CodeActions$IntegerType")]
	IntegerType(code_actions::IntegerType),
	#[serde(rename = "CodeActions$ListType")]
	ListType(code_actions::ListType),
	#[serde(rename = "CodeActions$MicroflowActionInfo")]
	MicroflowActionInfo(code_actions::MicroflowActionInfo),
	#[serde(rename = "CodeActions$ParameterizedEntityType")]
	ParameterizedEntityType(code_actions::ParameterizedEntityType),
	#[serde(rename = "CodeActions$StringTemplateParameterType")]
	StringTemplateParameterType(code_actions::StringTemplateParameterType),
	#[serde(rename = "CodeActions$StringType")]
	StringType(code_actions::StringType),
	#[serde(rename = "CodeActions$TypeParameter")]
	TypeParameter(code_actions::TypeParameter),
	#[serde(rename = "CodeActions$VoidType")]
	VoidType(code_actions::VoidType),
	#[serde(rename = "JavaActions$MicroflowJavaActionParameterType")]
	MicroflowJavaActionParameterType(java_actions::MicroflowJavaActionParameterType),
}

#[derive(Serialize, Deserialize)]
pub struct BasicParameterType {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Type")]
	var_type: ParameterType,
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
	parameter: Box<code_actions::ParameterType>,
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

