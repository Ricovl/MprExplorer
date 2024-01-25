use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Image {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Image")]
	image: Binary,
	#[serde(rename = "ImageFormat")]
	image_format: String,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageCollection {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Images")]
	images: Vec<images::Image, >,
	#[serde(rename = "Name")]
	name: String,
}

