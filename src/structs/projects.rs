use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Folder {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ModuleImpl {
	#[serde(rename = "$ID")]
	_id: Uuid,

	app_store_guid: String,
	app_store_package_id: i64,
	app_store_version: String,
	app_store_version_guid: String,
	from_app_store: bool,
	is_theme_module: bool,
	pub name: String,
	new_sort_index: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleSettings {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ProtectedModuleType")]
	protected_module_type: String,
	#[serde(rename = "Version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
pub struct OneTimeConversion {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "IsSystemProject")]
	is_system_project: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConversion {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "OneTimeConversions", deserialize_with = "deserialize_settings")]
	one_time_conversions: Vec<projects::OneTimeConversion>,
}

