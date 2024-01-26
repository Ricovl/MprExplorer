use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct JavaScriptAction {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ActionDefaultReturnName")]
	action_default_return_name: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "JavaReturnType")]
	java_return_type: code_actions::ParameterType,
	#[serde(rename = "MicroflowActionInfo")]
	microflow_action_info: Empty,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<java_script_actions::JavaScriptActionParameter>,
	#[serde(rename = "Platform")]
	platform: String,
	#[serde(rename = "TypeParameters")]
	type_parameters: Vec<code_actions::TypeParameter>,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptActionParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Category")]
	category: String,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "IsRequired")]
	is_required: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: code_actions::ParameterType,
}

