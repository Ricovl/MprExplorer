use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct XmlSchema {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "FilePath")]
	file_path: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "SchemaContentss", deserialize_with = "deserialize_settings")]
	schema_contentss: Vec<xml_schemas::XmlSchemaContents>,
}

#[derive(Serialize, Deserialize)]
pub struct XmlSchemaContents {
	#[serde(rename = "$ID")]
	_id: Uuid,

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

