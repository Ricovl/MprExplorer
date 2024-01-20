#[derive(Serialize, Deserialize)]
struct JavaScriptActionParameter {
	#[serde(rename = "Category")]
	category: String,
	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "IsRequired")]
	is_required: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: CodeActions$BasicParameterType,
}

#[derive(Serialize, Deserialize)]
struct JavaScriptAction {
	#[serde(rename = "ActionDefaultReturnName")]
	action_default_return_name: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "JavaReturnType")]
	java_return_type: CodeActions$VoidType,
	#[serde(rename = "MicroflowActionInfo")]
	microflow_action_info: Null,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters")]
	parameters: Vec<2, ["JavaScriptActions$JavaScriptActionParameter"]>,
	#[serde(rename = "Platform")]
	platform: String,
	#[serde(rename = "TypeParameters")]
	type_parameters: Vec<2, []>,
}

