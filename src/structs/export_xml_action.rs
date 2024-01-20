use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct FileDocumentExport {
	#[serde(rename = "TargetDocumentVariableName")]
	target_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringExport {
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
}

