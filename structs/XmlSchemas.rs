#[derive(Serialize, Deserialize)]
struct XmlSchemaContents {
	#[serde(rename = "Contents")]
	contents: String,
	#[serde(rename = "LocalizedContentsFormat")]
	localized_contents_format: String,
	#[serde(rename = "LocalizedLocationFormat")]
	localized_location_format: String,
	#[serde(rename = "Location")]
	location: String,
	#[serde(rename = "TargetNamespace")]
	target_namespace: String,
}

#[derive(Serialize, Deserialize)]
struct XmlSchema {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "FilePath")]
	file_path: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "SchemaContentss")]
	schema_contentss: Vec<2, ["XmlSchemas$XmlSchemaContents"]>,
}

