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
pub struct ModuleImpl {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AppStoreGuid")]
	app_store_guid: String,
	#[serde(rename = "AppStorePackageId")]
	app_store_package_id: i64,
	#[serde(rename = "AppStoreVersion")]
	app_store_version: String,
	#[serde(rename = "AppStoreVersionGuid")]
	app_store_version_guid: String,
	#[serde(rename = "FromAppStore")]
	from_app_store: bool,
	#[serde(rename = "IsThemeModule")]
	is_theme_module: bool,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "NewSortIndex")]
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

