#[derive(Serialize, Deserialize)]
struct Constant {
	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ExposedToClient")]
	exposed_to_client: Bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Type")]
	type: DataTypes$StringType,
}

