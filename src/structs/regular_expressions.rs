use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct RegularExpression {
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Expression")]
	expression: String,
	#[serde(rename = "Name")]
	name: String,
}

