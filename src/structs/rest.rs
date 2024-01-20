use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct PublishedRestServiceResource {
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "operations")]
	operations: Vec<>,
}

#[derive(Serialize, Deserialize)]
pub struct RestOperationParameter {
	#[serde(rename = "description")]
	description: String,
	#[serde(rename = "microflow_parameter")]
	microflow_parameter: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameter_type")]
	parameter_type: String,
	#[serde(rename = "type")]
	type: data_types::StringType,
}

#[derive(Serialize, Deserialize)]
pub struct PublishedRestServiceOperation {
	#[serde(rename = "commit")]
	commit: String,
	#[serde(rename = "deprecated")]
	deprecated: bool,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "export_mapping")]
	export_mapping: String,
	#[serde(rename = "http_method")]
	http_method: String,
	#[serde(rename = "import_mapping")]
	import_mapping: String,
	#[serde(rename = "microflow")]
	microflow: String,
	#[serde(rename = "object_handling_backup")]
	object_handling_backup: String,
	#[serde(rename = "parameters")]
	parameters: Vec<>,
	#[serde(rename = "path")]
	path: String,
	#[serde(rename = "summary")]
	summary: String,
}

#[derive(Serialize, Deserialize)]
pub struct CorsConfiguration {
	#[serde(rename = "allow_authentication")]
	allow_authentication: bool,
	#[serde(rename = "allowed_origins")]
	allowed_origins: String,
	#[serde(rename = "max_age")]
	max_age: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublishedRestService {
	#[serde(rename = "allowed_roles")]
	allowed_roles: Vec<>,
	#[serde(rename = "authentication_microflow")]
	authentication_microflow: String,
	#[serde(rename = "authentication_types")]
	authentication_types: Vec<>,
	#[serde(rename = "cors_configuration")]
	cors_configuration: Null,
	#[serde(rename = "documentation")]
	documentation: String,
	#[serde(rename = "excluded")]
	excluded: bool,
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "parameters")]
	parameters: Vec<>,
	#[serde(rename = "path")]
	path: String,
	#[serde(rename = "resources")]
	resources: Vec<>,
	#[serde(rename = "service_name")]
	service_name: String,
	#[serde(rename = "version")]
	version: String,
}

