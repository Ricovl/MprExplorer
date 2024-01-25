use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

// Todo: Rename this
pub enum MessageExposure {
	ExposedAssociation(message_definitions::ExposedAssociation), 
	ExposedAttribute(message_definitions::ExposedAttribute)
}

#[derive(Serialize, Deserialize)]
pub struct EntityMessageDefinition {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExposedEntity")]
	exposed_entity: message_definitions::ExposedEntity,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExposedAssociation {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Association")]
	association: String,
	#[serde(rename = "Children")]
	children: Vec<MessageExposure>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "Example")]
	example: String,
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
	#[serde(rename = "OriginalName")]
	original_name: String,
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
pub struct ExposedAttribute {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Attribute")]
	attribute: String,
	#[serde(rename = "Children")]
	children: Vec<UnknownType>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "Example")]
	example: String,
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
	#[serde(rename = "OriginalName")]
	original_name: String,
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
pub struct ExposedEntity {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Children")]
	children: Vec<MessageExposure>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ElementType")]
	element_type: String,
	#[serde(rename = "Entity")]
	entity: String,
	#[serde(rename = "ErrorMessage")]
	error_message: String,
	#[serde(rename = "Example")]
	example: String,
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
	#[serde(rename = "OriginalName")]
	original_name: String,
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
pub struct MessageDefinitionCollection {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "MessageDefinitions")]
	message_definitions: Vec<message_definitions::EntityMessageDefinition, >,
	#[serde(rename = "Name")]
	name: String,
}

