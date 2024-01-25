use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct Folder {
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleImpl {
	#[serde(rename = "app_store_guid")]
	app_store_guid: String,
	#[serde(rename = "app_store_package_id")]
	app_store_package_id: i64,
	#[serde(rename = "app_store_version")]
	app_store_version: String,
	#[serde(rename = "app_store_version_guid")]
	app_store_version_guid: String,
	#[serde(rename = "from_app_store")]
	from_app_store: bool,
	#[serde(rename = "is_theme_module")]
	is_theme_module: bool,
	#[serde(rename = "name")]
	name: String,
	#[serde(rename = "new_sort_index")]
	new_sort_index: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ModuleSettings {
	#[serde(rename = "export_level")]
	export_level: String,
	#[serde(rename = "protected_module_type")]
	protected_module_type: String,
	#[serde(rename = "version")]
	version: String,
}

#[derive(Serialize, Deserialize)]
pub struct OneTimeConversion {
	#[serde(rename = "name")]
	name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Project {
	#[serde(rename = "is_system_project")]
	is_system_project: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectConversion {
	#[serde(rename = "one_time_conversions")]
	one_time_conversions: Vec<projects::OneTimeConversion, >,
}

