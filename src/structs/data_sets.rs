use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct DataSet {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DataSetAccess")]
	data_set_access: Option<data_sets::DataSetAccess>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<data_sets::DataSetParameter>,
	#[serde(rename = "Source")]
	source: Option<data_sets::OqlDataSetSource>,
}

#[derive(Serialize, Deserialize)]
pub struct DataSetAccess {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ModuleRoleAccessList", deserialize_with = "deserialize_settings")]
	module_role_access_list: Vec<UnknownType>,
}

#[derive(Serialize, Deserialize)]
pub struct DataSetParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Constraints", deserialize_with = "deserialize_settings")]
	constraints: Vec<UnknownType>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: Option<data_types::ObjectType>,
	#[serde(rename = "ParameterTypeIsRange")]
	parameter_type_is_range: bool,
}

#[derive(Serialize, Deserialize)]
pub struct OqlDataSetSource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "IEIQ")]
	ieiq: bool,
	#[serde(rename = "Query")]
	query: String,
}

