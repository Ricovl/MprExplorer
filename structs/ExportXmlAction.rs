#[derive(Serialize, Deserialize)]
struct FileDocumentExport {
	#[serde(rename = "TargetDocumentVariableName")]
	target_document_variable_name: String,
}

#[derive(Serialize, Deserialize)]
struct StringExport {
	#[serde(rename = "OutputVariableName")]
	output_variable_name: String,
}

