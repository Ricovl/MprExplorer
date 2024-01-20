use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Constant {
	#[serde(rename = "default_value")]
	default_value: String,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "exposed_to_client")]
	exposed_to_client: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "type")]
	type: data_types::StringType,
}

