use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct DataSetAccess {
	#[serde(rename = "module_role_access_list")]
	module_role_access_list: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct DataSetParameter {
	#[serde(rename = "constraints")]
	constraints: Vec<>,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameter_type")]
	parameter_type: data_types::ObjectType,
	#[serde(rename = "parameter_type_is_range")]
	parameter_type_is_range: bool,
}

#[derive(Serialize, Deserialize)]
pub struct DataSet {
	#[serde(rename = "data_set_access")]
	data_set_access: data_sets::DataSetAccess,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameters")]
	parameters: Vec<>,
	#[serde(rename = "source")]
	source: data_sets::OqlDataSetSource,
}

#[derive(Serialize, Deserialize)]
pub struct OqlDataSetSource {
	#[serde(rename = "ieiq")]
	ieiq: bool,
	#[serde(rename = "query")]
	query: String,
}

