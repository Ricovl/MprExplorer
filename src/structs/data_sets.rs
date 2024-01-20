use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct OqlDataSetSource {
	#[serde(rename = "IEIQ")]
	ieiq: bool,
	#[serde(rename = "Query")]
	query: String,
}

#[derive(Serialize, Deserialize)]
pub struct DataSetParameter {
	#[serde(rename = "Constraints")]
	constraints: Vec<2, []>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: DataTypes$ObjectType,
	#[serde(rename = "ParameterTypeIsRange")]
	parameter_type_is_range: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DataSet {
	#[serde(rename = "DataSetAccess")]
	data_set_access: DataSets$DataSetAccess,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<DataSets$DataSetParameter>,
	#[serde(rename = "Source")]
	source: DataSets$OqlDataSetSource,
}

#[derive(Serialize, Deserialize)]
pub struct DataSetAccess {
	#[serde(rename = "ModuleRoleAccessList")]
	module_role_access_list: Vec<2, []>,
}

