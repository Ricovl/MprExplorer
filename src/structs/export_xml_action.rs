use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct FileDocumentExport {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "TargetDocumentVariableName")]
	target_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringExport {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
}

