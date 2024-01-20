use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Project {
	#[serde(rename = "IsSystemProject")]
	is_system_project: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleSettings {
	#[serde(rename = "ExportLevel")]
	export_level: String,
	#[serde(rename = "ProtectedModuleType")]
	protected_module_type: String,
	#[serde(rename = "Version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConversion {
	#[serde(rename = "OneTimeConversions")]
	one_time_conversions: Vec<Projects$OneTimeConversion>,
}

#[derive(Serialize, Deserialize)]
pub struct Folder {
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct OneTimeConversion {
	#[serde(rename = "Name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleImpl {
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

