#[derive(Serialize, Deserialize)]
struct DataSetParameter {
	#[serde(rename = "Constraints")]
	constraints: Vec<2, []>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: DataTypes$ObjectType,
	#[serde(rename = "ParameterTypeIsRange")]
	parameter_type_is_range: Bool,
}

#[derive(Serialize, Deserialize)]
struct DataSetAccess {
	#[serde(rename = "ModuleRoleAccessList")]
	module_role_access_list: Vec<2, []>,
}

#[derive(Serialize, Deserialize)]
struct OqlDataSetSource {
	#[serde(rename = "IEIQ")]
	ieiq: Bool,
	#[serde(rename = "Query")]
	query: String,
}

#[derive(Serialize, Deserialize)]
struct DataSet {
	#[serde(rename = "DataSetAccess")]
	data_set_access: DataSets$DataSetAccess,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<2, ["DataSets$DataSetParameter"]>,
	#[serde(rename = "Source")]
	source: DataSets$OqlDataSetSource,
}

