use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct MappingMicroflowCallImpl {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<mappings::MicroflowCallParameterMappingImpl, >,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallParameterMappingImpl {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "JsonValueElementPath")]
	json_value_element_path: String,
	#[serde(rename = "LevelOfParent")]
	level_of_parent: i64,
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "XmlValueElementPath")]
	xml_value_element_path: String,
}

