use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct ImageCollection {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Images")]
	images: Vec<Images$Image>,
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
	#[serde(rename = "Image")]
	image: Binary,
	#[serde(rename = "ImageFormat")]
	image_format: String,
	#[serde(rename = "Name")]
	name: String,
}

