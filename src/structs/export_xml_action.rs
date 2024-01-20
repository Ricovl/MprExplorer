use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct FileDocumentExport {
	#[serde(rename = "target_document_variable_name")]
	target_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct StringExport {
	#[serde(rename = "output_variable_name")]
	output_variable_name: String,
}

