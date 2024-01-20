#[derive(Serialize, Deserialize)]
struct Image {
	#[serde(rename = "Image")]
	image: Binary,
	#[serde(rename = "ImageFormat")]
	image_format: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
struct ImageCollection {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: Bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Images")]
	images: Vec<3, ["Images$Image"]>,
	#[serde(rename = "Name")]
	name: String,
}

