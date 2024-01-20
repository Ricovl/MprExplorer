#[derive(Serialize, Deserialize)]
struct CustomIconCollection {
	#[serde(rename = "CollectionClass")]
	collection_class: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "FontData")]
	font_data: Binary,
	#[serde(rename = "Icons")]
	icons: Vec<3, ["CustomIcons$CustomIcon"]>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Prefix")]
	prefix: String,
}

#[derive(Serialize, Deserialize)]
struct CustomIcon {
	#[serde(rename = "CharacterCode")]
	character_code: i64,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Tags")]
	tags: Vec<1, ["String"]>,
}

