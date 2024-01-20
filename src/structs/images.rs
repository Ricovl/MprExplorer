use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Image {
	#[serde(rename = "image")]
	image: Binary,
	#[serde(rename = "image_format")]
	image_format: String,
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ImageCollection {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "images")]
	images: Vec<>,
	#[serde(rename = "name")]
	name: String,
}

