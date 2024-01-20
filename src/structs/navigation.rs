use serde::{Deserialize, Serialize};
use super::*;

#[derive(Serialize, Deserialize)]
pub struct NavigationDocument {
	#[serde(rename = "Profiles")]
	profiles: Vec<Navigation$NavigationProfile>,
}

#[derive(Serialize, Deserialize)]
pub struct RoleBasedHomePage {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Page")]
	page: String,
	#[serde(rename = "UserRole")]
	user_role: String,
}

#[derive(Serialize, Deserialize)]
pub struct HomePage {
	#[serde(rename = "Microflow")]
	microflow: String,
	#[serde(rename = "Page")]
	page: String,
}

#[derive(Serialize, Deserialize)]
pub struct NavigationProfile {
	#[serde(rename = "AppIcon")]
	app_icon: String,
	#[serde(rename = "AppTitle")]
	app_title: Texts$Text,
	#[serde(rename = "HomeItems")]
	home_items: Vec<Navigation$RoleBasedHomePage>,
	#[serde(rename = "HomePage")]
	home_page: Navigation$HomePage,
	#[serde(rename = "Kind")]
	kind: String,
	#[serde(rename = "LoginPageSettings")]
	login_page_settings: Forms$FormSettings,
	#[serde(rename = "Menu")]
	menu: Menus$MenuItemCollection,
	#[serde(rename = "Name")]
	name: String,
	#[serde(rename = "OfflineEntityConfigs")]
	offline_entity_configs: Vec<3, []>,
	#[serde(rename = "ProgressiveWebAppSettings")]
	progressive_web_app_settings: Null,
}

