use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct XmlSchema {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "file_path")]
	file_path: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "schema_contentss")]
	schema_contentss: Vec<xml_schemas::XmlSchemaContents, >,
}

#[derive(Serialize, Deserialize)]
pub struct XmlSchemaContents {
	#[serde(rename = "contents")]
	contents: String,
	#[serde(rename = "localized_contents_format")]
	localized_contents_format: String,
	#[serde(rename = "localized_location_format")]
	localized_location_format: String,
	#[serde(rename = "location")]
	location: String,
	#[serde(rename = "target_namespace")]
	target_namespace: String,
}

