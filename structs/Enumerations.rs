#[derive(Serialize, Deserialize)]
struct Condition {
	#[serde(rename = "AttributeValue")]
	attribute_value: String,
	#[serde(rename = "EditableVisible")]
	editable_visible: Bool,
}

#[derive(Serialize, Deserialize)]
struct EnumerationValue {
	#[serde(rename = "Caption")]
	caption: Texts$Text,
	#[serde(rename = "Image")]
	image: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct Enumeration {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Values")]
	values: Vec<3, ["Enumerations$EnumerationValue"]>,
}

