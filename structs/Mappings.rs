#[derive(Serialize, Deserialize)]
struct MappingMicroflowCallImpl {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ParameterMappings")]
	parameter_mappings: Vec<2, ["Mappings$MicroflowCallParameterMappingImpl"]>,
}

#[derive(Serialize, Deserialize)]
struct MicroflowCallParameterMappingImpl {
	#[serde(rename = "JsonValueElementPath")]
	json_value_element_path: String,
	#[serde(rename = "LevelOfParent")]
	level_of_parent: i64,
	#[serde(rename = "Parameter")]
	parameter: String,
	#[serde(rename = "XmlValueElementPath")]
	xml_value_element_path: String,
}

