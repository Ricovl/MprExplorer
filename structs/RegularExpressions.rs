#[derive(Serialize, Deserialize)]
struct RegularExpression {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "Name")]
	name: String,
}

