use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Constant {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "DefaultValue")]
	default_value: String,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ExposedToClient")]
	exposed_to_client: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Type")]
	var_type: Option<data_types::DataType>,
}

