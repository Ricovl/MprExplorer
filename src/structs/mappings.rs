use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct MappingMicroflowCallImpl {
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "parameter_mappings")]
	parameter_mappings: Vec<mappings::MicroflowCallParameterMappingImpl>,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowCallParameterMappingImpl {
	#[serde(rename = "json_value_element_path")]
	json_value_element_path: String,
	#[serde(rename = "level_of_parent")]
	level_of_parent: i64,
	#[serde(rename = "parameter")]
	parameter: String,
	#[serde(rename = "xml_value_element_path")]
	xml_value_element_path: String,
}

