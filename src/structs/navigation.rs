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

	#[serde(rename = "Profiles")]
	profiles: Vec<navigation::NavigationProfile, >,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationProfile {
	#[serde(rename = "$ID")]
	_id: Uuid,

	#[serde(rename = "AppIcon")]
	app_icon: String,
	#[serde(rename = "AppTitle")]
	app_title: texts::Text,
	#[serde(rename = "HomeItems")]
	home_items: Vec<navigation::RoleBasedHomePage, >,
	#[serde(rename = "HomePage")]
	home_page: navigation::HomePage,
	#[serde(rename = "Kind")]
	kind: String,
	#[serde(rename = "LoginPageSettings")]
	login_page_settings: forms::FormSettings,
	#[serde(rename = "Menu")]
	menu: menus::MenuItemCollection,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OfflineEntityConfigs")]
	offline_entity_configs: Vec<UnknownType>,
	#[serde(rename = "ProgressiveWebAppSettings")]
	progressive_web_app_settings: Empty,
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

