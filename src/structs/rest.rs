use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct CorsConfiguration {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowAuthentication")]
	allow_authentication: bool,
	#[serde(rename = "AllowedOrigins")]
	allowed_origins: String,
	#[serde(rename = "MaxAge")]
	max_age: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublishedRestService {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AllowedRoles", deserialize_with = "deserialize_settings")]
	allowed_roles: Vec<String>,
	#[serde(rename = "AuthenticationMicroflow")]
	authentication_microflow: String,
	#[serde(rename = "AuthenticationTypes", deserialize_with = "deserialize_settings")]
	authentication_types: Vec<String>,
	#[serde(rename = "CorsConfiguration")]
	cors_configuration: Option<rest::CorsConfiguration>,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Excluded")]
	excluded: bool,
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<UnknownType>,
	#[serde(rename = "Path")]
	path: String,
	#[serde(rename = "Resources", deserialize_with = "deserialize_settings")]
	resources: Vec<rest::PublishedRestServiceResource>,
	#[serde(rename = "ServiceName")]
	service_name: String,
	#[serde(rename = "Version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublishedRestServiceOperation {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Commit")]
	commit: String,
	#[serde(rename = "Deprecated")]
	deprecated: bool,
	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "ExportMapping")]
	export_mapping: String,
	#[serde(rename = "HttpMethod")]
	http_method: String,
	#[serde(rename = "ImportMapping")]
	import_mapping: String,
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "ObjectHandlingBackup")]
	object_handling_backup: String,
	#[serde(rename = "Parameters", deserialize_with = "deserialize_settings")]
	parameters: Vec<rest::RestOperationParameter>,
	#[serde(rename = "Path")]
	path: String,
	#[serde(rename = "Summary")]
	summary: String,
}

#[derive(Serialize, Deserialize)]
pub struct PublishedRestServiceResource {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Documentation")]
	documentation: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "Operations", deserialize_with = "deserialize_settings")]
	operations: Vec<rest::PublishedRestServiceOperation>,
}

#[derive(Serialize, Deserialize)]
pub struct RestOperationParameter {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Description")]
	description: String,
	#[serde(rename = "MicroflowParameter")]
	microflow_parameter: String,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "ParameterType")]
	parameter_type: String,
	#[serde(rename = "Type")]
	var_type: Option<data_types::DataType>,
}

