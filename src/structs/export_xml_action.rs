use serde::{Deserialize, Serialize};
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
#[serde(tag = "$Type")]
pub enum XmlExportAction {
	#[serde(rename = "ExportXmlAction$StringExport")]
	StringExport(StringExport),
	#[serde(rename = "ExportXmlAction$FileDocumentExport")]
	FileDocumentExport(FileDocumentExport),
}

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

