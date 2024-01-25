use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct JavaAction {
	#[serde(rename = "action_default_return_name")]
	action_default_return_name: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "java_return_type")]
	java_return_type: code_actions::BooleanType,
	#[serde(rename = "microflow_action_info")]
	microflow_action_info: Empty,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameters")]
	parameters: Vec<java_actions::JavaActionParameter, >,
	#[serde(rename = "type_parameters")]
	type_parameters: Vec<code_actions::TypeParameter, >,
}

#[derive(Serialize, Deserialize)]
pub struct JavaActionParameter {
	#[serde(rename = "category")]
	category: String,
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "is_required")]
	is_required: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameter_type")]
	parameter_type: code_actions::BasicParameterType,
}

#[derive(Serialize, Deserialize)]
pub struct MicroflowJavaActionParameterType {
}

