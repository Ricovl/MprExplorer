use serde::{Deserialize, Serialize};
use uuid::Uuid;
use super::*;

#[derive(Serialize, Deserialize)]
pub struct HomePage {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Page")]
	page: String,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationDocument {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Profiles", deserialize_with = "deserialize_settings")]
	profiles: Vec<navigation::NavigationProfile>,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationProfile {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AppIcon")]
	app_icon: String,
	#[serde(rename = "AppTitle")]
	app_title: Option<texts::Text>,
	#[serde(rename = "HomeItems", deserialize_with = "deserialize_settings")]
	home_items: Vec<navigation::RoleBasedHomePage>,
	#[serde(rename = "HomePage")]
	home_page: Option<navigation::HomePage>,
	#[serde(rename = "Kind")]
	kind: String,
	#[serde(rename = "LoginPageSettings")]
	login_page_settings: Option<forms::FormSettings>,
	#[serde(rename = "Menu")]
	menu: Option<menus::MenuItemCollection>,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OfflineEntityConfigs", deserialize_with = "deserialize_settings")]
	offline_entity_configs: Vec<UnknownType>,
	#[serde(rename = "ProgressiveWebAppSettings")]
	progressive_web_app_settings: Option<Empty>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleBasedHomePage {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Page")]
	page: String,
	#[serde(rename = "UserRole")]
	user_role: String,
}

