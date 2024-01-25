use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct EntityMessageDefinition {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "exposed_entity")]
	exposed_entity: message_definitions::ExposedEntity,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ExposedAssociation {
	#[serde(rename = "association")]
	association: String,
	#[serde(rename = "children")]
	children: Vec<message_definitions::ExposedAttribute, message_definitions::ExposedAssociation, >,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "element_type")]
	element_type: String,
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "error_message")]
	error_message: String,
	#[serde(rename = "example")]
	example: String,
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
	#[serde(rename = "original_name")]
	original_name: String,
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
pub struct ExposedAttribute {
	#[serde(rename = "attribute")]
	attribute: String,
	#[serde(rename = "children")]
	children: Vec<UnknownType>,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "element_type")]
	element_type: String,
	#[serde(rename = "error_message")]
	error_message: String,
	#[serde(rename = "example")]
	example: String,
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
	#[serde(rename = "original_name")]
	original_name: String,
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
pub struct ExposedEntity {
	#[serde(rename = "children")]
	children: Vec<message_definitions::ExposedAttribute, message_definitions::ExposedAssociation, >,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "element_type")]
	element_type: String,
	#[serde(rename = "entity")]
	entity: String,
	#[serde(rename = "error_message")]
	error_message: String,
	#[serde(rename = "example")]
	example: String,
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
	#[serde(rename = "original_name")]
	original_name: String,
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
pub struct MessageDefinitionCollection {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "message_definitions")]
	message_definitions: Vec<message_definitions::EntityMessageDefinition, >,
	#[serde(rename = "name")]
	name: String,
}

